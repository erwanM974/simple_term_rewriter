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

//! Unit tests for [`PartialACReorderRule`].
//!
//! Each test calls `try_apply` directly on the root of the `Then`-chain.
//! No traversal machinery is involved.
//!
//! The rule flattens the entire chain and sorts it in a single firing, so
//! each test is a single `try_apply` call.

use std::cmp::Ordering;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
#[allow(unused_imports)]
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::primitives::reorder_apc::{
    ModuloAssociativePartialReorderer, PartialACReorderRule,
};
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

// == test language =============================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum SeqOp {
    Then,
    Stop,
    Num(usize),
}

impl RewritableLanguageOperatorSymbol for SeqOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            SeqOp::Then => LanguageOperatorArity::Fixed(2),
            _ => LanguageOperatorArity::Fixed(0),
        }
    }
}

fn op_rank(op: &SeqOp) -> usize {
    match op {
        SeqOp::Then => usize::MAX,
        SeqOp::Stop => usize::MAX - 1,
        SeqOp::Num(n) => *n,
    }
}

struct ThenChecker;

impl ModuloAssociativePartialReorderer<SeqOp> for ThenChecker {
    fn is_an_associative_partially_commutative_binary_operator_we_may_consider(
        &self,
        op: &SeqOp,
    ) -> bool {
        *op == SeqOp::Then
    }
    fn may_commute_under(
        &self,
        _: &SeqOp,
        left: &LanguageTerm<SeqOp>,
        right: &LanguageTerm<SeqOp>,
    ) -> bool {
        left.operator != SeqOp::Stop && right.operator != SeqOp::Stop
    }
    fn compare_operators(&self, a: &SeqOp, b: &SeqOp) -> Ordering {
        op_rank(a).cmp(&op_rank(b))
    }
}

// == helpers ===================================================================

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

fn rule() -> impl RewriteRule<SeqOp> {
    PartialACReorderRule::new("reorder apc", ThenChecker)
}

/// Build a right-associated `Then`-chain from a slice of operators.
fn chain(elements: &[SeqOp], f: &mut TermFactory<SeqOp>) -> LanguageTerm<SeqOp> {
    assert!(!elements.is_empty());
    let leaves: Vec<LanguageTerm<SeqOp>> = elements
        .iter()
        .rev()
        .map(|op| LanguageTermNode::build(op.clone(), vec![], f))
        .collect();
    leaves
        .into_iter()
        .reduce(|right, left| LanguageTermNode::build(SeqOp::Then, vec![left, right], f))
        .unwrap()
}

/// Extract the flat element sequence from a `Then`-chain.
fn flatten(term: &LanguageTerm<SeqOp>) -> Vec<SeqOp> {
    match &term.operator {
        SeqOp::Then => {
            let mut v = flatten(&term.sub_terms[0]);
            v.extend(flatten(&term.sub_terms[1]));
            v
        }
        op => vec![op.clone()],
    }
}

/// Apply the rule once and return the flattened sequence of the result.
fn apply_flat(input: &[SeqOp]) -> Option<Vec<SeqOp>> {
    let mut f: TermFactory<SeqOp> = HConsign::empty();
    let t = chain(input, &mut f);
    rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .map(|r| flatten(&r))
}

// == no-change cases ===========================================================

