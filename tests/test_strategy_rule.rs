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

//! Tests for [`RewriteProcess::Rule`].
//!
//! `Rule(r)` applies `r` at the root of the current term only.
//! These tests verify that it fires exactly when the rule matches the root,
//! and does not traverse into sub-terms.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::RewriteProcess;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

// == Rule fires at root ========================================================

#[test]
fn rule_fires_at_root() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Empty) matches the root → should return [Epsilon]
    let t = star(empty(&mut f), &mut f);
    let result = rewrite(RewriteProcess::Rule(Box::new(rule_star_empty())), t, &mut f);
    assert_eq!(result, vec![epsilon(&mut f)]);
}

#[test]
fn rule_does_not_fire_at_root_wrong_operator() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Empty, Epsilon): root is Concat, not Star → rule_star_empty must not fire
    let t = concat(empty(&mut f), epsilon(&mut f), &mut f);
    let result = rewrite(
        RewriteProcess::Rule(Box::new(rule_star_empty())),
        t.clone(),
        &mut f,
    );
    assert!(result.is_empty());
}

#[test]
fn rule_does_not_descend_into_sub_terms() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Alt(Star(Empty), Epsilon): the Star(Empty) is a sub-term, not the root.
    // Rule(r) must not descend — result must be empty.
    let t = alt(star(empty(&mut f), &mut f), epsilon(&mut f), &mut f);
    let result = rewrite(RewriteProcess::Rule(Box::new(rule_star_empty())), t, &mut f);
    assert!(result.is_empty());
}

#[test]
fn rule_on_leaf_term_no_match() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Atom has no sub-terms; rule_star_empty must return empty.
    let t = atom(b'a', &mut f);
    let result = rewrite(RewriteProcess::Rule(Box::new(rule_star_empty())), t, &mut f);
    assert!(result.is_empty());
}

#[test]
fn rule_concat_left_epsilon_fires() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Epsilon, Atom(a)) → Atom(a)
    let t = concat(epsilon(&mut f), atom(b'a', &mut f), &mut f);
    let result = rewrite(
        RewriteProcess::Rule(Box::new(rule_concat_left_epsilon())),
        t,
        &mut f,
    );
    assert_eq!(result, vec![atom(b'a', &mut f)]);
}
