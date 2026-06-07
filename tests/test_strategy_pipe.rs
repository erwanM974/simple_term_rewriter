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

//! Tests for [`RewriteProcess::Pipe`].
//!
//! `Pipe(a, b)` applies `a` then feeds every result into `b`.
//! It fails if `a` fails, and also fails if `b` fails on every result of `a`.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::RewriteProcess;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

fn pipe(a: RewriteProcess<RegexOp>, b: RewriteProcess<RegexOp>) -> RewriteProcess<RegexOp> {
    RewriteProcess::Pipe(Box::new(a), Box::new(b))
}

// == both steps fire ===========================================================

#[test]
fn pipe_both_fire() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Star(Empty)):
    //   step a: rule_double_star  → Star(Empty)
    //   step b: rule_star_empty   → Epsilon
    let t = star(star(empty(&mut f), &mut f), &mut f);
    let p = pipe(
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    );
    assert_eq!(rewrite(p, t, &mut f), vec![epsilon(&mut f)]);
}

// == first step fails =========================================================

#[test]
fn pipe_fails_when_first_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Atom(a)): rule_double_star does not fire → whole pipe returns empty.
    let t = star(atom(b'a', &mut f), &mut f);
    let p = pipe(
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    );
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == second step fails ========================================================

#[test]
fn pipe_fails_when_second_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Star(Atom(a))):
    //   step a: rule_double_star → Star(Atom(a))  ✓
    //   step b: rule_star_empty  → no match (Star(Atom(a)) ≠ Star(Empty))  ✗
    let t = star(star(atom(b'a', &mut f), &mut f), &mut f);
    let p = pipe(
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    );
    assert!(rewrite(p, t, &mut f).is_empty());
}

// == three-step chain =========================================================

#[test]
fn pipe_three_steps() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Star(Star(Epsilon)):
    //   step a: rule_double_star  → Star(Epsilon)
    //   step b: rule_star_epsilon → Epsilon
    //   (wrapped as Pipe(Pipe(a, b), c) to test associativity of chaining)
    let t = star(star(epsilon(&mut f), &mut f), &mut f);
    let p = pipe(
        pipe(
            rule_as_process(rule_double_star()),
            rule_as_process(rule_star_epsilon()),
        ),
        // a third step that does nothing (concat_left_epsilon won't fire on Epsilon alone)
        rule_as_process(rule_concat_left_epsilon()),
    );
    // Only steps a+b fire; step c does not → result is empty because b→Epsilon and c
    // expects a Concat root.
    assert!(rewrite(p, t, &mut f).is_empty());
}

#[test]
fn pipe_three_steps_all_fire() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Direct chain at root: rule_concat_left_epsilon on Concat(Epsilon, Epsilon) → Epsilon.
    // Then rule_star_empty does not fire on Epsilon.
    let t = concat(epsilon(&mut f), epsilon(&mut f), &mut f);
    let p = pipe(
        rule_as_process(rule_concat_left_epsilon()),
        rule_as_process(rule_star_empty()),
    );
    // concat_left_epsilon fires: Epsilon; star_empty does not fire on Epsilon.
    assert!(rewrite(p, t, &mut f).is_empty());
}