#[test]
fn single_element_no_fire() {
    let mut f: TermFactory<SeqOp> = HConsign::empty();
    let t = LanguageTermNode::build(SeqOp::Num(5), vec![], &mut f);
    assert!(rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn two_elements_in_order_no_fire() {
    assert!(apply_flat(&[SeqOp::Num(1), SeqOp::Num(9)]).is_none());
}

#[test]
fn long_chain_in_order_no_fire() {
    let input = [
        SeqOp::Num(1),
        SeqOp::Num(3),
        SeqOp::Num(5),
        SeqOp::Num(7),
        SeqOp::Num(9),
    ];
    assert!(apply_flat(&input).is_none());
}

#[test]
fn chain_with_stop_in_order_no_fire() {
    let input = [
        SeqOp::Num(1),
        SeqOp::Num(3),
        SeqOp::Stop,
        SeqOp::Num(2),
        SeqOp::Num(7),
    ];
    assert!(apply_flat(&input).is_none());
}

// == full commutativity (no Stop) ==============================================

#[test]
fn two_elements_out_of_order_sorted_in_one_firing() {
    assert_eq!(
        apply_flat(&[SeqOp::Num(9), SeqOp::Num(1)]).unwrap(),
        vec![SeqOp::Num(1), SeqOp::Num(9)]
    );
}

#[test]
fn three_elements_sorted_in_one_firing() {
    // [3, 9, 7] → [3, 7, 9] in a single firing
    assert_eq!(
        apply_flat(&[SeqOp::Num(3), SeqOp::Num(9), SeqOp::Num(7)]).unwrap(),
        vec![SeqOp::Num(3), SeqOp::Num(7), SeqOp::Num(9)]
    );
}

#[test]
fn five_elements_sorted_in_one_firing() {
    assert_eq!(
        apply_flat(&[
            SeqOp::Num(5),
            SeqOp::Num(2),
            SeqOp::Num(9),
            SeqOp::Num(1),
            SeqOp::Num(7)
        ])
        .unwrap(),
        vec![
            SeqOp::Num(1),
            SeqOp::Num(2),
            SeqOp::Num(5),
            SeqOp::Num(7),
            SeqOp::Num(9)
        ]
    );
}

#[test]
fn sorted_result_is_irreducible() {
    let mut f: TermFactory<SeqOp> = HConsign::empty();
    let t = chain(&[SeqOp::Num(9), SeqOp::Num(3), SeqOp::Num(7)], &mut f);
    let sorted = rule()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .expect("should fire");
    assert!(
        rule()
            .try_apply(&sorted, &sorted, &root_pos(), &mut f)
            .is_none(),
        "sorted result must be irreducible"
    );
}

// == Stop barrier partitioning =================================================

#[test]
fn stop_at_start_sorts_the_rest() {
    assert_eq!(
        apply_flat(&[SeqOp::Stop, SeqOp::Num(3), SeqOp::Num(9), SeqOp::Num(7)]).unwrap(),
        vec![SeqOp::Stop, SeqOp::Num(3), SeqOp::Num(7), SeqOp::Num(9)]
    );
}

#[test]
fn stop_at_end_sorts_the_prefix() {
    assert_eq!(
        apply_flat(&[SeqOp::Num(9), SeqOp::Num(3), SeqOp::Num(7), SeqOp::Stop]).unwrap(),
        vec![SeqOp::Num(3), SeqOp::Num(7), SeqOp::Num(9), SeqOp::Stop]
    );
}

#[test]
fn stop_in_middle_creates_two_independent_segments() {
    assert_eq!(
        apply_flat(&[
            SeqOp::Num(3),
            SeqOp::Num(9),
            SeqOp::Num(7),
            SeqOp::Stop,
            SeqOp::Num(4),
            SeqOp::Num(1),
            SeqOp::Num(5),
        ])
        .unwrap(),
        vec![
            SeqOp::Num(3),
            SeqOp::Num(7),
            SeqOp::Num(9),
            SeqOp::Stop,
            SeqOp::Num(1),
            SeqOp::Num(4),
            SeqOp::Num(5),
        ]
    );
}

#[test]
fn two_adjacent_stops_divide_three_segments() {
    assert_eq!(
        apply_flat(&[
            SeqOp::Num(3),
            SeqOp::Num(9),
            SeqOp::Stop,
            SeqOp::Stop,
            SeqOp::Num(7),
            SeqOp::Num(2),
        ])
        .unwrap(),
        vec![
            SeqOp::Num(3),
            SeqOp::Num(9),
            SeqOp::Stop,
            SeqOp::Stop,
            SeqOp::Num(2),
            SeqOp::Num(7),
        ]
    );
}

#[test]
fn two_stops_three_independent_segments() {
    assert_eq!(
        apply_flat(&[
            SeqOp::Num(3),
            SeqOp::Num(9),
            SeqOp::Num(7),
            SeqOp::Stop,
            SeqOp::Num(4),
            SeqOp::Num(10),
            SeqOp::Stop,
            SeqOp::Num(2),
            SeqOp::Num(1),
            SeqOp::Num(6),
        ])
        .unwrap(),
        vec![
            SeqOp::Num(3),
            SeqOp::Num(7),
            SeqOp::Num(9),
            SeqOp::Stop,
            SeqOp::Num(4),
            SeqOp::Num(10),
            SeqOp::Stop,
            SeqOp::Num(1),
            SeqOp::Num(2),
            SeqOp::Num(6),
        ]
    );
}

// == get_desc =================================================================

#[test]
fn partial_ac_reorder_rule_get_desc() {
    assert_eq!(rule().get_desc(), "reorder apc");
}
