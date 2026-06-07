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

//! Unit tests for [`RootRule`] and [`ClosureRewriteRule`].
//!
//! Each test calls `try_apply` directly at the root position.
//! No traversal machinery (`get_transformations`, `PhaseRule`) is involved.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::rule::{ClosureRewriteRule, RewriteRule};
use simple_term_rewriter::rules::primitives::root::RootRule;
use simple_term_rewriter::term::syntax::{LanguageTerm, LanguageTermNode, TermFactory};

use common::regex::constructors::*;
use common::regex::lang::RegexOp;

// == language helpers ==========================================================

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

/// A non-constant term used as a stand-in for a "variable" in rules like
/// `Star(Star(x)) → x`, where x is an arbitrary sub-term.
fn some_term(f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    concat(epsilon(f), empty(f), f)
}

// == RootRule: Star rules =====================================================

fn star_epsilon_rule() -> RootRule<RegexOp> {
    RootRule::unary(
        "Star(ε) → ε",
        |op| *op == RegexOp::Star,
        |_, child, f| {
            if child.operator == RegexOp::Epsilon {
                Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
            } else {
                None
            }
        },
    )
}

fn star_empty_rule() -> RootRule<RegexOp> {
    RootRule::unary(
        "Star(∅) → ε",
        |op| *op == RegexOp::Star,
        |_, child, f| {
            if child.operator == RegexOp::Empty {
                Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
            } else {
                None
            }
        },
    )
}

fn double_star_reduce_rule() -> RootRule<RegexOp> {
    // Star(Star(r)) → r
    RootRule::unary(
        "Star(Star(r)) → r",
        |op| *op == RegexOp::Star,
        |_, child, _f| {
            if child.operator == RegexOp::Star {
                Some(child.sub_terms[0].clone())
            } else {
                None
            }
        },
    )
}

#[test]
fn star_epsilon_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(epsilon(&mut f), &mut f);
    assert_eq!(
        star_epsilon_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(epsilon(&mut f))
    );
}

#[test]
fn star_empty_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(empty(&mut f), &mut f);
    assert_eq!(
        star_empty_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(epsilon(&mut f))
    );
}

