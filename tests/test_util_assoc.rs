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

//! Direct unit tests for [`get_associative_sub_terms_recursively`] and
//! [`fold_associative_sub_terms_recursively`].

mod common;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use hashconsing::HConsign;
use simple_term_rewriter::rules::util::assoc::{
    fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively,
};
use simple_term_rewriter::term::syntax::TermFactory;

// == get_associative_sub_terms_recursively =====================================

#[test]
fn get_assoc_leaf_non_matching_op() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // A leaf whose operator != Concat → returned as a single-element Vec.
    let t = epsilon(&mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 1);
    assert_eq!(*result[0], t);
}

#[test]
fn get_assoc_binary_non_matching_root() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Alt(a, b): root is Alt, not Concat → the whole term is treated as a leaf.
    let t = alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 1);
    assert_eq!(*result[0], t);
}

#[test]
fn get_assoc_flat_pair() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(a, b): root matches, two non-Concat children → [a, b].
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let t = concat(a.clone(), b.clone(), &mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 2);
    assert_eq!(*result[0], a);
    assert_eq!(*result[1], b);
}

#[test]
fn get_assoc_right_associated_chain() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(a, Concat(b, c)) → [a, b, c].
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let t = concat(a.clone(), concat(b.clone(), c.clone(), &mut f), &mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 3);
    assert_eq!(*result[0], a);
    assert_eq!(*result[1], b);
    assert_eq!(*result[2], c);
}

#[test]
fn get_assoc_left_associated_chain() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Concat(a, b), c) → [a, b, c].
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let t = concat(concat(a.clone(), b.clone(), &mut f), c.clone(), &mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 3);
    assert_eq!(*result[0], a);
    assert_eq!(*result[1], b);
    assert_eq!(*result[2], c);
}

#[test]
fn get_assoc_four_elements_mixed_association() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Concat(a, b), Concat(c, d)) → [a, b, c, d].
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let d = atom(b'd', &mut f);
    let t = concat(
        concat(a.clone(), b.clone(), &mut f),
        concat(c.clone(), d.clone(), &mut f),
        &mut f,
    );
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 4);
    assert_eq!(*result[0], a);
    assert_eq!(*result[1], b);
    assert_eq!(*result[2], c);
    assert_eq!(*result[3], d);
}

#[test]
fn get_assoc_stops_at_non_matching_subtree() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Alt(a, b), c): Alt is not Concat, so Alt(a,b) is a leaf.
    let inner = alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f);
    let c = atom(b'c', &mut f);
    let t = concat(inner.clone(), c.clone(), &mut f);
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 2);
    assert_eq!(*result[0], inner);
    assert_eq!(*result[1], c);
}

#[test]
fn get_assoc_preserves_order() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(a, Concat(b, Concat(c, Concat(d, e)))) → [a, b, c, d, e] in order.
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let d = atom(b'd', &mut f);
    let e = atom(b'e', &mut f);
    let t = concat(
        a.clone(),
        concat(
            b.clone(),
            concat(c.clone(), concat(d.clone(), e.clone(), &mut f), &mut f),
            &mut f,
        ),
        &mut f,
    );
    let result = get_associative_sub_terms_recursively(&t, &RegexOp::Concat);
    assert_eq!(result.len(), 5);
    for (got, expected) in result.iter().zip(&[a, b, c, d, e]) {
        assert_eq!(*got, expected);
    }
}

// == fold_associative_sub_terms_recursively ====================================

#[test]
fn fold_empty_no_default_returns_none() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let mut v = vec![];
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut v, &None, &mut f);
    assert!(result.is_none());
}

#[test]
fn fold_empty_with_default_returns_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let mut v = vec![];
    let result = fold_associative_sub_terms_recursively(
        &RegexOp::Concat,
        &mut v,
        &Some(RegexOp::Epsilon),
        &mut f,
    );
    assert_eq!(result, Some(epsilon(&mut f)));
}

#[test]
fn fold_one_element_returns_it_unchanged() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(atom(b'x', &mut f), &mut f);
    let mut v = vec![t.clone()];
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut v, &None, &mut f);
    assert_eq!(result, Some(t));
}

#[test]
fn fold_two_elements_returns_binary_node() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let mut v = vec![a.clone(), b.clone()];
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut v, &None, &mut f);
    assert_eq!(result, Some(concat(a, b, &mut f)));
}

#[test]
fn fold_three_elements_is_right_associated() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // [a, b, c] → Concat(a, Concat(b, c)).
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let mut v = vec![a.clone(), b.clone(), c.clone()];
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut v, &None, &mut f);
    assert_eq!(result, Some(concat(a, concat(b, c, &mut f), &mut f)));
}

#[test]
fn fold_four_elements_is_right_associated() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // [a, b, c, d] → Concat(a, Concat(b, Concat(c, d))).
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let d = atom(b'd', &mut f);
    let mut v = vec![a.clone(), b.clone(), c.clone(), d.clone()];
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut v, &None, &mut f);
    assert_eq!(
        result,
        Some(concat(a, concat(b, concat(c, d, &mut f), &mut f), &mut f))
    );
}

// == round-trips ===============================================================

#[test]
fn round_trip_right_associated_chain_is_identity() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let original = concat(a, concat(b, c, &mut f), &mut f);
    let mut flat: Vec<_> = get_associative_sub_terms_recursively(&original, &RegexOp::Concat)
        .into_iter()
        .cloned()
        .collect();
    let reconstructed =
        fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut flat, &None, &mut f);
    assert_eq!(reconstructed, Some(original));
}

#[test]
fn round_trip_left_chain_gives_right_associated() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let a = atom(b'a', &mut f);
    let b = atom(b'b', &mut f);
    let c = atom(b'c', &mut f);
    let left_chain = concat(concat(a.clone(), b.clone(), &mut f), c.clone(), &mut f);
    let right_chain = concat(a, concat(b, c, &mut f), &mut f);
    let mut flat: Vec<_> = get_associative_sub_terms_recursively(&left_chain, &RegexOp::Concat)
        .into_iter()
        .cloned()
        .collect();
    let reconstructed =
        fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut flat, &None, &mut f);
    assert_eq!(reconstructed, Some(right_chain));
}

#[test]
fn round_trip_single_leaf_is_identity() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(b'z', &mut f);
    let mut flat: Vec<_> = get_associative_sub_terms_recursively(&t, &RegexOp::Concat)
        .into_iter()
        .cloned()
        .collect();
    let result = fold_associative_sub_terms_recursively(&RegexOp::Concat, &mut flat, &None, &mut f);
    assert_eq!(result, Some(t));
}
