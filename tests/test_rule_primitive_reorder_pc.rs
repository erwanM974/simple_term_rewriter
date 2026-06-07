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

//! Unit tests for [`CommuteReorderRule`].
//!
//! Each test calls `try_apply` directly at the root position.
//! No traversal machinery is involved.
//!
//! The test language uses a single commutative binary operator `Pair`
//! (non-associative) with three constant leaves ordered A < B < C by LPO.

use std::cmp::Ordering;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
#[allow(unused_imports)]
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::primitives::reorder_pc::{
    CommutativeCheckerAndOrderer, CommuteReorderRule,
};
use simple_term_rewriter::term;
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory,
};

// == test language =============================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum PairOp {
    Pair,
    C,
    B,
    A,
}

impl RewritableLanguageOperatorSymbol for PairOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            PairOp::Pair => LanguageOperatorArity::Fixed(2),
            _ => LanguageOperatorArity::Fixed(0),
        }
    }
}

fn op_rank(op: &PairOp) -> u8 {
    match op {
        PairOp::Pair => 3,
        PairOp::C => 2,
        PairOp::B => 1,
        PairOp::A => 0,
    }
}

fn compare_ops(a: &PairOp, b: &PairOp) -> Ordering {
    op_rank(a).cmp(&op_rank(b))
}

// == checkers ==================================================================

/// Fully commutative: any pair of operands under `Pair` may be swapped.
struct FullChecker;

impl CommutativeCheckerAndOrderer<PairOp> for FullChecker {
    fn is_a_binary_commutative_operator(&self, op: &PairOp) -> bool {
        *op == PairOp::Pair
    }
    fn may_commute_under(
        &self,
        _: &PairOp,
        _: &LanguageTerm<PairOp>,
        _: &LanguageTerm<PairOp>,
    ) -> bool {
        true
    }
    fn compare_operators(&self, a: &PairOp, b: &PairOp) -> Ordering {
        compare_ops(a, b)
    }
}

/// Partially commutative: operands may NOT be swapped when either root is `C`.
struct PartialChecker;

impl CommutativeCheckerAndOrderer<PairOp> for PartialChecker {
    fn is_a_binary_commutative_operator(&self, op: &PairOp) -> bool {
        *op == PairOp::Pair
    }
    fn may_commute_under(
        &self,
        _: &PairOp,
        left: &LanguageTerm<PairOp>,
        right: &LanguageTerm<PairOp>,
    ) -> bool {
        left.operator != PairOp::C && right.operator != PairOp::C
    }
    fn compare_operators(&self, a: &PairOp, b: &PairOp) -> Ordering {
        compare_ops(a, b)
    }
}

// == helpers ===================================================================

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

fn full() -> impl RewriteRule<PairOp> {
    CommuteReorderRule::new("reorder full", FullChecker)
}
fn partial() -> impl RewriteRule<PairOp> {
    CommuteReorderRule::new("reorder partial", PartialChecker)
}

// == full commutativity ========================================================

#[test]
fn full_swap_b_a() {
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::B), term!(&mut f, PairOp::A));
    assert_eq!(
        full().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B)))
    );
}

#[test]
fn full_swap_c_a() {
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::C), term!(&mut f, PairOp::A));
    assert_eq!(
        full().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::C)))
    );
}

#[test]
fn full_swap_c_b() {
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::C), term!(&mut f, PairOp::B));
    assert_eq!(
        full().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::B), term!(&mut f, PairOp::C)))
    );
}

#[test]
fn full_no_fire_when_already_in_order() {
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B));
    assert!(full().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn full_no_fire_on_equal_operands() {
    // Pair(A, A) — equal, LPO is irreflexive, no swap
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::A));
    assert!(full().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn full_compound_operand_dominates_constant() {
    // Pair(Pair(A,B), A): Pair(A,B) >_lpo A → swap to Pair(A, Pair(A,B))
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair;
        term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B)),
        term!(&mut f, PairOp::A));
    assert_eq!(
        full().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair;
            term!(&mut f, PairOp::A),
            term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B))))
    );
}

#[test]
fn full_no_fire_on_wrong_operator() {
    // Leaf operator — not Pair, must not fire
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::A);
    assert!(full().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == partial commutativity =====================================================

#[test]
fn partial_allows_swap_when_no_c() {
    // B >_lpo A and neither is C → swap permitted
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::B), term!(&mut f, PairOp::A));
    assert_eq!(
        partial().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B)))
    );
}

#[test]
fn partial_blocks_swap_when_left_is_c() {
    // C >_lpo A, but left is C → may_commute_under returns false → no swap
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::C), term!(&mut f, PairOp::A));
    assert!(partial().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn partial_blocks_swap_when_right_is_c() {
    // Pair(C, B): C >_lpo B, but left is C → blocked
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::C), term!(&mut f, PairOp::B));
    assert!(partial().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn partial_blocks_swap_when_compound_left_would_need_move_but_right_is_c() {
    // Pair(Pair(A,B), C): Pair(A,B) >_lpo C.
    // Full checker: would swap → Pair(C, Pair(A,B)).
    // Partial checker: right is C → blocked.
    let mut f: TermFactory<PairOp> = HConsign::empty();
    let t = term!(&mut f, PairOp::Pair;
        term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B)),
        term!(&mut f, PairOp::C));

    // Full swaps
    assert_eq!(
        full().try_apply(&t, &t, &root_pos(), &mut f),
        Some(term!(&mut f, PairOp::Pair;
            term!(&mut f, PairOp::C),
            term!(&mut f, PairOp::Pair; term!(&mut f, PairOp::A), term!(&mut f, PairOp::B))))
    );
    // Partial blocks
    assert!(partial().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == get_desc =================================================================

#[test]
fn commute_reorder_rule_get_desc() {
    assert_eq!(full().get_desc(), "reorder full");
    assert_eq!(partial().get_desc(), "reorder partial");
}
