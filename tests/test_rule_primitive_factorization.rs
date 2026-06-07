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

//! Unit tests for the four factorization rule families:
//!
//! - [`FactorizeLeftRule`]  / [`FactorizeRightRule`]  — syntactic, no AC
//! - [`DefactorizeLeftRule`] / [`DefactorizeRightRule`] — syntactic inverse
//! - [`FactorizeLeftModACRule`] / [`FactorizeRightModACRule`] — modulo AC
//!
//! Each test calls `try_apply` directly.  Most tests use the root position;
//! the context-guard tests use a child position to verify the "skip when
//! parent == op2" guard in the mod-AC rules.
//!
//! Domain: `ArithOp` — `Add` is associative + commutative; `Mul` distributes
//! over `Add` from both sides.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::primitives::factorization::{
    defactorize::{DefactorizeLeftRule, DefactorizeRightRule},
    factorize_modulo_ac::{FactorizeLeftModACRule, FactorizeRightModACRule},
    factorize_simple::{FactorizeLeftRule, FactorizeRightRule},
};
use simple_term_rewriter::term::syntax::TermFactory;

use common::arith::constructors::*;
use common::arith::lang::ArithOp;
use common::arith::rules::ArithChecker;

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

// == FactorizeLeftRule =========================================================
//
// Pattern: add(mul(x,y), mul(x,z)) → mul(x, add(y,z))

