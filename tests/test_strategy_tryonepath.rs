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

//! Tests for [`RewriteProcess::TryOnePath`].
//!
//! `TryOnePath(alternatives)` returns the results of the first alternative
//! that succeeds, skipping all subsequent ones.  Fails if all alternatives fail.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::RewriteProcess;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

// == empty alternative list ====================================================

#[test]
fn try_one_path_empty_list_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::TryOnePath(vec![]);
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == first alternative fires ===================================================

#[test]
fn try_one_path_first_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty): rule_star_empty fires first → [Epsilon], rule_star_epsilon skipped.
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_epsilon()),
    ]);
    assert_eq!(rewrite(p, t, &mut f), vec![epsilon(&mut f)]);
}

// == first fails, second fires =================================================

#[test]
fn try_one_path_second_fires_when_first_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Epsilon): rule_star_empty does not fire (not Empty), rule_star_epsilon fires.
    let t = star(epsilon(&mut f), &mut f);
    let p = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_epsilon()),
    ]);
    assert_eq!(rewrite(p, t, &mut f), vec![epsilon(&mut f)]);
}

// == all alternatives fail =====================================================

#[test]
fn try_one_path_all_fail() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Atom(a): neither rule fires.
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_star_epsilon()),
        rule_as_process(rule_double_star()),
    ]);
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == priority: earlier alternative wins =======================================

#[test]
fn try_one_path_earlier_wins_when_both_applicable() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Star(Empty)): both rule_double_star and rule_star_empty could eventually
    // apply, but rule_double_star fires at the root directly.
    let t = star(star(empty(&mut f), &mut f), &mut f);
    let p_double_first = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    ]);
    let p_empty_first = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_star_empty()),
        rule_as_process(rule_double_star()),
    ]);
    // double_star fires on Star(Star(Empty)) → Star(Empty)
    assert_eq!(
        rewrite(p_double_first, t.clone(), &mut f),
        vec![star(empty(&mut f), &mut f)]
    );
    // star_empty does NOT fire on Star(Star(Empty)) (inner is Star, not Empty)
    // so it falls through to double_star → Star(Empty)
    assert_eq!(
        rewrite(p_empty_first, t, &mut f),
        vec![star(empty(&mut f), &mut f)]
    );
}
