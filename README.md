# simple_term_rewriter

[![CI](https://github.com/erwanM974/simple_term_rewriter/actions/workflows/ci.yml/badge.svg)](https://github.com/erwanM974/simple_term_rewriter/actions/workflows/ci.yml)

A library for rewriting concrete ground terms according to user-defined rules,
with a composable strategy language and built-in support for associative,
commutative, and partially commutative operators.

---

## Scope

This is not a general-purpose TRS engine.  What it does and does not do:

- **Ground terms only.** There are no variables or pattern matching.  Rules
  inspect concrete terms and decide whether to fire.
- **No termination or confluence guarantees.** The engine applies rules blindly
  according to the strategy you provide.  If your rule set is non-terminating or
  non-confluent, the engine will reflect that.
- **User-supplied rule logic.** Rules are Rust closures or trait implementations.
  The library provides a catalogue of common built-in rules for associative /
  commutative operators, but the rule logic itself always lives in user code.

---

## Terms and the factory

### Defining an operator set

Implement `RewritableLanguageOperatorSymbol` on your operator enum:

```rust
use simple_term_rewriter::term::syntax::{LanguageOperatorArity, RewritableLanguageOperatorSymbol};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RegexOp { Alt, Concat, Star, Atom(u8), Epsilon, Empty }

impl RewritableLanguageOperatorSymbol for RegexOp {
    fn arity(&self) -> LanguageOperatorArity {
        use LanguageOperatorArity::Fixed;
        match self {
            RegexOp::Alt | RegexOp::Concat => Fixed(2),
            RegexOp::Star                  => Fixed(1),
            _                              => Fixed(0),
        }
    }
}
```

### Building terms

Terms are hash-consed: every `LanguageTerm<LOS>` is a reference-counted handle
into a `TermFactory<LOS>`.  Structurally identical terms built from the same
factory share the same allocation, and equality is O(1).

```rust
use hashconsing::HConsign;
use simple_term_rewriter::term::syntax::{LanguageTermNode, TermFactory};
use simple_term_rewriter::term; // term! macro

let mut f: TermFactory<RegexOp> = HConsign::empty();

// Using LanguageTermNode::build directly
let a  = LanguageTermNode::build(RegexOp::Atom(b'a'), vec![], &mut f);
let sa = LanguageTermNode::build(RegexOp::Star, vec![a.clone()], &mut f);

// Using the term! macro
let t = term!(&mut f, RegexOp::Alt;
    term!(&mut f, RegexOp::Star; term!(&mut f, RegexOp::Atom(b'a'))),
    term!(&mut f, RegexOp::Atom(b'b'))
);
```

**Factory discipline:** always use a single factory for a given rewriting session.
All terms that may be compared, and all terms produced by rewriting, must come
from the same factory.  Equality is based on an internal uid. Handles from
different factories are never equal even if structurally identical.

---

## Writing rules

All rules implement the `RewriteRule<LOS>` trait:

```rust
pub trait RewriteRule<LOS: RewritableLanguageOperatorSymbol> {
    fn get_desc(&self) -> String;
    fn try_apply(
        &self,
        term    : &LanguageTerm<LOS>,
        ctx     : &LanguageTerm<LOS>,
        pos     : &PositionInLanguageTerm,
        factory : &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>>;
}
```

`ctx` and `pos` give the surrounding context (the full term being rewritten and
the position of `term` within it).  Rules that do not need context may ignore them.

### RootRule : closure over the root node

The lightest option: a guard predicate plus a rewrite closure.

```rust
use simple_term_rewriter::rules::primitives::root::RootRule;

// Star(Star(x)) → Star(x)
let double_star = RootRule::unary(
    "double star",
    |op: &RegexOp| *op == RegexOp::Star,
    |_op, child, f| {
        if child.operator == RegexOp::Star {
            Some(child.sub_terms[0].clone())
        } else {
            None
        }
    },
);
```

`RootRule::unary` and `RootRule::binary` are convenience constructors that
unpack the children before calling the rewrite closure.  Use `RootRule::new`
for operators with variable arity.

### ClosureRewriteRule : full context available

When you need `ctx` or `pos`:

```rust
use simple_term_rewriter::rule::ClosureRewriteRule;

let rule = ClosureRewriteRule::new("my rule", |term, ctx, pos, f| {
    // inspect term, ctx, pos; construct result with f
    None
});
```

### Custom struct : for stateful or complex rules

Implement `RewriteRule<LOS>` directly on a struct when the rule carries data or
has non-trivial logic that does not fit a closure.

---

## Strategies

A `RewriteProcess<LOS>` describes how rules are applied.  The combinators are:

| Variant | Meaning |
|---|---|
| `Rule(r)` | Apply rule `r` at the current root.  Returns one result or nothing. |
| `AnyChild(sibling, depth, p)` | Apply `p` to the first child where it succeeds. |
| `Pipe(a, b)` | Apply `a`, then apply `b` to each result. |
| `Repeat(p)` | Apply `p` until it produces no result (fixpoint).  Never fails. |
| `TryOnePath(vec)` | Try each alternative in order; return the first success. |
| `TryAllPaths(vec)` | Try all alternatives; return the union of all results. |

`AnyChild` controls two orthogonal axes:

- **`SiblingOrder`**: `Leftmost` (index 0 first) or `Rightmost` (last first).
- **`DepthOrder`**: `Outermost` (try the node before its subtree) or `Innermost`
  (try the subtree before the node).

Standard reduction strategies expressed as strategies:

```rust
// Outermost (call-by-name): fire at the shallowest applicable position first.
fn outermost_step(r: Box<dyn RewriteRule<RegexOp>>) -> RewriteProcess<RegexOp> {
    use RewriteProcess::*;
    use SiblingOrder::Leftmost;
    use DepthOrder::Outermost;
    TryOnePath(vec![
        Rule(r.clone()),
        AnyChild(Leftmost, Outermost, Box::new(Rule(r))),
    ])
}

// Full normalization: repeat outermost_step to fixpoint.
fn normalize(r: Box<dyn RewriteRule<RegexOp>>) -> RewriteProcess<RegexOp> {
    RewriteProcess::Repeat(Box::new(outermost_step(r)))
}
```

---

## Executors

### Untraced : single call, returns all results

```rust
use simple_term_rewriter::process::untraced::RewriteProcessUntracedExecutor;

let results: Vec<LanguageTerm<RegexOp>> =
    RewriteProcessUntracedExecutor::rewrite(&strategy, &term, &mut f);
// For a deterministic (TryOnePath-only) strategy, results.len() == 1.
```

### Traced : step-by-step with rule trace

```rust
use simple_term_rewriter::process::traced::RewriteProcessTracedExecutor;

let mut executor = RewriteProcessTracedExecutor::new(strategy, term, f);

while !executor.get_current_terms().is_empty() {
    let applications = executor.progress();
    // applications records which rules fired and where (left_id / right_id
    // index into the before/after frontiers).
}

let normal_forms = executor.get_completed_terms();
// Reclaim the factory for subsequent sessions:
let f = executor.into_factory();
```

`progress()` returns a `Vec<AtomicRuleApplication>`, each carrying:
- `left_id` : index of the source term in the *pre-progress* frontier,
- `rule_chain` : ordered `(PositionInRewriteProcess, PositionInLanguageTerm)` pairs for every `Rule` leaf that fired,
- `right_id` : index of the result term in the *post-progress* frontier.

After the executor is done, call `into_factory()` to reclaim the `TermFactory`
and reuse it for the next session.  This keeps all terms in the same
hash-consing universe.

---

## Built-in rules

### Associativity flush (`rules::primitives::flush`)

| Rule | Effect |
|---|---|
| `FlushRightRule` | `op(op(x, y), z) → op(x, op(y, z))` |
| `FlushLeftRule`  | `op(x, op(y, z)) → op(op(x, y), z)` |

Implement `AssociativityChecker<LOS>` to specify which operators are binary-associative.

### AC reordering (`rules::primitives::reorder_apc`)

`PartialACReorderRule` flattens an associative operator chain, sorts its
elements into a canonical order using a Lexicographic Path Ordering, and
reconstructs.  Implement `ModuloAssociativePartialReorderer<LOS>`:

- `is_an_associative_partially_commutative_binary_operator_we_may_consider` : selects operators.
- `may_commute_under(parent, left, right)` : the independence relation: returns `true` if the two adjacent elements may be swapped.
- `compare_operators` : total order on operators, used for canonical ordering.

The `may_commute_under` predicate is precisely a Mazurkiewicz independence
relation; the reorderer produces a canonical trace representative.

### AC flattened transformation (`rules::primitives::flat_apc`)

`FlattenedACTransfoRule` flattens an associative chain, hands the flat list to a
user-defined transformation, and reassembles.  Implement
`ModuloAssociativeGenericFlattenedChecker<LOS>`:

```rust
impl ModuloAssociativeGenericFlattenedChecker<SumOp> for MergeItems {
    fn is_an_associative_binary_operator_we_may_consider(&self, op: &SumOp) -> bool {
        *op == SumOp::Sum
    }
    fn transform_flattened_sub_terms(
        &self,
        _ac_op : &SumOp,
        items  : Vec<&LanguageTerm<SumOp>>,
        f      : &mut TermFactory<SumOp>,
    ) -> Option<Vec<LanguageTerm<SumOp>>> {
        // return Some(new_list) to replace, None to leave unchanged
        todo!()
    }
}
```

### Partially commutative reordering (`rules::primitives::reorder_pc`)

`PartiallyCommutativeReorderRule` is a variant of the AC reorderer for
operators that are commutative but not necessarily associative.

### Factorization (`rules::primitives::factorization`)

Rules for factoring out or distributing sub-terms across an operator:

- `FactorizeSimpleRule` : simple factorization of a common sub-term.
- `FactorizeModuloACRule` : factorization modulo an AC operator.
- `DefactorizeRule` : the inverse: distribute a factored sub-term back.

Implement `DistributivityChecker<LOS>` to describe how your operators distribute
over each other.

---

## Rule combinators

### GuardedRule (`rules::combinators::guarded`)

Wraps any rule with a `RewriteApplicationGuard` that restricts the positions at
which the rule may fire:

```rust
use simple_term_rewriter::rules::combinators::guarded::GuardedRule;
use simple_term_rewriter::rules::combinators::guard::NotUnderSameOpRewriteApplicationGuard;

let guarded = GuardedRule::new(my_rule, NotUnderSameOpRewriteApplicationGuard);
```

Built-in guards:

| Guard | When it allows firing |
|---|---|
| `RootOnlyRewriteApplicationGuard` | Only at depth 0 (the context root). |
| `NotUnderSameOpRewriteApplicationGuard` | Only at the topmost node of an operator chain (not under a node with the same operator). Useful for rules that flatten an entire chain internally. |
| `OnlyUnderOpRewriteApplicationGuard` | Only when the immediate parent satisfies a given predicate. |

Implement `RewriteApplicationGuard<LOS>` for custom guards.

---

## Metrics

```rust
use simple_term_rewriter::metrics::builtin::{tree_size, dag_size, term_depth, operator_count_by_symbol};

let n  = tree_size(&term);          // total node count (counting duplicates)
let d  = dag_size(&term);           // distinct sub-terms (hash-consing shared count)
let dp = term_depth(&term);         // depth of the deepest leaf (root = 1)
let m  = operator_count_by_symbol(&term); // HashMap<LOS, usize>
```

For richer, multi-dimensional metrics, implement `TermSymbolMetric<LOS>` and use
`TermMetrics::extract_from_term(&term)`.  This collects occurrence counts and
maximum consecutive nesting depths for each metric symbol in a single traversal.

---

## Random term generation

`random_term_generation` provides seeded random generation of well-formed terms,
useful for property-based testing and benchmarking.

```rust
use simple_term_rewriter::random_term_generation::gen::random_term;
use simple_term_rewriter::random_term_generation::types::RandomTermGenerationParameters;
```

Configure operator probabilities via `RandomTermGenerationParameters`.  Always
pass the same `TermFactory` that will be used for rewriting; terms built in a
different factory cannot be compared against rewriting results.

---

## Dependencies

| Crate | Role |
|---|---|
| [`hashconsing`](https://crates.io/crates/hashconsing) | Hash-consed term handles and factory (`HConsign`). |
| [`rand`](https://crates.io/crates/rand) | Seeded random term generation. |
| [`map-macro`](https://crates.io/crates/map-macro) | Internal convenience macros for map literals. |