#[test]
fn factorize_left_fires() {
    // add(mul(x,a), mul(x,b)) → mul(x, add(a,b))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x.clone(), var('b', &mut f), &mut f),
        &mut f,
    );
    let expected = mul(x, add(var('a', &mut f), var('b', &mut f), &mut f), &mut f);
    let rule = FactorizeLeftRule::new("factL", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_left_no_fire_different_left_factors() {
    // add(mul(x,a), mul(y,b)) — left factors differ (x ≠ y)
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        mul(var('x', &mut f), var('a', &mut f), &mut f),
        mul(var('y', &mut f), var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftRule::new("factL", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn factorize_left_no_fire_inner_ops_differ() {
    // add(mul(x,a), add(x,b)) — inner operators Mul ≠ Add
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        add(x, var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftRule::new("factL", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn factorize_left_no_fire_outer_op_not_distributive() {
    // mul(mul(x,a), mul(x,b)) — Mul is not left-distributive over Mul
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x, var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftRule::new("factL", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == FactorizeRightRule ========================================================
//
// Pattern: add(mul(y,x), mul(z,x)) → mul(add(y,z), x)

#[test]
fn factorize_right_fires() {
    // add(mul(a,x), mul(b,x)) → mul(add(a,b), x)
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x.clone(), &mut f),
        &mut f,
    );
    let expected = mul(add(var('a', &mut f), var('b', &mut f), &mut f), x, &mut f);
    let rule = FactorizeRightRule::new("factR", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_right_no_fire_different_right_factors() {
    // add(mul(a,x), mul(b,y)) — right factors differ (x ≠ y)
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        mul(var('a', &mut f), var('x', &mut f), &mut f),
        mul(var('b', &mut f), var('y', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeRightRule::new("factR", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn factorize_right_no_fire_outer_op_not_distributive() {
    // mul(mul(a,x), mul(b,x)) — Mul is not right-distributive over Mul
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x, &mut f),
        &mut f,
    );
    let rule = FactorizeRightRule::new("factR", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == DefactorizeLeftRule =======================================================
//
// Pattern: mul(x, add(y,z)) → add(mul(x,y), mul(x,z))

#[test]
fn defactorize_left_fires() {
    // mul(x, add(a,b)) → add(mul(x,a), mul(x,b))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        x.clone(),
        add(var('a', &mut f), var('b', &mut f), &mut f),
        &mut f,
    );
    let expected = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x, var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = DefactorizeLeftRule::new("defactL", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn defactorize_left_no_fire_right_subterm_not_binary() {
    // mul(x, y) — right subterm y is a leaf, not binary
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = mul(var('x', &mut f), var('y', &mut f), &mut f);
    let rule = DefactorizeLeftRule::new("defactL", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn defactorize_left_no_fire_op_not_left_distributive() {
    // add(x, mul(a,b)) — Add is not left-distributive over Mul
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        var('x', &mut f),
        mul(var('a', &mut f), var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = DefactorizeLeftRule::new("defactL", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == DefactorizeRightRule ======================================================
//
// Pattern: mul(add(y,z), x) → add(mul(y,x), mul(z,x))

#[test]
fn defactorize_right_fires() {
    // mul(add(a,b), x) → add(mul(a,x), mul(b,x))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        add(var('a', &mut f), var('b', &mut f), &mut f),
        x.clone(),
        &mut f,
    );
    let expected = add(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x, &mut f),
        &mut f,
    );
    let rule = DefactorizeRightRule::new("defactR", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn defactorize_right_no_fire_left_subterm_not_binary() {
    // mul(x, y) — left subterm x is a leaf, not binary
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = mul(var('x', &mut f), var('y', &mut f), &mut f);
    let rule = DefactorizeRightRule::new("defactR", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn defactorize_right_no_fire_op_not_right_distributive() {
    // add(mul(a,b), x) — Add is not right-distributive over Mul
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        mul(var('a', &mut f), var('b', &mut f), &mut f),
        var('x', &mut f),
        &mut f,
    );
    let rule = DefactorizeRightRule::new("defactR", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == FactorizeLeftModACRule ====================================================
//
// Like FactorizeLeftRule but flattens the outer Add layer (AC) first, allowing
// e.g. add(mul(x,a), add(mul(x,b), w)) to be treated as a three-way sum.
//
// Rebuild order: remaining (non-factorized) sub-terms come first, then the new
// factorized term.  The list is folded right-associatively with Add.

#[test]
fn factorize_left_mod_ac_fires_two_subterms() {
    // add(mul(x,a), mul(x,b)) → mul(x, add(a,b))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x.clone(), var('b', &mut f), &mut f),
        &mut f,
    );
    let expected = mul(x, add(var('a', &mut f), var('b', &mut f), &mut f), &mut f);
    let rule = FactorizeLeftModACRule::new("factLAC", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_left_mod_ac_fires_with_associative_flattening() {
    // add(mul(x,a), add(mul(x,b), w))
    // Flattened Add layer: [mul(x,a), mul(x,b), w]
    // Common left factor x on indices 0 and 1 → factorized to mul(x, add(a,b))
    // Remaining: [w]; rebuilt: add(w, mul(x, add(a,b)))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        add(
            mul(x.clone(), var('b', &mut f), &mut f),
            var('w', &mut f),
            &mut f,
        ),
        &mut f,
    );
    let expected = add(
        var('w', &mut f),
        mul(x, add(var('a', &mut f), var('b', &mut f), &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftModACRule::new("factLAC", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_left_mod_ac_no_fire_outer_op_not_commutative() {
    // mul(mul(x,a), mul(x,b)) — Mul is not commutative, guard rejects it
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x, var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftModACRule::new("factLAC", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn factorize_left_mod_ac_no_fire_no_common_left_factor() {
    // add(mul(x,a), mul(y,b)) — x ≠ y, nothing to factorize
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        mul(var('x', &mut f), var('a', &mut f), &mut f),
        mul(var('y', &mut f), var('b', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeLeftModACRule::new("factLAC", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == FactorizeRightModACRule ===================================================
//
// Like FactorizeRightRule but modulo AC on the outer Add layer.

#[test]
fn factorize_right_mod_ac_fires_two_subterms() {
    // add(mul(a,x), mul(b,x)) → mul(add(a,b), x)
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x.clone(), &mut f),
        &mut f,
    );
    let expected = mul(add(var('a', &mut f), var('b', &mut f), &mut f), x, &mut f);
    let rule = FactorizeRightModACRule::new("factRAC", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_right_mod_ac_fires_with_associative_flattening() {
    // add(mul(a,x), add(mul(b,x), w))
    // Flattened Add layer: [mul(a,x), mul(b,x), w]
    // Common right factor x on indices 0 and 1 → factorized to mul(add(a,b), x)
    // Remaining: [w]; rebuilt: add(w, mul(add(a,b), x))
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = add(
        mul(var('a', &mut f), x.clone(), &mut f),
        add(
            mul(var('b', &mut f), x.clone(), &mut f),
            var('w', &mut f),
            &mut f,
        ),
        &mut f,
    );
    let expected = add(
        var('w', &mut f),
        mul(add(var('a', &mut f), var('b', &mut f), &mut f), x, &mut f),
        &mut f,
    );
    let rule = FactorizeRightModACRule::new("factRAC", ArithChecker);
    assert_eq!(rule.try_apply(&t, &t, &root_pos(), &mut f), Some(expected));
}

#[test]
fn factorize_right_mod_ac_no_fire_outer_op_not_commutative() {
    // mul(mul(a,x), mul(b,x)) — Mul is not commutative
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let t = mul(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x, &mut f),
        &mut f,
    );
    let rule = FactorizeRightModACRule::new("factRAC", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn factorize_right_mod_ac_no_fire_no_common_right_factor() {
    // add(mul(a,x), mul(b,y)) — x ≠ y, nothing to factorize
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let t = add(
        mul(var('a', &mut f), var('x', &mut f), &mut f),
        mul(var('b', &mut f), var('y', &mut f), &mut f),
        &mut f,
    );
    let rule = FactorizeRightModACRule::new("factRAC", ArithChecker);
    assert!(rule.try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == context guard: skip factorization when parent op == op2 (mod-AC rules) ==

#[test]
fn factorize_left_mod_ac_no_fire_when_parent_is_same_op() {
    // term = add(mul(x,a), mul(x,b)) — would normally be factorized.
    // When the context term has Add at its root and term sits at position [0],
    // the guard in transformation_factorize_left_distributive_modulo_ac detects
    // that the parent is the same commutative operator and returns None to avoid
    // redundant partial factorizations that should be handled from the parent.
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let term = add(
        mul(x.clone(), var('a', &mut f), &mut f),
        mul(x.clone(), var('b', &mut f), &mut f),
        &mut f,
    );
    let context = add(term.clone(), var('w', &mut f), &mut f);
    let child_pos = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0);
    let rule = FactorizeLeftModACRule::new("factLAC", ArithChecker);

    assert!(
        rule.try_apply(&term, &context, &child_pos, &mut f)
            .is_none(),
        "guard must block when parent operator == op2"
    );
    assert!(
        rule.try_apply(&term, &term, &root_pos(), &mut f).is_some(),
        "must fire at root where there is no parent"
    );
}

#[test]
fn factorize_right_mod_ac_no_fire_when_parent_is_same_op() {
    let mut f: TermFactory<ArithOp> = HConsign::empty();
    let x = var('x', &mut f);
    let term = add(
        mul(var('a', &mut f), x.clone(), &mut f),
        mul(var('b', &mut f), x.clone(), &mut f),
        &mut f,
    );
    let context = add(term.clone(), var('w', &mut f), &mut f);
    let child_pos = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0);
    let rule = FactorizeRightModACRule::new("factRAC", ArithChecker);

    assert!(
        rule.try_apply(&term, &context, &child_pos, &mut f)
            .is_none(),
        "guard must block when parent operator == op2"
    );
    assert!(
        rule.try_apply(&term, &term, &root_pos(), &mut f).is_some(),
        "must fire at root where there is no parent"
    );
}

// == get_desc =================================================================

#[test]
fn factorize_rule_get_desc() {
    assert_eq!(
        FactorizeLeftRule::new("factL", ArithChecker).get_desc(),
        "factL"
    );
    assert_eq!(
        FactorizeRightRule::new("factR", ArithChecker).get_desc(),
        "factR"
    );
}

#[test]
fn defactorize_rule_get_desc() {
    assert_eq!(
        DefactorizeLeftRule::new("defactL", ArithChecker).get_desc(),
        "defactL"
    );
    assert_eq!(
        DefactorizeRightRule::new("defactR", ArithChecker).get_desc(),
        "defactR"
    );
}

#[test]
fn factorize_mod_ac_rule_get_desc() {
    assert_eq!(
        FactorizeLeftModACRule::new("factLAC", ArithChecker).get_desc(),
        "factLAC"
    );
    assert_eq!(
        FactorizeRightModACRule::new("factRAC", ArithChecker).get_desc(),
        "factRAC"
    );
}
