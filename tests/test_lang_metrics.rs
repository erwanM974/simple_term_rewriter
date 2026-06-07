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

//! Tests for `TermMetrics` and `TermSymbolMetric`.
//!
//! A locally-defined `RegexMetric` classifies each `RegexOp` as either a leaf,
//! a unary, or a binary operator.  Only unary operators (`Star`) have their
//! nesting depth tracked, so nested stars show up in `max_nested_metrics_depths`.

mod common;

use std::collections::{HashMap, HashSet};
use std::fmt;

use hashconsing::HConsign;

use simple_term_rewriter::metrics::{TermMetrics, TermSymbolMetric};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::{alt, atom, concat, epsilon, star};
use common::regex::lang::RegexOp;

// == metric definition =========================================================

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum RegexMetric {
    Leaf,
    Binary,
    Unary,
}

impl fmt::Display for RegexMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegexMetric::Leaf => write!(f, "leaf"),
            RegexMetric::Binary => write!(f, "binary"),
            RegexMetric::Unary => write!(f, "unary"),
        }
    }
}

impl TermSymbolMetric<RegexOp> for RegexMetric {
    fn measure_nested_depth(&self) -> bool {
        *self == RegexMetric::Unary
    }

    fn from_operator_symbol(op: &RegexOp) -> HashSet<Self> {
        let mut s = HashSet::new();
        match op {
            RegexOp::Empty | RegexOp::Epsilon | RegexOp::Atom(_) => {
                s.insert(RegexMetric::Leaf);
            }
            RegexOp::Alt | RegexOp::Concat => {
                s.insert(RegexMetric::Binary);
            }
            RegexOp::Star => {
                s.insert(RegexMetric::Unary);
            }
        }
        s
    }
}

fn metrics(
    term: &simple_term_rewriter::term::syntax::LanguageTerm<RegexOp>,
) -> TermMetrics<RegexOp, RegexMetric> {
    TermMetrics::extract_from_term(term)
}

// == leaf term: Atom('a') ======================================================

#[test]
fn metrics_leaf_term() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&atom(b'a', &mut f));
    assert_eq!(m.term_depth, 1);
    assert_eq!(m.metrics_count, HashMap::from([(RegexMetric::Leaf, 1)]));
    assert!(m.max_nested_metrics_depths.is_empty());
}

// == unary term: Star(Atom('a')) ===============================================

#[test]
fn metrics_unary_star_of_atom() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&star(atom(b'a', &mut f), &mut f));
    assert_eq!(m.term_depth, 2);
    assert_eq!(
        m.metrics_count,
        HashMap::from([(RegexMetric::Unary, 1), (RegexMetric::Leaf, 1),])
    );
    assert_eq!(
        m.max_nested_metrics_depths,
        HashMap::from([(RegexMetric::Unary, 1)])
    );
}

// == binary term: Alt(Atom('a'), Atom('b')) ====================================

#[test]
fn metrics_binary_alt_of_two_atoms() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f));
    assert_eq!(m.term_depth, 2);
    assert_eq!(
        m.metrics_count,
        HashMap::from([(RegexMetric::Binary, 1), (RegexMetric::Leaf, 2),])
    );
    assert!(m.max_nested_metrics_depths.is_empty());
}

// == nested stars track nesting depth =========================================
//
// Star(Star(Atom)) has two Star nodes and nesting depth 2.

#[test]
fn metrics_nested_stars_track_nesting_depth() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&star(star(atom(b'a', &mut f), &mut f), &mut f));
    assert_eq!(m.term_depth, 3);
    assert_eq!(
        m.metrics_count,
        HashMap::from([(RegexMetric::Unary, 2), (RegexMetric::Leaf, 1),])
    );
    assert_eq!(
        m.max_nested_metrics_depths,
        HashMap::from([(RegexMetric::Unary, 2)])
    );
}

// == sibling stars do not increase nesting depth ===============================
//
// Concat(Star(Atom), Star(Atom)): two Star nodes but neither is nested inside
// the other; max nesting depth remains 1.

#[test]
fn metrics_sibling_stars_do_not_increase_nesting_depth() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&concat(
        star(atom(b'a', &mut f), &mut f),
        star(atom(b'b', &mut f), &mut f),
        &mut f,
    ));
    assert_eq!(m.term_depth, 3);
    assert_eq!(
        m.metrics_count,
        HashMap::from([
            (RegexMetric::Binary, 1),
            (RegexMetric::Unary, 2),
            (RegexMetric::Leaf, 2),
        ])
    );
    assert_eq!(
        m.max_nested_metrics_depths,
        HashMap::from([(RegexMetric::Unary, 1)])
    );
}

// == string_summary format =====================================================

#[test]
fn string_summary_contains_expected_strings() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let m = metrics(&star(star(atom(b'a', &mut f), &mut f), &mut f));
    let summary: HashSet<String> = m.string_summary().into_iter().collect();
    assert!(summary.contains("term depth : 3"), "missing depth line");
    assert!(summary.contains("unary : 2"), "missing unary count");
    assert!(summary.contains("leaf : 1"), "missing leaf count");
    assert!(
        summary.contains("unary-max-nested-depth : 2"),
        "missing nested depth"
    );
    assert_eq!(summary.len(), 4);
}

// == compound term mixing all metric types =====================================
//
// Alt(Star(Atom('a')), Concat(Epsilon, Atom('b')))
// - Alt and Concat are binary (IsBinary count = 2)
// - Star is unary  (IsUnary count = 1)
// - Atom('a'), Epsilon, Atom('b') are leaves (IsLeaf count = 3)
// - term depth: Alt(depth 1) → Star/Concat (depth 2) → Atom/Epsilon/Atom (depth 3) = 3
// - Star nesting depth = 1 (only one level deep)

#[test]
fn metrics_mixed_compound_term() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(
        star(atom(b'a', &mut f), &mut f),
        concat(epsilon(&mut f), atom(b'b', &mut f), &mut f),
        &mut f,
    );
    let m = metrics(&t);
    assert_eq!(m.term_depth, 3);
    assert_eq!(
        m.metrics_count,
        HashMap::from([
            (RegexMetric::Binary, 2),
            (RegexMetric::Unary, 1),
            (RegexMetric::Leaf, 3),
        ])
    );
    assert_eq!(
        m.max_nested_metrics_depths,
        HashMap::from([(RegexMetric::Unary, 1)])
    );
}
