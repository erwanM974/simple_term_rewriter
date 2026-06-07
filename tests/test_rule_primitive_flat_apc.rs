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

//! Unit tests for [`FlattenedACTransfoRule`].
//!
//! Each test calls `try_apply` directly at the root position.
//! No traversal machinery is involved.
//!
//! Domain: multiset of typed items.  `Sum` is the AC accumulator.
//! `Item(count, kind)` is a signed quantity of a particular kind.

use std::collections::BTreeMap;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
#[allow(unused_imports)]
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::primitives::flat_apc::{
    FlattenedACTransfoRule, ModuloAssociativeGenericFlattenedChecker,
};
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

// == test language =============================================================

const APPLE: u8 = 0;
const PEAR: u8 = 1;
const ORANGE: u8 = 2;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum SumOp {
    Sum,
    Item(i32, u8),
}

impl RewritableLanguageOperatorSymbol for SumOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            SumOp::Sum => LanguageOperatorArity::Fixed(2),
            SumOp::Item(_, _) => LanguageOperatorArity::Fixed(0),
        }
    }
}

fn item(count: i32, kind: u8, f: &mut TermFactory<SumOp>) -> LanguageTerm<SumOp> {
    LanguageTermNode::build(SumOp::Item(count, kind), vec![], f)
}

fn sum2(
    a: LanguageTerm<SumOp>,
    b: LanguageTerm<SumOp>,
    f: &mut TermFactory<SumOp>,
) -> LanguageTerm<SumOp> {
    LanguageTermNode::build(SumOp::Sum, vec![a, b], f)
}

// == checker ===================================================================

struct MergeItems;

impl ModuloAssociativeGenericFlattenedChecker<SumOp> for MergeItems {
    fn is_an_associative_binary_operator_we_may_consider(&self, op: &SumOp) -> bool {
        *op == SumOp::Sum
    }

    fn transform_flattened_sub_terms(
        &self,
        _ac_op: &SumOp,
        items: Vec<&LanguageTerm<SumOp>>,
        f: &mut TermFactory<SumOp>,
    ) -> Option<Vec<LanguageTerm<SumOp>>> {
        let mut counts: BTreeMap<u8, i32> = BTreeMap::new();
        for it in &items {
            if let SumOp::Item(count, kind) = it.operator {
                *counts.entry(kind).or_insert(0) += count;
            }
        }
        let new_items: Vec<LanguageTerm<SumOp>> = counts
            .into_iter()
            .filter(|(_, count)| *count != 0)
            .map(|(kind, count)| LanguageTermNode::build(SumOp::Item(count, kind), vec![], f))
            .collect();
        let already_canonical =
            items.len() == new_items.len() && items.iter().zip(&new_items).all(|(a, b)| *a == b);
        if already_canonical {
            None
        } else {
            Some(new_items)
        }
    }
}

fn make_rule() -> impl RewriteRule<SumOp> {
    FlattenedACTransfoRule::new("merge items", MergeItems)
}

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

// == no-change cases ===========================================================

#[test]
fn single_item_no_fire() {
    // A single leaf is not a Sum node; the rule must not fire.
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = item(5, APPLE, &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn two_distinct_items_no_fire() {
    // Sum(3·apple, 5·pear) — different kinds, already canonical.
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let a = item(3, APPLE, &mut f);
    let p = item(5, PEAR, &mut f);
    let t = sum2(a, p, &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn three_distinct_items_no_fire() {
    // Sum(1·apple, Sum(2·pear, 3·orange)) — all distinct, canonical order.
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let a = item(1, APPLE, &mut f);
    let p = item(2, PEAR, &mut f);
    let o = item(3, ORANGE, &mut f);
    let inner = sum2(p, o, &mut f);
    let t = sum2(a, inner, &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

#[test]
fn negative_count_stays_when_nonzero() {
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let a = item(-3, APPLE, &mut f);
    let p = item(5, PEAR, &mut f);
    let t = sum2(a, p, &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}

// == merging like items ========================================================

#[test]
fn two_items_same_kind_merge() {
    // Sum(3·apple, 5·apple) → 8·apple
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(item(3, APPLE, &mut f), item(5, APPLE, &mut f), &mut f);
    let expected = item(8, APPLE, &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

#[test]
fn three_items_same_kind_merge_in_one_firing() {
    // Sum(Sum(1·apple, 2·apple), 3·apple) — flattened and merged in one firing.
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let inner = sum2(item(1, APPLE, &mut f), item(2, APPLE, &mut f), &mut f);
    let t = sum2(inner, item(3, APPLE, &mut f), &mut f);
    let expected = item(6, APPLE, &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

#[test]
fn mixed_kinds_merge_and_reorder() {
    // Sum(1·apple, Sum(2·pear, Sum(3·apple, 1·pear)))
    // Flattened: [1·apple, 2·pear, 3·apple, 1·pear] → merged: {apple:4, pear:3}
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(
        item(1, APPLE, &mut f),
        sum2(
            item(2, PEAR, &mut f),
            sum2(item(3, APPLE, &mut f), item(1, PEAR, &mut f), &mut f),
            &mut f,
        ),
        &mut f,
    );
    let expected = sum2(item(4, APPLE, &mut f), item(3, PEAR, &mut f), &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

#[test]
fn reordering_to_canonical_kind_order() {
    // Sum(5·pear, 3·apple) → Sum(3·apple, 5·pear)  (apple < pear by kind id)
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(item(5, PEAR, &mut f), item(3, APPLE, &mut f), &mut f);
    let expected = sum2(item(3, APPLE, &mut f), item(5, PEAR, &mut f), &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

// == cancellation =============================================================

#[test]
fn opposite_counts_cancel() {
    // Sum(5·apple, Sum(−5·apple, 2·pear)) → 2·pear
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(
        item(5, APPLE, &mut f),
        sum2(item(-5, APPLE, &mut f), item(2, PEAR, &mut f), &mut f),
        &mut f,
    );
    let expected = item(2, PEAR, &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

#[test]
fn partial_cancellation_leaves_remainder() {
    // Sum(7·apple, Sum(−3·apple, 4·pear)) → Sum(4·apple, 4·pear)
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(
        item(7, APPLE, &mut f),
        sum2(item(-3, APPLE, &mut f), item(4, PEAR, &mut f), &mut f),
        &mut f,
    );
    let expected = sum2(item(4, APPLE, &mut f), item(4, PEAR, &mut f), &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

#[test]
fn cancellation_across_three_kinds() {
    // Sum(2·orange, Sum(5·apple, Sum(−5·apple, Sum(−2·orange, 3·pear)))) → 3·pear
    let mut f: TermFactory<SumOp> = HConsign::empty();
    let t = sum2(
        item(2, ORANGE, &mut f),
        sum2(
            item(5, APPLE, &mut f),
            sum2(
                item(-5, APPLE, &mut f),
                sum2(item(-2, ORANGE, &mut f), item(3, PEAR, &mut f), &mut f),
                &mut f,
            ),
            &mut f,
        ),
        &mut f,
    );
    let expected = item(3, PEAR, &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(expected)
    );
}

// == get_desc =================================================================

#[test]
fn flattened_ac_transfo_rule_get_desc() {
    assert_eq!(make_rule().get_desc(), "merge items");
}
