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

use crate::position::{PositionInLanguageTerm, PositionInRewriteProcess};
use crate::process::strategy::{run_traced_step, RewriteProcess};
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// Records one atomic transition produced by a single [`RewriteProcessTracedExecutor::progress`] call.
///
/// `left_id` and `right_id` index into the term slices returned by
/// [`RewriteProcessTracedExecutor::get_current_terms`] immediately **before** and **after**
/// the `progress` call respectively.
///
/// `rule_chain` is the ordered sequence of `Rule` leaves that fired to produce
/// this transition.  For strategies that do not involve `Pipe` it always has
/// length 1.  For `Pipe(a, b)` it has length 2 (first `a`'s rule, then `b`'s
/// rule); deeper nesting produces longer chains.  Each entry records:
/// - which `Rule(...)` leaf in the [`RewriteProcess`] tree fired
///   (`PositionInRewriteProcess`),
/// - where in the term it fired (`PositionInLanguageTerm`).
pub struct AtomicRuleApplication {
    /// Index of the source term in the **pre-progress** frontier.
    pub left_id: usize,
    /// Ordered list of `(strategy position, term position)` for every rule that
    /// fired in sequence to produce this transition.
    pub rule_chain: Vec<(PositionInRewriteProcess, PositionInLanguageTerm)>,
    /// Index of the result term in the **post-progress** frontier.
    pub right_id: usize,
}

/// Stateful, step-by-step evaluator for a [`RewriteProcess`].
///
/// The frontier of "live" terms is advanced one layer at a time via
/// [`progress`](Self::progress).  Each call applies the strategy to every
/// current term simultaneously, replaces the frontier with the union of all
/// results, and returns the full trace of what happened.
///
/// Terms for which the strategy produces no result have reached a fixpoint and
/// are moved to the **completed** set (accessible via
/// [`get_completed_terms`](Self::get_completed_terms)).
///
/// The process is finished when [`get_current_terms`](Self::get_current_terms)
/// returns an empty slice.
pub struct RewriteProcessTracedExecutor<LOS: RewritableLanguageOperatorSymbol> {
    strategy: RewriteProcess<LOS>,
    factory: TermFactory<LOS>,
    current_terms: Vec<LanguageTerm<LOS>>,
    completed_terms: Vec<LanguageTerm<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteProcessTracedExecutor<LOS> {
    /// Creates an executor that will apply `strategy` starting from `initial`.
    pub fn new(
        strategy: RewriteProcess<LOS>,
        initial: LanguageTerm<LOS>,
        factory: TermFactory<LOS>,
    ) -> Self {
        Self {
            strategy,
            factory,
            current_terms: vec![initial],
            completed_terms: vec![],
        }
    }

    /// Consumes the executor and returns its factory.
    ///
    /// Call this after the executor has run to completion to reclaim the
    /// factory and reuse it for subsequent rewriting sessions, ensuring all
    /// terms remain in the same hash-consing universe.
    pub fn into_factory(self) -> TermFactory<LOS> {
        self.factory
    }

    /// Terms still being rewritten (not yet at a fixpoint).
    pub fn get_current_terms(&self) -> &[LanguageTerm<LOS>] {
        &self.current_terms
    }

    /// Terms that have reached a fixpoint (the strategy produced no further
    /// result for them).  For a `Repeat`-based normalization strategy these are
    /// the normal forms.
    pub fn get_completed_terms(&self) -> &[LanguageTerm<LOS>] {
        &self.completed_terms
    }

    /// Advance every current term by one strategy step.
    ///
    /// For each term in the frontier the strategy is applied once:
    /// - Terms that produce at least one result are replaced by those results
    ///   in the new frontier.
    /// - Terms that produce no result have reached a fixpoint: they are moved
    ///   to [`get_completed_terms`](Self::get_completed_terms) and removed from
    ///   the frontier.
    ///
    /// Returns the full trace of every atomic rule application that occurred,
    /// with `left_id` / `right_id` indexing into the old / new frontiers.
    /// An empty return value together with an empty
    /// [`get_current_terms`](Self::get_current_terms) means all terms are done.
    pub fn progress(&mut self) -> Vec<AtomicRuleApplication> {
        let root_sp = PositionInRewriteProcess::get_root_position();
        let root_tp = PositionInLanguageTerm::get_root_position();

        let mut next_terms: Vec<LanguageTerm<LOS>> = Vec::new();
        let mut applications: Vec<AtomicRuleApplication> = Vec::new();

        for (left_id, term) in self.current_terms.iter().enumerate() {
            let results = run_traced_step(
                &self.strategy,
                term,
                term,
                &root_tp,
                &root_sp,
                &mut self.factory,
            );
            if results.is_empty() {
                self.completed_terms.push(term.clone());
            } else {
                for (rule_chain, result) in results {
                    let right_id = next_terms.len();
                    next_terms.push(result);
                    applications.push(AtomicRuleApplication {
                        left_id,
                        rule_chain,
                        right_id,
                    });
                }
            }
        }

        self.current_terms = next_terms;
        applications
    }
}