#[test]
fn star_epsilon_no_fire_on_star_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(empty(&mut f), &mut f);
    assert!(star_epsilon_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn star_empty_no_fire_on_star_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(epsilon(&mut f), &mut f);
    assert!(star_empty_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn star_rules_do_not_fire_on_wrong_operator() {
    // Star rules require root to be Star; Concat must not trigger them.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(epsilon(&mut f), empty(&mut f), &mut f);
    assert!(star_epsilon_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
    assert!(star_empty_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
    assert!(double_star_reduce_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn double_star_reduce_fires() {
    // Star(Star(some_term())) → some_term()
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let x = some_term(&mut f);
    let t = star(star(x.clone(), &mut f), &mut f);
    assert_eq!(
        double_star_reduce_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(x)
    );
}

#[test]
fn double_star_reduce_no_fire_on_star_single() {
    // Star(Epsilon) — child is not Star; double-reduce rule returns None
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(epsilon(&mut f), &mut f);
    assert!(double_star_reduce_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

// == RootRule: Concat rules ===================================================

fn concat_rule() -> RootRule<RegexOp> {
    RootRule::binary(
        "Concat simplification",
        |op| *op == RegexOp::Concat,
        |_, left, right, f| match (&left.operator, &right.operator) {
            (RegexOp::Epsilon, _) => Some(right.clone()),
            (_, RegexOp::Epsilon) => Some(left.clone()),
            (RegexOp::Empty, _) | (_, RegexOp::Empty) => {
                Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
            }
            _ => None,
        },
    )
}

#[test]
fn concat_epsilon_left_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let x = some_term(&mut f);
    let t = concat(epsilon(&mut f), x.clone(), &mut f);
    assert_eq!(
        concat_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(x)
    );
}

#[test]
fn concat_epsilon_right_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let x = some_term(&mut f);
    let t = concat(x.clone(), epsilon(&mut f), &mut f);
    assert_eq!(
        concat_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(x)
    );
}

#[test]
fn concat_empty_left_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(empty(&mut f), some_term(&mut f), &mut f);
    assert_eq!(
        concat_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(empty(&mut f))
    );
}

#[test]
fn concat_empty_right_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(some_term(&mut f), empty(&mut f), &mut f);
    assert_eq!(
        concat_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(empty(&mut f))
    );
}

#[test]
fn concat_no_fire_on_nested_stars() {
    // Concat(Star(Epsilon), Star(Empty)) — neither child is a bare constant
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(epsilon(&mut f), &mut f),
        star(empty(&mut f), &mut f),
        &mut f,
    );
    assert!(concat_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn concat_rule_no_fire_on_alt() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(epsilon(&mut f), empty(&mut f), &mut f);
    assert!(concat_rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

// == RootRule: Alt rules ======================================================

fn alt_rule() -> RootRule<RegexOp> {
    // Empty is the identity element of Alt; Epsilon absorbs.
    RootRule::binary(
        "Alt simplification",
        |op| *op == RegexOp::Alt,
        |_, left, right, f| match (&left.operator, &right.operator) {
            (RegexOp::Empty, _) => Some(right.clone()),
            (_, RegexOp::Empty) => Some(left.clone()),
            (RegexOp::Epsilon, _) | (_, RegexOp::Epsilon) => {
                Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
            }
            _ => None,
        },
    )
}

#[test]
fn alt_empty_left_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let x = some_term(&mut f);
    let t = alt(empty(&mut f), x.clone(), &mut f);
    assert_eq!(alt_rule().try_apply(&t, &t, &root_pos(), &mut f), Some(x));
}

#[test]
fn alt_empty_right_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let x = some_term(&mut f);
    let t = alt(x.clone(), empty(&mut f), &mut f);
    assert_eq!(alt_rule().try_apply(&t, &t, &root_pos(), &mut f), Some(x));
}

#[test]
fn alt_epsilon_left_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(epsilon(&mut f), some_term(&mut f), &mut f);
    assert_eq!(
        alt_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(epsilon(&mut f))
    );
}

#[test]
fn alt_epsilon_right_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(some_term(&mut f), epsilon(&mut f), &mut f);
    assert_eq!(
        alt_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(epsilon(&mut f))
    );
}

#[test]
fn alt_no_fire_on_nested_stars() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(
        star(epsilon(&mut f), &mut f),
        star(empty(&mut f), &mut f),
        &mut f,
    );
    assert!(alt_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == ClosureRewriteRule agrees with RootRule ==================================

fn star_epsilon_closure() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("Star(ε) → ε", |t, _, _, f| {
        if t.operator == RegexOp::Star && t.sub_terms[0].operator == RegexOp::Epsilon {
            Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
        } else {
            None
        }
    })
}

fn star_empty_closure() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("Star(∅) → ε", |t, _, _, f| {
        if t.operator == RegexOp::Star && t.sub_terms[0].operator == RegexOp::Empty {
            Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
        } else {
            None
        }
    })
}

fn double_star_reduce_closure() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("Star(Star(r)) → r", |t, _, _, _f| {
        if t.operator == RegexOp::Star && t.sub_terms[0].operator == RegexOp::Star {
            Some(t.sub_terms[0].sub_terms[0].clone())
        } else {
            None
        }
    })
}

fn concat_closure() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("Concat simplification", |t, _, _, f| {
        if t.operator != RegexOp::Concat {
            return None;
        }
        let (l, r) = (&t.sub_terms[0], &t.sub_terms[1]);
        match (&l.operator, &r.operator) {
            (RegexOp::Epsilon, _) => Some(r.clone()),
            (_, RegexOp::Epsilon) => Some(l.clone()),
            (RegexOp::Empty, _) | (_, RegexOp::Empty) => {
                Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
            }
            _ => None,
        }
    })
}

fn alt_closure() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("Alt simplification", |t, _, _, f| {
        if t.operator != RegexOp::Alt {
            return None;
        }
        let (l, r) = (&t.sub_terms[0], &t.sub_terms[1]);
        match (&l.operator, &r.operator) {
            (RegexOp::Empty, _) => Some(r.clone()),
            (_, RegexOp::Empty) => Some(l.clone()),
            (RegexOp::Epsilon, _) | (_, RegexOp::Epsilon) => {
                Some(LanguageTermNode::build(RegexOp::Epsilon, vec![], f))
            }
            _ => None,
        }
    })
}

#[test]
fn root_rule_and_closure_agree_on_star_cases() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let pos = root_pos();
    let cases = vec![
        star(epsilon(&mut f), &mut f),
        star(empty(&mut f), &mut f),
        star(star(epsilon(&mut f), &mut f), &mut f),
        star(star(some_term(&mut f), &mut f), &mut f),
    ];
    for t in &cases {
        assert_eq!(
            star_epsilon_rule().try_apply(t, t, &pos, &mut f),
            star_epsilon_closure().try_apply(t, t, &pos, &mut f),
            "star_epsilon disagrees on {t:?}"
        );
        assert_eq!(
            star_empty_rule().try_apply(t, t, &pos, &mut f),
            star_empty_closure().try_apply(t, t, &pos, &mut f),
            "star_empty disagrees on {t:?}"
        );
        assert_eq!(
            double_star_reduce_rule().try_apply(t, t, &pos, &mut f),
            double_star_reduce_closure().try_apply(t, t, &pos, &mut f),
            "double_star_reduce disagrees on {t:?}"
        );
    }
}

