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

//! Tests for [`RewriteProcess::AnyChild`].

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::{DepthOrder, RewriteProcess, SiblingOrder};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

fn any_child(
    so: SiblingOrder,
    d: DepthOrder,
    inner: RewriteProcess<RegexOp>,
) -> RewriteProcess<RegexOp> {
    RewriteProcess::AnyChild(so, d, Box::new(inner))
}

fn rule_double() -> RewriteProcess<RegexOp> {
    rule_as_process(rule_double_star())
}

// == leaf term (no children) ===================================================

#[test]
fn anychild_on_leaf_is_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Atom(a) has no children; AnyChild must always return empty.
    let t = atom(b'a', &mut f);
    for so in [SiblingOrder::Leftmost, SiblingOrder::Rightmost] {
        for d in [DepthOrder::Outermost, DepthOrder::Innermost] {
            let result = rewrite(any_child(so, d, rule_double()), t.clone(), &mut f);
            assert!(
                result.is_empty(),
                "expected empty for leaf with {so:?}/{d:?}"
            );
        }
    }
}

#[test]
fn anychild_on_term_with_no_applicable_child_is_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Concat(Atom(a), Atom(b)): neither child matches rule_double_star.
    let t = concat(atom(b'a', &mut f), atom(b'b', &mut f), &mut f);
    let result = rewrite(
        any_child(SiblingOrder::Leftmost, DepthOrder::Outermost, rule_double()),
        t,
        &mut f,
    );
    assert!(result.is_empty());
}

// == sibling order =============================================================

#[test]
fn leftmost_picks_first_child() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Alt(Star(Star(a)), Star(Star(b))): both children match double_star.
    // Leftmost must pick the first child → result replaces child 0.
    let t = alt(
        star(star(atom(b'a', &mut f), &mut f), &mut f),
        star(star(atom(b'b', &mut f), &mut f), &mut f),
        &mut f,
    );
    let result = rewrite(
        any_child(SiblingOrder::Leftmost, DepthOrder::Outermost, rule_double()),
        t,
        &mut f,
    );
    assert_eq!(
        result,
        vec![alt(
            star(atom(b'a', &mut f), &mut f),
            star(star(atom(b'b', &mut f), &mut f), &mut f),
            &mut f
        )]
    );
}

#[test]
fn rightmost_picks_last_child() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Same term, but Rightmost must pick the second child → result replaces child 1.
    let t = alt(
        star(star(atom(b'a', &mut f), &mut f), &mut f),
        star(star(atom(b'b', &mut f), &mut f), &mut f),
        &mut f,
    );
    let result = rewrite(
        any_child(
            SiblingOrder::Rightmost,
            DepthOrder::Outermost,
            rule_double(),
        ),
        t,
        &mut f,
    );
    assert_eq!(
        result,
        vec![alt(
            star(star(atom(b'a', &mut f), &mut f), &mut f),
            star(atom(b'b', &mut f), &mut f),
            &mut f
        )]
    );
}

// == depth order: outermost vs innermost =======================================

#[test]
fn outermost_fires_at_shallower_position() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let result = rewrite(
        any_child(SiblingOrder::Leftmost, DepthOrder::Outermost, rule_double()),
        t,
        &mut f,
    );
    assert_eq!(
        result,
        vec![concat(
            star(star(atom(b'a', &mut f), &mut f), &mut f),
            epsilon(&mut f),
            &mut f
        )]
    );
}

#[test]
fn innermost_fires_at_deeper_position() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let result = rewrite(
        any_child(SiblingOrder::Leftmost, DepthOrder::Innermost, rule_double()),
        t,
        &mut f,
    );
    assert_eq!(
        result,
        vec![concat(
            star(star(atom(b'a', &mut f), &mut f), &mut f),
            epsilon(&mut f),
            &mut f
        )]
    );
}

// == genuine outermost vs innermost divergence =================================

#[test]
fn outermost_and_innermost_diverge() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // Term: Concat(Star(Star(∅)), Atom(a))
    // Inner process: TryOnePath([rule_double_star, rule_star_empty])
    //
    // Outermost: Star(Star(∅)) → Star(∅) via double_star.
    // Innermost: recurses into Star(∅) → ε via star_empty, giving Star(ε).
    let t = concat(
        star(star(empty(&mut f), &mut f), &mut f),
        atom(b'a', &mut f),
        &mut f,
    );

    let outermost_inner = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    ]);
    let outermost = rewrite(
        any_child(
            SiblingOrder::Leftmost,
            DepthOrder::Outermost,
            outermost_inner,
        ),
        t.clone(),
        &mut f,
    );

    let innermost_inner = RewriteProcess::TryOnePath(vec![
        rule_as_process(rule_double_star()),
        rule_as_process(rule_star_empty()),
    ]);
    let innermost = rewrite(
        any_child(
            SiblingOrder::Leftmost,
            DepthOrder::Innermost,
            innermost_inner,
        ),
        t,
        &mut f,
    );

    assert_eq!(
        outermost,
        vec![concat(
            star(empty(&mut f), &mut f),
            atom(b'a', &mut f),
            &mut f
        )]
    );
    assert_eq!(
        innermost,
        vec![concat(
            star(epsilon(&mut f), &mut f),
            atom(b'a', &mut f),
            &mut f
        )]
    );
}

// == outermost: second branch

#[test]
fn outermost_second_branch_when_rule_fires_two_levels_deep() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        atom(b'a', &mut f),
        concat(
            atom(b'b', &mut f),
            star(star(atom(b'c', &mut f), &mut f), &mut f),
            &mut f,
        ),
        &mut f,
    );
    let result = rewrite(
        any_child(SiblingOrder::Leftmost, DepthOrder::Outermost, rule_double()),
        t,
        &mut f,
    );
    assert_eq!(
        result,
        vec![concat(
            atom(b'a', &mut f),
            concat(atom(b'b', &mut f), star(atom(b'c', &mut f), &mut f), &mut f),
            &mut f
        )]
    );
}
