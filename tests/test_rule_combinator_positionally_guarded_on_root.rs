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

//! Unit tests for [`GuardedRule`] wrapping a [`RootRule`].
//!
//! Each test calls `try_apply` directly with an explicit context term and
//! position, which is what the guard inspects.  No traversal machinery is
//! involved.
//!
//! Rule under test: Alt(x, y) → Concat(x, y), but only when Alt is NOT directly
//! under Concat.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::combinators::guard::ClosureRewriteApplicationGuard;
use simple_term_rewriter::rules::combinators::guarded::GuardedRule;
use simple_term_rewriter::rules::primitives::root::RootRule;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;

// == rule factory =============================================================

fn make_rule() -> GuardedRule<RegexOp> {
    GuardedRule::new(
        RootRule::binary(
            "Alt → Concat unless under Concat",
            |op| *op == RegexOp::Alt,
            |_, left, right, f| Some(concat(left.clone(), right.clone(), f)),
        ),
        ClosureRewriteApplicationGuard::new(|_term, ctx, pos| match pos.get_parent_position() {
            None => true,
            Some(parent_pos) => match ctx.get_sub_term_at_position(&parent_pos) {
                None => true,
                Some(parent) => parent.operator != RegexOp::Concat,
            },
        }),
    )
}

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

// == tests =====================================================================

#[test]
fn fires_at_root_when_no_concat_parent() {
    // Alt is the root of the context term — no parent → guard passes.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(epsilon(&mut f), empty(&mut f), &mut f);
    assert_eq!(
        make_rule().try_apply(&t, &t, &root_pos(), &mut f),
        Some(concat(epsilon(&mut f), empty(&mut f), &mut f))
    );
}

#[test]
fn suppressed_when_directly_under_concat() {
    // Alt is child 0 of Concat — parent is Concat → guard blocks.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = concat(
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let alt_node = ctx.sub_terms[0].clone();
    let child_pos = root_pos().get_position_of_nth_child(0);

    assert!(make_rule()
        .try_apply(&alt_node, &ctx, &child_pos, &mut f)
        .is_none());
}

#[test]
fn fires_when_under_alt_not_concat() {
    // Alt nested under another Alt — parent is Alt, not Concat → guard passes.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = alt(
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let inner_alt = ctx.sub_terms[0].clone();
    let child_pos = root_pos().get_position_of_nth_child(0);

    assert_eq!(
        make_rule().try_apply(&inner_alt, &ctx, &child_pos, &mut f),
        Some(concat(epsilon(&mut f), empty(&mut f), &mut f))
    );
}

#[test]
fn does_not_fire_on_non_alt_term() {
    // The inner rule is guarded to Alt only; Concat at root must not fire.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(epsilon(&mut f), empty(&mut f), &mut f);
    assert!(make_rule().try_apply(&t, &t, &root_pos(), &mut f).is_none());
}
