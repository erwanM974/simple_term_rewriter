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

//! Tests for the built-in term metrics:
//! [`tree_size`], [`dag_size`], [`term_depth`], [`operator_count_by_symbol`].

mod common;

use std::collections::HashMap;

use hashconsing::HConsign;

use simple_term_rewriter::metrics::{dag_size, operator_count_by_symbol, term_depth, tree_size};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::{alt, atom, concat, empty, epsilon, star};
use common::regex::lang::RegexOp;

// == tree_size =================================================================

#[test]
fn tree_size_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(tree_size(&epsilon(&mut f)), 1);
}

#[test]
fn tree_size_unary() {
    // star(atom) = 2 nodes
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(tree_size(&star(atom(b'a', &mut f), &mut f)), 2);
}

#[test]
fn tree_size_binary() {
    // alt(a, b) = 3 nodes
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        tree_size(&alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f)),
        3
    );
}

#[test]
fn tree_size_counts_duplicates() {
    // alt(atom('a'), atom('a')) — both leaves are counted even though structurally equal
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        tree_size(&alt(atom(b'a', &mut f), atom(b'a', &mut f), &mut f)),
        3
    );
}

#[test]
fn tree_size_nested() {
    // concat(star(atom('a')), alt(epsilon, empty)) = 1+2+3 = 6 nodes
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        tree_size(&concat(
            star(atom(b'a', &mut f), &mut f),
            alt(epsilon(&mut f), empty(&mut f), &mut f),
            &mut f
        )),
        6
    );
}

// == dag_size ==================================================================

#[test]
fn dag_size_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(dag_size(&epsilon(&mut f)), 1);
}

#[test]
fn dag_size_no_sharing() {
    // alt(atom('a'), atom('b')) — all three nodes are structurally distinct
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        dag_size(&alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f)),
        3
    );
}

#[test]
fn dag_size_with_sharing() {
    // alt(atom('a'), atom('a')) — the two atom('a') leaves are identical;
    // distinct sub-terms: {alt(atom('a'),atom('a')), atom('a')} → 2
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        dag_size(&alt(atom(b'a', &mut f), atom(b'a', &mut f), &mut f)),
        2
    );
}

#[test]
fn dag_size_repeated_subtree() {
    // concat(star(atom('a')), star(atom('a')))
    // Distinct: concat(...), star(atom('a')), atom('a') → 3
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(atom(b'a', &mut f), &mut f),
        star(atom(b'a', &mut f), &mut f),
        &mut f,
    );
    assert_eq!(dag_size(&t), 3);
}

#[test]
fn dag_size_no_sharing_mixed() {
    // concat(star(atom('a')), alt(epsilon, empty)) — all 6 nodes are distinct
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        dag_size(&concat(
            star(atom(b'a', &mut f), &mut f),
            alt(epsilon(&mut f), empty(&mut f), &mut f),
            &mut f
        )),
        6
    );
}

// == term_depth ================================================================

#[test]
fn term_depth_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(term_depth(&atom(b'x', &mut f)), 1);
}

#[test]
fn term_depth_unary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(term_depth(&star(atom(b'a', &mut f), &mut f)), 2);
}

#[test]
fn term_depth_binary_balanced() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        term_depth(&alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f)),
        2
    );
}

#[test]
fn term_depth_takes_max_branch() {
    // alt(atom, star(atom)) — right branch is deeper (depth 3)
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        term_depth(&alt(
            atom(b'a', &mut f),
            star(atom(b'b', &mut f), &mut f),
            &mut f
        )),
        3
    );
}

#[test]
fn term_depth_deeply_nested() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        term_depth(&star(
            star(star(atom(b'a', &mut f), &mut f), &mut f),
            &mut f
        )),
        4
    );
}

// == operator_count_by_symbol ==================================================

#[test]
fn op_count_single_leaf() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let counts = operator_count_by_symbol(&epsilon(&mut f));
    assert_eq!(counts, HashMap::from([(RegexOp::Epsilon, 1)]));
}

#[test]
fn op_count_unary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let counts = operator_count_by_symbol(&star(atom(b'a', &mut f), &mut f));
    assert_eq!(
        counts,
        HashMap::from([(RegexOp::Star, 1), (RegexOp::Atom(b'a'), 1),])
    );
}

#[test]
fn op_count_counts_repeated_symbols() {
    // alt(atom('a'), atom('a')) — Atom(b'a') appears twice
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let counts = operator_count_by_symbol(&alt(atom(b'a', &mut f), atom(b'a', &mut f), &mut f));
    assert_eq!(
        counts,
        HashMap::from([(RegexOp::Alt, 1), (RegexOp::Atom(b'a'), 2),])
    );
}

#[test]
fn op_count_mixed_term() {
    // concat(star(atom('a')), alt(epsilon, empty))
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(atom(b'a', &mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        &mut f,
    );
    let counts = operator_count_by_symbol(&t);
    assert_eq!(
        counts,
        HashMap::from([
            (RegexOp::Concat, 1),
            (RegexOp::Star, 1),
            (RegexOp::Atom(b'a'), 1),
            (RegexOp::Alt, 1),
            (RegexOp::Epsilon, 1),
            (RegexOp::Empty, 1),
        ])
    );
}
