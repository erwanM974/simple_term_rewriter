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

//! Tests for [`RewriteProcess::TryAllPaths`].
//!
//! `TryAllPaths(alternatives)` returns the union of all results from all
//! alternatives, including duplicates when multiple alternatives produce the
//! same term.  Fails (empty) only if every alternative fails.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::RewriteProcess;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

// == empty alternative list ====================================================

#[test]
fn try_all_paths_empty_list_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::TryAllPaths(vec![]);
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == all alternatives fail =====================================================

#[test]
fn try_all_paths_all_fail() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_epsilon()),
    ]);
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == one alternative fires =====================================================

#[test]
fn try_all_paths_one_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty): only rule_star_empty fires.
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_double_star()),
    ]);
    assert_eq!(rewrite(p, t, &mut f), vec![epsilon(&mut f)]);
}

// == two alternatives each fire, producing distinct results ====================

#[test]
fn try_all_paths_two_fire_distinct_results() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Alt(Empty, Empty): both rule_alt_left_empty and rule_alt_right_empty fire,
    // but both produce Empty → two copies of Empty in the result Vec.
    let t = alt(empty(&mut f), empty(&mut f), &mut f);
    let p = RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_alt_left_empty()),
        rule_as_process(rule_alt_right_empty()),
    ]);
    // Both rules fire and both produce Empty → duplicates are NOT deduplicated.
    let result = rewrite(p, t, &mut f);
    let e = empty(&mut f);
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|r| *r == e));
}

// == duplicate results are not deduplicated ====================================

#[test]
fn try_all_paths_duplicates_not_removed() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty): two identical copies of rule_star_empty → [Epsilon, Epsilon].
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::TryAllPaths(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_empty()),
    ]);
    let result = rewrite(p, t, &mut f);
    let e = epsilon(&mut f);
    assert_eq!(result, vec![e.clone(), e]);
}
