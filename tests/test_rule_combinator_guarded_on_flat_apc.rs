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

//! Unit tests for [`GuardedRule`] wrapping a [`FlattenedACTransfoRule`].
//!
//! Each test calls `try_apply` directly on the Alt node, with an explicit
//! context term (the surrounding Star, when applicable) and position.
//! No traversal machinery is involved.
//!
//! Rule under test: star absorption — when an `Alt` sits directly under a
//! `Star`, each `Star(x)` element inside the alt is stripped to `x`, because
//! `(x*)* = x*`.  The rule fires only when the parent operator is `Star`.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
#[allow(unused_imports)]
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::combinators::guard::OnlyUnderOpRewriteApplicationGuard;
use simple_term_rewriter::rules::combinators::guarded::GuardedRule;
use simple_term_rewriter::rules::primitives::flat_apc::{
    FlattenedACTransfoRule, ModuloAssociativeGenericFlattenedChecker,
};
use simple_term_rewriter::term::syntax::{LanguageTerm, LanguageTermNode, TermFactory};

use common::regex::constructors::*;
use common::regex::lang::RegexOp;

// == local helper ==============================================================

/// Build a right-associated `Alt` chain from a slice.
fn alt_chain(
    elements: &[LanguageTerm<RegexOp>],
    f: &mut TermFactory<RegexOp>,
) -> LanguageTerm<RegexOp> {
    assert!(!elements.is_empty());
    elements
        .iter()
        .rev()
        .cloned()
        .reduce(|right, left| LanguageTermNode::build(RegexOp::Alt, vec![left, right], f))
        .unwrap()
}

// == checker ===================================================================

struct StarAbsorption;

impl ModuloAssociativeGenericFlattenedChecker<RegexOp> for StarAbsorption {
    fn is_an_associative_binary_operator_we_may_consider(&self, op: &RegexOp) -> bool {
        *op == RegexOp::Alt
    }
    fn transform_flattened_sub_terms(
        &self,
        _: &RegexOp,
        elements: Vec<&LanguageTerm<RegexOp>>,
        _f: &mut TermFactory<RegexOp>,
    ) -> Option<Vec<LanguageTerm<RegexOp>>> {
        let mut changed = false;
        let new_elements: Vec<_> = elements
            .into_iter()
            .map(|e| {
                if e.operator == RegexOp::Star {
                    changed = true;
                    e.sub_terms[0].clone()
                } else {
                    e.clone()
                }
            })
            .collect();
        if changed {
            Some(new_elements)
        } else {
            None
        }
    }
}

// == rule and position helpers =================================================

fn make_rule() -> GuardedRule<RegexOp> {
    GuardedRule::new(
        FlattenedACTransfoRule::new("star absorption", StarAbsorption),
        OnlyUnderOpRewriteApplicationGuard::new(|op: &RegexOp| *op == RegexOp::Star),
    )
}

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

/// Position of child 0 within the root (i.e. the Alt inside a Star).
fn child0_pos() -> PositionInLanguageTerm {
    root_pos().get_position_of_nth_child(0)
}

// == guard blocks when there is no Star parent =================================

#[test]
fn no_fire_at_root_without_star_parent() {
    // The Alt is the context root — no parent → guard blocks.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let u = alt_chain(&[star(atom(0, &mut f), &mut f), atom(1, &mut f)], &mut f);
    assert!(make_rule().try_apply(&u, &u, &root_pos(), &mut f).is_none());
}

#[test]
fn no_fire_on_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(0, &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == guard passes when Alt is directly under Star =============================

#[test]
fn fires_when_one_element_is_starred() {
    // Star(Alt(Star(a), b)) — Alt is at child_0 of Star.
    // try_apply on Alt returns Alt(a, b).
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let star_term = star(
        alt_chain(&[star(atom(0, &mut f), &mut f), atom(1, &mut f)], &mut f),
        &mut f,
    );
    let alt_node = &star_term.sub_terms[0];
    assert_eq!(
        make_rule().try_apply(alt_node, &star_term, &child0_pos(), &mut f),
        Some(alt_chain(&[atom(0, &mut f), atom(1, &mut f)], &mut f))
    );
}

#[test]
fn fires_when_all_elements_are_starred() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let star_term = star(
        alt_chain(
            &[
                star(atom(0, &mut f), &mut f),
                star(atom(1, &mut f), &mut f),
                star(atom(2, &mut f), &mut f),
            ],
            &mut f,
        ),
        &mut f,
    );
    let alt_node = &star_term.sub_terms[0];
    assert_eq!(
        make_rule().try_apply(alt_node, &star_term, &child0_pos(), &mut f),
        Some(alt_chain(
            &[atom(0, &mut f), atom(1, &mut f), atom(2, &mut f)],
            &mut f
        ))
    );
}

#[test]
fn no_fire_when_no_element_is_starred() {
    // Star(Alt(a, b)) — no inner Stars to strip → rule returns None.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let star_term = star(
        alt_chain(&[atom(0, &mut f), atom(1, &mut f)], &mut f),
        &mut f,
    );
    let alt_node = &star_term.sub_terms[0];
    assert!(make_rule()
        .try_apply(alt_node, &star_term, &child0_pos(), &mut f)
        .is_none());
}

// == multi-step: doubly-nested stars ==========================================

#[test]
fn doubly_nested_star_reduces_in_two_steps() {
    // Star(Alt(Star(Star(a)), b))
    //
    // Step 1: Alt node has Star(Star(a)) — it IS a Star, stripped to Star(a).
    //   Result Alt: Alt(Star(a), b)
    //
    // Step 2: Alt node now has Star(a) — still a Star, stripped to a.
    //   Result Alt: Alt(a, b)
    //
    // Step 3: no starred elements → None.

    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t0 = star(
        alt(
            star(star(atom(0, &mut f), &mut f), &mut f),
            atom(1, &mut f),
            &mut f,
        ),
        &mut f,
    );
    let alt0 = &t0.sub_terms[0]; // Alt(Star(Star(a)), b)

    let a1 = make_rule()
        .try_apply(alt0, &t0, &child0_pos(), &mut f)
        .expect("step 1 should fire");
    assert_eq!(
        a1,
        alt(star(atom(0, &mut f), &mut f), atom(1, &mut f), &mut f)
    );

    let t1 = star(a1.clone(), &mut f);
    let a2 = make_rule()
        .try_apply(&a1, &t1, &child0_pos(), &mut f)
        .expect("step 2 should fire");
    assert_eq!(a2, alt(atom(0, &mut f), atom(1, &mut f), &mut f));

    let t2 = star(a2.clone(), &mut f);
    assert!(
        make_rule()
            .try_apply(&a2, &t2, &child0_pos(), &mut f)
            .is_none(),
        "should be irreducible"
    );
}

// == get_desc =================================================================

#[test]
fn guarded_rule_get_desc_delegates_to_inner() {
    // GuardedRule::get_desc must return the inner rule's description.
    assert_eq!(make_rule().get_desc(), "star absorption");
}
