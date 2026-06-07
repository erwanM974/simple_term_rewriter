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

//! Tests for all [`RewriteApplicationGuard`] implementations.
//!
//! Each guard is exercised directly via `.allows(term, context, position)`
//! without involving any rule or rule combinator.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::rules::combinators::guard::{
    ClosureRewriteApplicationGuard, NotUnderSameOpRewriteApplicationGuard,
    OnlyUnderOpRewriteApplicationGuard, RewriteApplicationGuard, RootOnlyRewriteApplicationGuard,
    TermPredicateRewriteApplicationGuard,
};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;

// == position helpers ==========================================================

fn root() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

fn child(n: usize) -> PositionInLanguageTerm {
    root().get_position_of_nth_child(n)
}

// == RootOnlyRewriteApplicationGuard ==========================================
//
// Allows only when position depth == 0; term and context are ignored.

#[test]
fn root_only_allows_at_root() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = RootOnlyRewriteApplicationGuard;
    let t = epsilon(&mut f);
    assert!(g.allows(&t, &t, &root()));
}

#[test]
fn root_only_blocks_at_depth_one() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = RootOnlyRewriteApplicationGuard;
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let node = ctx.sub_terms[0].clone();
    assert!(!g.allows(&node, &ctx, &child(0)));
}

#[test]
fn root_only_blocks_at_depth_two() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = RootOnlyRewriteApplicationGuard;
    let ctx = concat(star(epsilon(&mut f), &mut f), empty(&mut f), &mut f);
    let node = ctx.sub_terms[0].sub_terms[0].clone();
    let pos = child(0).get_position_of_nth_child(0);
    assert!(!g.allows(&node, &ctx, &pos));
}

// == NotUnderSameOpRewriteApplicationGuard ====================================
//
// At root (no parent) → true.
// Parent operator differs from term's root operator → true.
// Parent operator equals term's root operator → false.
// Parent position resolves to None in context (inconsistent args) → true.

#[test]
fn not_under_same_op_allows_at_root() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = NotUnderSameOpRewriteApplicationGuard;
    let t = concat(epsilon(&mut f), empty(&mut f), &mut f);
    assert!(g.allows(&t, &t, &root()));
}

#[test]
fn not_under_same_op_allows_when_parent_differs() {
    // Concat(Alt(Epsilon, Empty), Epsilon): Alt at child(0), parent is Concat → different → allows
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = NotUnderSameOpRewriteApplicationGuard;
    let ctx = concat(
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let node = ctx.sub_terms[0].clone();
    assert!(g.allows(&node, &ctx, &child(0)));
}

#[test]
fn not_under_same_op_blocks_when_parent_is_same() {
    // Concat(Concat(Epsilon, Empty), Epsilon): inner Concat at child(0), parent is Concat → same → blocks
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = NotUnderSameOpRewriteApplicationGuard;
    let ctx = concat(
        concat(epsilon(&mut f), empty(&mut f), &mut f),
        epsilon(&mut f),
        &mut f,
    );
    let node = ctx.sub_terms[0].clone();
    assert!(!g.allows(&node, &ctx, &child(0)));
}

#[test]
fn not_under_same_op_allows_when_parent_lookup_returns_none() {
    // Position says the node is at child(5), but context only has 2 children.
    // parent_pos = root, context.get_sub_term_at_position(child(5)) = None → true.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = NotUnderSameOpRewriteApplicationGuard;
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let t = epsilon(&mut f);
    // child(5) of child(5): parent_pos = child(5), which doesn't exist in ctx
    let pos = child(5).get_position_of_nth_child(0);
    assert!(g.allows(&t, &ctx, &pos));
}

// == OnlyUnderOpRewriteApplicationGuard =======================================
//
// At root (no parent) → false.
// Parent operator satisfies predicate → true.
// Parent operator does not satisfy predicate → false.
// Parent position resolves to None in context (inconsistent args) → false.

fn only_under_concat() -> OnlyUnderOpRewriteApplicationGuard<RegexOp> {
    OnlyUnderOpRewriteApplicationGuard::new(|op: &RegexOp| *op == RegexOp::Concat)
}

#[test]
fn only_under_op_blocks_at_root() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    assert!(!only_under_concat().allows(&ctx, &ctx, &root()));
}

