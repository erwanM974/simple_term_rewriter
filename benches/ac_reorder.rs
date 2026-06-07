/*
Copyright 2024 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! Benchmark: AC reordering of a partially-commutative associative chain.
//!
//! Input: a right-skewed `Add` chain of `Var(n-1), Var(n-2), ..., Var(0)` — the
//! worst case for the insertion-sort-like algorithm in `PartialACReorderRule`,
//! which must move every element past all its predecessors.
//!
//! The rule flattens the whole chain in one call, sorts the flat list, then
//! rebuilds.  Lengths [16, 64, 256] expose the O(n²) worst-case sort cost.

use std::cmp::Ordering;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use hashconsing::HConsign;
use simple_term_rewriter::process::strategy::RewriteProcess;
use simple_term_rewriter::process::untraced::RewriteProcessUntracedExecutor;
use simple_term_rewriter::rules::primitives::reorder_apc::{
    ModuloAssociativePartialReorderer, PartialACReorderRule,
};
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

// == language ==================================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum ArithOp {
    Add,
    Var(u32),
}

impl RewritableLanguageOperatorSymbol for ArithOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            ArithOp::Add => LanguageOperatorArity::Fixed(2),
            ArithOp::Var(_) => LanguageOperatorArity::Fixed(0),
        }
    }
}

// == reorderer =================================================================
//
// All Var nodes commute under Add.
// Canonical order: ascending by index (Var(0) < Var(1) < …).

struct VarReorderer;

impl ModuloAssociativePartialReorderer<ArithOp> for VarReorderer {
    fn is_an_associative_partially_commutative_binary_operator_we_may_consider(
        &self,
        op: &ArithOp,
    ) -> bool {
        *op == ArithOp::Add
    }

    fn may_commute_under(
        &self,
        _parent_op: &ArithOp,
        left: &LanguageTerm<ArithOp>,
        right: &LanguageTerm<ArithOp>,
    ) -> bool {
        matches!(
            (&left.operator, &right.operator),
            (ArithOp::Var(_), ArithOp::Var(_))
        )
    }

    fn compare_operators(&self, op1: &ArithOp, op2: &ArithOp) -> Ordering {
        match (op1, op2) {
            (ArithOp::Var(i), ArithOp::Var(j)) => i.cmp(j),
            (ArithOp::Add, ArithOp::Add) => Ordering::Equal,
            (ArithOp::Var(_), ArithOp::Add) => Ordering::Less,
            (ArithOp::Add, ArithOp::Var(_)) => Ordering::Greater,
        }
    }
}

// == input construction ========================================================

/// Right-skewed chain in descending index order: Add(Var(n-1), Add(Var(n-2), …, Var(0))).
/// This is the worst case for the insertion-sort: every element must move.
fn build_reverse_chain(n: usize, f: &mut TermFactory<ArithOp>) -> LanguageTerm<ArithOp> {
    let mut result = LanguageTermNode::build(ArithOp::Var(0), vec![], f);
    for i in 1..n {
        let i_term = LanguageTermNode::build(ArithOp::Var(i as u32), vec![], f);
        result = LanguageTermNode::build(ArithOp::Add, vec![i_term, result], f);
    }
    result
}

// == benchmark =================================================================

fn bench_ac_reorder(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("ac_reorder_worst_case");
    for n in [16usize, 64, 256] {
        let term = build_reverse_chain(n, f);
        let process =
            RewriteProcess::Rule(Box::new(PartialACReorderRule::new("reorder", VarReorderer)));
        group.bench_with_input(BenchmarkId::new("chain_length", n), &term, |b, t| {
            b.iter(|| RewriteProcessUntracedExecutor::rewrite(&process, black_box(t), f))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ac_reorder);
criterion_main!(benches);
