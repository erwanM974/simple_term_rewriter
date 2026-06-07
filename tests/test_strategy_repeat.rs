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

//! Tests for [`RewriteProcess::Repeat`].
//!
//! `Repeat(p)` applies `p` until it produces no result, then returns the last
//! successful state.  It never fails: if `p` never fires, the original term is
//! returned unchanged.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::{DepthOrder, RewriteProcess, SiblingOrder};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

// == inner process never fires → original term returned =======================

#[test]
fn repeat_never_fires_returns_original() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Atom(a): rule_double_star never fires → Repeat returns [Atom(a)].
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_double_star()))));
    assert_eq!(rewrite(p, t.clone(), &mut f), vec![t]);
}

// == single step to fixpoint ===================================================

#[test]
fn repeat_single_step_to_fixpoint() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty): rule_star_empty fires once → Epsilon, then stops.
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_star_empty()))));
    assert_eq!(rewrite(p, t, &mut f), vec![epsilon(&mut f)]);
}

// == multi-step reduction ======================================================

#[test]
fn repeat_multi_step_double_star() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Star(Star(Star(a)))): four nested stars.
    // rule_double_star fires repeatedly until only Star(a) remains.
    let t = star(
        star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f),
        &mut f,
    );
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_double_star()))));
    assert_eq!(
        rewrite(p, t, &mut f),
        vec![star(atom(b'a', &mut f), &mut f)]
    );
}

// == repeat with branching inner process ======================================

#[test]
fn repeat_with_try_all_paths_explores_every_branch_to_fixpoint() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Alt(Empty, Empty) under TryAllPaths([rule_alt_left_empty, rule_alt_right_empty]):
    // Both rules fire at the root, each independently producing Empty.
    // Repeat recurses on each of the two results; Empty is irreducible.
    // Final: [Empty, Empty] — two fixpoints, one per branch, duplicates preserved.
    let t = alt(empty(&mut f), empty(&mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_alt_left_empty()),
        rule_as_process(rule_alt_right_empty()),
    ])));
    assert_eq!(rewrite(p, t, &mut f), vec![empty(&mut f), empty(&mut f)]);
}

#[test]
fn repeat_with_try_all_paths_reduces_each_branch_independently() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty) under TryAllPaths([rule_star_empty, rule_star_empty]):
    // Both copies of the same rule fire → [Epsilon, Epsilon] after the first step.
    // Repeat recurses: Epsilon is irreducible → final [Epsilon, Epsilon].
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_empty()),
    ])));
    assert_eq!(
        rewrite(p, t, &mut f),
        vec![epsilon(&mut f), epsilon(&mut f)]
    );
}

// == repeat with traversal: normalize a whole term ============================

#[test]
fn repeat_with_anychild_normalizes_term() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Star(Star(a)), Concat(Epsilon, Star(Star(b)))):
    // Applying double_star everywhere until fixpoint should yield
    // Concat(Star(a), Concat(Epsilon, Star(b))).
    let t = concat(
        star(star(atom(b'a', &mut f), &mut f), &mut f),
        concat(
            epsilon(&mut f),
            star(star(atom(b'b', &mut f), &mut f), &mut f),
            &mut f,
        ),
        &mut f,
    );
    let one_step = RewriteProcess::AnyChild(
        SiblingOrder::Leftmost,
        DepthOrder::Innermost,
        Box::new(RewriteProcess::Rule(Box::new(rule_double_star()))),
    );
    let p = RewriteProcess::Repeat(Box::new(one_step));
    let expected = concat(
        star(atom(b'a', &mut f), &mut f),
        concat(epsilon(&mut f), star(atom(b'b', &mut f), &mut f), &mut f),
        &mut f,
    );
    assert_eq!(rewrite(p, t, &mut f), vec![expected]);
}