#[test]
fn only_under_op_allows_when_parent_matches_predicate() {
    // Concat(Epsilon, Empty): Epsilon at child(0), parent is Concat → predicate holds → allows
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let node = ctx.sub_terms[0].clone();
    assert!(only_under_concat().allows(&node, &ctx, &child(0)));
}

#[test]
fn only_under_op_blocks_when_parent_does_not_match_predicate() {
    // Alt(Epsilon, Empty): Epsilon at child(0), parent is Alt → predicate fails → blocks
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = alt(epsilon(&mut f), empty(&mut f), &mut f);
    let node = ctx.sub_terms[0].clone();
    assert!(!only_under_concat().allows(&node, &ctx, &child(0)));
}

#[test]
fn only_under_op_blocks_when_parent_lookup_returns_none() {
    // Inconsistent position: parent_pos = child(5) doesn't exist in context → false.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let t = epsilon(&mut f);
    let pos = child(5).get_position_of_nth_child(0);
    assert!(!only_under_concat().allows(&t, &ctx, &pos));
}

// == TermPredicateRewriteApplicationGuard =====================================
//
// Only the term is inspected; context and position are irrelevant.

#[test]
fn term_predicate_allows_when_predicate_holds() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g =
        TermPredicateRewriteApplicationGuard::<RegexOp>::new(|t| t.operator == RegexOp::Epsilon);
    let t = epsilon(&mut f);
    let ctx = concat(empty(&mut f), empty(&mut f), &mut f);
    // Different context and an arbitrary position — neither should matter.
    assert!(g.allows(&t, &ctx, &child(3)));
}

#[test]
fn term_predicate_blocks_when_predicate_fails() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g =
        TermPredicateRewriteApplicationGuard::<RegexOp>::new(|t| t.operator == RegexOp::Epsilon);
    let t = empty(&mut f);
    assert!(!g.allows(&t, &t, &root()));
}

#[test]
fn term_predicate_ignores_context_and_position() {
    // Same predicate, same term, different context and position → same outcome.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = TermPredicateRewriteApplicationGuard::<RegexOp>::new(|t| t.operator == RegexOp::Star);
    let t = star(epsilon(&mut f), &mut f);
    let ctx = concat(empty(&mut f), empty(&mut f), &mut f);
    assert!(g.allows(&t, &ctx, &root()));
    assert!(g.allows(&t, &ctx, &child(7)));
    assert!(g.allows(&t, &t, &child(0).get_position_of_nth_child(2)));
}

// == ClosureRewriteApplicationGuard ===========================================

#[test]
fn closure_guard_returns_true_when_closure_returns_true() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = ClosureRewriteApplicationGuard::<RegexOp>::new(|_, _, _| true);
    let t = epsilon(&mut f);
    assert!(g.allows(&t, &t, &root()));
}

#[test]
fn closure_guard_returns_false_when_closure_returns_false() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = ClosureRewriteApplicationGuard::<RegexOp>::new(|_, _, _| false);
    let t = epsilon(&mut f);
    assert!(!g.allows(&t, &t, &root()));
}

#[test]
fn closure_guard_receives_all_three_arguments() {
    // The closure checks a property of each of the three arguments:
    // - term has operator Epsilon
    // - context has operator Concat at the root
    // - position is at depth 1
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let g = ClosureRewriteApplicationGuard::<RegexOp>::new(|term, ctx, pos| {
        term.operator == RegexOp::Epsilon && ctx.operator == RegexOp::Concat && pos.get_depth() == 1
    });
    let ctx = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let node = ctx.sub_terms[0].clone(); // Epsilon at child(0)
    assert!(g.allows(&node, &ctx, &child(0)));
    assert!(!g.allows(&node, &ctx, &root())); // depth 0, not 1
    assert!(!g.allows(&ctx.sub_terms[1].clone(), &ctx, &child(1))); // Empty, not Epsilon
}
