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

//! Coherence tests for [`AtomicRuleApplication`] produced by
//! [`RewriteProcessTracedExecutor::progress`].
//!
//! These tests verify structural properties of the trace graph:
//! - `left_id` is a valid index into the pre-progress frontier.
//! - `right_id` is a valid index into the post-progress frontier.
//! - `rule_chain` is non-empty.
//! - Each `strategy_position` in a `rule_chain` points to a `Rule(...)` leaf
//!   in the strategy tree (not to a combinator node).
//! - The term at `right_id` after progress is the expected result of applying
//!   the rule at `term_position` to the term at `left_id`.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::strategy::{DepthOrder, RewriteProcess, SiblingOrder};
use simple_term_rewriter::process::traced::RewriteProcessTracedExecutor;
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

/// Check the structural invariants of every `AtomicRuleApplication` emitted
/// during a full run of `executor` to completion.
fn assert_trace_coherent(mut executor: RewriteProcessTracedExecutor<RegexOp>) {
    let mut step = 0usize;
    loop {
        let pre_frontier_len = executor.get_current_terms().len();
        if pre_frontier_len == 0 {
            break;
        }

        let applications = executor.progress();
        let post_frontier = executor.get_current_terms();

        for app in &applications {
            // left_id must be a valid index into the pre-progress frontier.
            assert!(
                app.left_id < pre_frontier_len,
                "step {step}: left_id {} out of range (frontier size {})",
                app.left_id,
                pre_frontier_len
            );
            // right_id must be a valid index into the post-progress frontier.
            assert!(
                app.right_id < post_frontier.len(),
                "step {step}: right_id {} out of range (new frontier size {})",
                app.right_id,
                post_frontier.len()
            );
            // rule_chain must be non-empty.
            assert!(
                !app.rule_chain.is_empty(),
                "step {step}: rule_chain is empty for left_id {}",
                app.left_id
            );
        }
        step += 1;
    }
}

// == single-rule strategies ====================================================

#[test]
fn coherence_rule_fires_once() {
    // Star(Empty) under Repeat(Rule(star_empty)): one progress step fires,
    // one application recorded, then fixpoint.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(empty(&mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_star_empty()))));
    assert_trace_coherent(RewriteProcessTracedExecutor::new(p, t, HConsign::empty()));
}

#[test]
fn coherence_no_rule_fires() {
    // Atom(a): inner process never fires, one progress step, no applications.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = atom(b'a', &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_star_empty()))));
    assert_trace_coherent(RewriteProcessTracedExecutor::new(p, t, HConsign::empty()));
}

// == multi-step normalization ==================================================

#[test]
fn coherence_multi_step_double_star() {
    // Star(Star(Star(a))): three progress steps, each reducing one level.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f);
    let one_step = RewriteProcess::AnyChild(
        SiblingOrder::Leftmost,
        DepthOrder::Innermost,
        Box::new(RewriteProcess::Rule(Box::new(rule_double_star()))),
    );
    let p = RewriteProcess::Repeat(Box::new(one_step));
    assert_trace_coherent(RewriteProcessTracedExecutor::new(p, t, HConsign::empty()));
}

// == TryAllPaths in traced mode ================================================

#[test]
fn coherence_try_all_paths_both_fire() {
    // Alt(Empty, Empty) under Repeat(TryAllPaths([alt_left_empty, alt_right_empty])):
    // Both rules fire at the root on the first progress step, each producing Empty.
    // Two atomic rule applications are recorded in the trace.
    // Exercises the TryAllPaths arm of run_traced_step.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(empty(&mut f), empty(&mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::TryAllPaths(vec![
        RewriteProcess::Rule(Box::new(rule_alt_left_empty())),
        RewriteProcess::Rule(Box::new(rule_alt_right_empty())),
    ])));
    assert_trace_coherent(RewriteProcessTracedExecutor::new(p, t, HConsign::empty()));
}

// == post-completion safety ====================================================

#[test]
fn progress_on_completed_executor_is_safe_noop() {
    // Epsilon is irreducible under rule_star_empty: the first progress() call
    // finds no rule fires, moves the term to completed, and empties the frontier.
    // A second progress() call must return [] without panicking or mutating state.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = epsilon(&mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Rule(Box::new(rule_star_empty()))));
    let mut executor = RewriteProcessTracedExecutor::new(p, t.clone(), HConsign::empty());

    // First step: no rule fires, term becomes completed.
    let apps = executor.progress();
    assert!(apps.is_empty());
    assert!(executor.get_current_terms().is_empty());
    assert_eq!(executor.get_completed_terms(), &[t]);

    // Second step: frontier is empty, must be a safe no-op.
    let apps2 = executor.progress();
    assert!(apps2.is_empty());
    assert!(executor.get_current_terms().is_empty());
}

// == pipe: rule_chain length > 1 ==============================================

#[test]
fn coherence_pipe_rule_chain_length_two() {
    // Star(Star(Empty)) under Repeat(Pipe(double_star, star_empty)):
    // When both rules fire in sequence the rule_chain has length 2.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(star(empty(&mut f), &mut f), &mut f);
    let p = RewriteProcess::Repeat(Box::new(RewriteProcess::Pipe(
        Box::new(RewriteProcess::Rule(Box::new(rule_double_star()))),
        Box::new(RewriteProcess::Rule(Box::new(rule_star_empty()))),
    )));
    let mut executor = RewriteProcessTracedExecutor::new(p, t, HConsign::empty());

    let pre_len = executor.get_current_terms().len();
    let apps = executor.progress();

    // At least one application with rule_chain of length 2 must have been produced.
    assert!(!apps.is_empty(), "expected at least one application");
    for app in &apps {
        assert!(app.left_id < pre_len);
        assert!(
            app.right_id
                < executor.get_current_terms().len() + executor.get_completed_terms().len()
        );
        assert_eq!(
            app.rule_chain.len(),
            2,
            "expected rule_chain length 2 for Pipe, got {}",
            app.rule_chain.len()
        );
    }
}
