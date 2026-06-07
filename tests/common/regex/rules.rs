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

use simple_term_rewriter::process::strategy::{DepthOrder, RewriteProcess, SiblingOrder};
use simple_term_rewriter::process::untraced::RewriteProcessUntracedExecutor;
use simple_term_rewriter::rule::{ClosureRewriteRule, RewriteRule};
use simple_term_rewriter::term::syntax::{LanguageTerm, LanguageTermNode, TermFactory};

use super::lang::RegexOp;

// == test utilities ============================================================

pub fn rewrite(
    process: RewriteProcess<RegexOp>,
    term: LanguageTerm<RegexOp>,
    factory: &mut TermFactory<RegexOp>,
) -> Vec<LanguageTerm<RegexOp>> {
    RewriteProcessUntracedExecutor::rewrite(&process, &term, factory)
}

pub fn rule_as_process(r: impl RewriteRule<RegexOp> + 'static) -> RewriteProcess<RegexOp> {
    RewriteProcess::Rule(Box::new(r))
}

/// `Star(Empty) → Epsilon`
pub fn rule_star_empty() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("star(∅)→ε", |t, _, _, f| {
        if t.operator != RegexOp::Star {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Empty {
            Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
        } else {
            None
        }
    })
}

/// `Star(Epsilon) → Epsilon`
pub fn rule_star_epsilon() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("star(ε)→ε", |t, _, _, f| {
        if t.operator != RegexOp::Star {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Epsilon {
            Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
        } else {
            None
        }
    })
}

/// `Star(Star(r)) → Star(r)`
pub fn rule_double_star() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("star(star(r))→star(r)", |t, _, _, _f| {
        if t.operator != RegexOp::Star {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Star {
            Some(t.sub_terms[0].clone())
        } else {
            None
        }
    })
}

/// `Concat(Empty, r) → Empty`
pub fn rule_concat_left_empty() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("concat(∅,r)→∅", |t, _, _, f| {
        if t.operator != RegexOp::Concat {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Empty {
            Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
        } else {
            None
        }
    })
}

/// `Concat(r, Empty) → Empty`
pub fn rule_concat_right_empty() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("concat(r,∅)→∅", |t, _, _, f| {
        if t.operator != RegexOp::Concat {
            return None;
        }
        if t.sub_terms[1].operator == RegexOp::Empty {
            Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
        } else {
            None
        }
    })
}

/// `Concat(Epsilon, r) → r`
pub fn rule_concat_left_epsilon() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("concat(ε,r)→r", |t, _, _, _f| {
        if t.operator != RegexOp::Concat {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Epsilon {
            Some(t.sub_terms[1].clone())
        } else {
            None
        }
    })
}

/// `Concat(r, Epsilon) → r`
pub fn rule_concat_right_epsilon() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("concat(r,ε)→r", |t, _, _, _f| {
        if t.operator != RegexOp::Concat {
            return None;
        }
        if t.sub_terms[1].operator == RegexOp::Epsilon {
            Some(t.sub_terms[0].clone())
        } else {
            None
        }
    })
}

/// `Alt(Empty, r) → r`
pub fn rule_alt_left_empty() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("alt(∅,r)→r", |t, _, _, _f| {
        if t.operator != RegexOp::Alt {
            return None;
        }
        if t.sub_terms[0].operator == RegexOp::Empty {
            Some(t.sub_terms[1].clone())
        } else {
            None
        }
    })
}

/// `Alt(r, Empty) → r`
pub fn rule_alt_right_empty() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("alt(r,∅)→r", |t, _, _, _f| {
        if t.operator != RegexOp::Alt {
            return None;
        }
        if t.sub_terms[1].operator == RegexOp::Empty {
            Some(t.sub_terms[0].clone())
        } else {
            None
        }
    })
}

/// `Alt(r, r) → r`  (idempotence of alternation)
pub fn rule_alt_idempotent() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("alt(r,r)→r", |t, _, _, _f| {
        if t.operator != RegexOp::Alt {
            return None;
        }
        if t.sub_terms[0] == t.sub_terms[1] {
            Some(t.sub_terms[0].clone())
        } else {
            None
        }
    })
}

// == strategy utilities ========================================================

/// All ten built-in regex normalisation rules as boxed trait objects.
///
/// The rules cover: `Star(∅)→ε`, `Star(ε)→ε`, `Star(Star(r))→Star(r)`,
/// `Concat(∅,r)→∅`, `Concat(r,∅)→∅`, `Concat(ε,r)→r`, `Concat(r,ε)→r`,
/// `Alt(∅,r)→r`, `Alt(r,∅)→r`, `Alt(r,r)→r`.
pub fn all_rules() -> Vec<Box<dyn RewriteRule<RegexOp>>> {
    vec![
        Box::new(rule_star_empty()),
        Box::new(rule_star_epsilon()),
        Box::new(rule_double_star()),
        Box::new(rule_concat_left_empty()),
        Box::new(rule_concat_right_empty()),
        Box::new(rule_concat_left_epsilon()),
        Box::new(rule_concat_right_epsilon()),
        Box::new(rule_alt_left_empty()),
        Box::new(rule_alt_right_empty()),
        Box::new(rule_alt_idempotent()),
    ]
}

/// One outermost normalisation step over all regex rules.
///
/// Tries each rule at the root first; if nothing fires there, descends into
/// the leftmost child via outermost traversal.
pub fn one_step_outermost() -> RewriteProcess<RegexOp> {
    let try_root =
        RewriteProcess::TryOnePath(all_rules().into_iter().map(RewriteProcess::Rule).collect());
    let try_child = RewriteProcess::AnyChild(
        SiblingOrder::Leftmost,
        DepthOrder::Outermost,
        Box::new(RewriteProcess::TryOnePath(
            all_rules().into_iter().map(RewriteProcess::Rule).collect(),
        )),
    );
    RewriteProcess::TryOnePath(vec![try_root, try_child])
}

/// Full outermost normalisation strategy: repeats `one_step_outermost` until
/// the term reaches a fixpoint.
pub fn normalization_strategy() -> RewriteProcess<RegexOp> {
    RewriteProcess::Repeat(Box::new(one_step_outermost()))
}