#[test]
fn root_rule_and_closure_agree_on_concat_cases() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let pos = root_pos();
    let cases = vec![
        concat(epsilon(&mut f), empty(&mut f), &mut f),
        concat(empty(&mut f), epsilon(&mut f), &mut f),
        concat(epsilon(&mut f), epsilon(&mut f), &mut f),
        concat(empty(&mut f), empty(&mut f), &mut f),
        concat(star(epsilon(&mut f), &mut f), epsilon(&mut f), &mut f),
    ];
    for t in &cases {
        assert_eq!(
            concat_rule().try_apply(t, t, &pos, &mut f),
            concat_closure().try_apply(t, t, &pos, &mut f),
            "Concat disagrees on {t:?}"
        );
    }
}

#[test]
fn root_rule_and_closure_agree_on_alt_cases() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let pos = root_pos();
    let cases = vec![
        alt(empty(&mut f), empty(&mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        alt(empty(&mut f), epsilon(&mut f), &mut f),
        alt(epsilon(&mut f), epsilon(&mut f), &mut f),
        alt(star(empty(&mut f), &mut f), empty(&mut f), &mut f),
    ];
    for t in &cases {
        assert_eq!(
            alt_rule().try_apply(t, t, &pos, &mut f),
            alt_closure().try_apply(t, t, &pos, &mut f),
            "Alt disagrees on {t:?}"
        );
    }
}

// == get_desc ==================================================================

#[test]
fn closure_rule_get_desc_returns_name() {
    assert_eq!(star_epsilon_closure().get_desc(), "Star(ε) → ε");
    assert_eq!(star_empty_closure().get_desc(), "Star(∅) → ε");
    assert_eq!(double_star_reduce_closure().get_desc(), "Star(Star(r)) → r");
}

#[test]
fn root_rule_get_desc_returns_name() {
    assert_eq!(star_epsilon_rule().get_desc(), "Star(ε) → ε");
    assert_eq!(star_empty_rule().get_desc(), "Star(∅) → ε");
    assert_eq!(double_star_reduce_rule().get_desc(), "Star(Star(r)) → r");
    assert_eq!(concat_rule().get_desc(), "Concat simplification");
    assert_eq!(alt_rule().get_desc(), "Alt simplification");
}

// == ClosureRewriteRule: context_term and position are forwarded ===============

#[test]
fn closure_rule_receives_context_term() {
    // A rule that fires only when the context root is Concat.
    // This verifies that context_term is passed through unchanged.
    let rule = ClosureRewriteRule::new("fires-when-context-is-concat", |t, ctx, _pos, f| {
        if t.operator != RegexOp::Epsilon {
            return None;
        }
        if ctx.operator == RegexOp::Concat {
            Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
        } else {
            None
        }
    });
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = epsilon(&mut f);

    assert_eq!(
        rule.try_apply(
            &t,
            &concat(epsilon(&mut f), epsilon(&mut f), &mut f),
            &root_pos(),
            &mut f
        ),
        Some(empty(&mut f)),
        "should fire when context root is Concat"
    );
    assert!(
        rule.try_apply(&t, &star(epsilon(&mut f), &mut f), &root_pos(), &mut f)
            .is_none(),
        "should not fire when context root is Star"
    );
    assert!(
        rule.try_apply(&t, &t, &root_pos(), &mut f).is_none(),
        "should not fire when context root is Epsilon"
    );
}

#[test]
fn closure_rule_receives_position() {
    // A rule that fires only when the position depth is > 0 (i.e., not the root).
    // This verifies that the position argument is passed through correctly.
    let rule = ClosureRewriteRule::new("fires-below-root", |t, _ctx, pos, f| {
        if t.operator != RegexOp::Epsilon {
            return None;
        }
        if pos.get_depth() > 0 {
            Some(LanguageTermNode::build(RegexOp::Empty, vec![], f))
        } else {
            None
        }
    });
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = epsilon(&mut f);
    let child_pos = root_pos().get_position_of_nth_child(0);

    assert!(
        rule.try_apply(&t, &t, &root_pos(), &mut f).is_none(),
        "should not fire at root (depth 0)"
    );
    assert_eq!(
        rule.try_apply(&t, &t, &child_pos, &mut f),
        Some(empty(&mut f)),
        "should fire at depth 1"
    );
}
