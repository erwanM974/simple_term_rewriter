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

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;

// == sub-term traversal (happy path) ==========================================

#[test]
fn sub_term_traversal_two_level_tree() {
    // Concat(Star(Epsilon), Star(Empty))
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = concat(
        star(epsilon(&mut f), &mut f),
        star(empty(&mut f), &mut f),
        &mut f,
    );

    let root = PositionInLanguageTerm::get_root_position();
    let p1 = root.get_position_of_nth_child(0);
    let p11 = p1.get_position_of_nth_child(0);
    let p2 = root.get_position_of_nth_child(1);
    let p21 = p2.get_position_of_nth_child(0);

    assert_eq!(
        term.get_sub_term_at_position(&root).unwrap().operator,
        RegexOp::Concat
    );
    assert_eq!(
        term.get_sub_term_at_position(&p1).unwrap().operator,
        RegexOp::Star
    );
    assert_eq!(
        term.get_sub_term_at_position(&p11).unwrap().operator,
        RegexOp::Epsilon
    );
    assert_eq!(
        term.get_sub_term_at_position(&p2).unwrap().operator,
        RegexOp::Star
    );
    assert_eq!(
        term.get_sub_term_at_position(&p21).unwrap().operator,
        RegexOp::Empty
    );

    assert_eq!(p1.get_parent_position().unwrap(), root);
    assert_eq!(p11.get_parent_position().unwrap(), p1);
    assert_eq!(p2.get_parent_position().unwrap(), root);
    assert_eq!(p21.get_parent_position().unwrap(), p2);
}

#[test]
fn sub_term_traversal_three_level_tree() {
    // Star(Alt(Epsilon, Concat(Empty, Epsilon)))
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = star(
        alt(
            epsilon(&mut f),
            concat(empty(&mut f), epsilon(&mut f), &mut f),
            &mut f,
        ),
        &mut f,
    );

    let root = PositionInLanguageTerm::get_root_position();
    let p1 = root.get_position_of_nth_child(0);
    let p11 = p1.get_position_of_nth_child(0);
    let p12 = p1.get_position_of_nth_child(1);
    let p121 = p12.get_position_of_nth_child(0);
    let p122 = p12.get_position_of_nth_child(1);

    assert_eq!(
        term.get_sub_term_at_position(&root).unwrap().operator,
        RegexOp::Star
    );
    assert_eq!(
        term.get_sub_term_at_position(&p1).unwrap().operator,
        RegexOp::Alt
    );
    assert_eq!(
        term.get_sub_term_at_position(&p11).unwrap().operator,
        RegexOp::Epsilon
    );
    assert_eq!(
        term.get_sub_term_at_position(&p12).unwrap().operator,
        RegexOp::Concat
    );
    assert_eq!(
        term.get_sub_term_at_position(&p121).unwrap().operator,
        RegexOp::Empty
    );
    assert_eq!(
        term.get_sub_term_at_position(&p122).unwrap().operator,
        RegexOp::Epsilon
    );

    assert_eq!(p1.get_parent_position().unwrap(), root);
    assert_eq!(p11.get_parent_position().unwrap(), p1);
    assert_eq!(p12.get_parent_position().unwrap(), p1);
    assert_eq!(p121.get_parent_position().unwrap(), p12);
    assert_eq!(p122.get_parent_position().unwrap(), p12);
}

// == depth =====================================================================

#[test]
fn depth_of_root_is_zero() {
    assert_eq!(PositionInLanguageTerm::get_root_position().get_depth(), 0);
}

#[test]
fn depth_increases_with_each_child_step() {
    let root = PositionInLanguageTerm::get_root_position();
    let p1 = root.get_position_of_nth_child(0);
    let p12 = p1.get_position_of_nth_child(1);
    let p123 = p12.get_position_of_nth_child(2);

    assert_eq!(root.get_depth(), 0);
    assert_eq!(p1.get_depth(), 1);
    assert_eq!(p12.get_depth(), 2);
    assert_eq!(p123.get_depth(), 3);
}

// == absolute coordinates ======================================================

#[test]
fn root_has_empty_coordinates() {
    let root = PositionInLanguageTerm::get_root_position();
    assert!(root.get_absolute_coordinates_from_root().is_empty());
}

#[test]
fn child_coordinates_are_appended() {
    let root = PositionInLanguageTerm::get_root_position();
    let p2 = root.get_position_of_nth_child(2);
    let p21 = p2.get_position_of_nth_child(1);
    let p210 = p21.get_position_of_nth_child(0);

    assert_eq!(root.get_absolute_coordinates_from_root(), &[] as &[usize]);
    assert_eq!(p2.get_absolute_coordinates_from_root(), &[2]);
    assert_eq!(p21.get_absolute_coordinates_from_root(), &[2, 1]);
    assert_eq!(p210.get_absolute_coordinates_from_root(), &[2, 1, 0]);
}

#[test]
fn from_absolute_coordinates_round_trips() {
    // Positions built from coordinates must equal those built by child steps.
    let root = PositionInLanguageTerm::get_root_position();
    let by_steps = root
        .get_position_of_nth_child(1)
        .get_position_of_nth_child(3);

    let by_coords = PositionInLanguageTerm::from_absolute_coordinates(vec![1, 3]);

    assert_eq!(by_steps, by_coords);
    assert_eq!(by_coords.get_absolute_coordinates_from_root(), &[1, 3]);
}

// == parent position ===========================================================

#[test]
fn parent_of_root_is_none() {
    assert!(PositionInLanguageTerm::get_root_position()
        .get_parent_position()
        .is_none());
}

#[test]
fn parent_of_depth_one_is_root() {
    let root = PositionInLanguageTerm::get_root_position();
    let child = root.get_position_of_nth_child(5);
    assert_eq!(child.get_parent_position().unwrap(), root);
}

// == Display format ============================================================

#[test]
fn display_root_is_epsilon() {
    let root = PositionInLanguageTerm::get_root_position();
    assert_eq!(format!("{root}"), "ε");
}

#[test]
fn display_non_root_uses_underscore_separated_indices() {
    let root = PositionInLanguageTerm::get_root_position();
    let p0 = root.get_position_of_nth_child(0);
    let p01 = p0.get_position_of_nth_child(1);
    let p012 = p01.get_position_of_nth_child(2);

    assert_eq!(format!("{p0}"), "0");
    assert_eq!(format!("{p01}"), "0_1");
    assert_eq!(format!("{p012}"), "0_1_2");
}

// == invalid path returns None =================================================

#[test]
fn get_sub_term_at_out_of_bounds_child_index_returns_none() {
    // Epsilon is a leaf; requesting child 0 of a leaf must return None.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = epsilon(&mut f);
    let pos = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0);
    assert!(term.get_sub_term_at_position(&pos).is_none());
}

#[test]
fn get_sub_term_at_deep_invalid_path_returns_none() {
    // Concat(Epsilon, Empty): child 2 does not exist; neither does child 0 of child 2.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let bad_child = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(2);
    let bad_deeper = bad_child.get_position_of_nth_child(0);
    assert!(term.get_sub_term_at_position(&bad_child).is_none());
    assert!(term.get_sub_term_at_position(&bad_deeper).is_none());
}
