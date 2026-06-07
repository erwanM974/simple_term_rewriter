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
use crate::rule::RewriteRule;
use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

/// Which sibling is visited first when iterating over children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiblingOrder {
    /// Visit children left-to-right (index 0 first).
    Leftmost,
    /// Visit children right-to-left (last index first).
    Rightmost,
}

/// Whether the inner process is tried on a node before or after its subtree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepthOrder {
    /// Try the node directly before descending into its subtree (pre-order).
    /// Finds the shallowest applicable position.
    Outermost,
    /// Descend into the subtree before trying the node directly (post-order).
    /// Finds the deepest applicable position.
    Innermost,
}

/// A composable rewriting strategy.
///
/// Evaluating a `RewriteProcess` on a term via [`run_to_completion`] returns
/// all possible resulting terms reachable by executing the strategy.
/// An empty result means the strategy failed (no rule applied).
///
/// Single-path behaviour (normalization) emerges from strategies built with
/// [`TryOnePath`](RewriteProcess::TryOnePath) only; multi-path behaviour
/// (exploration) emerges from [`TryAllPaths`](RewriteProcess::TryAllPaths).
/// Both are deterministic: the same process on the same term always yields the
/// same result set.
///
/// ## Traversal combinator
///
/// `AnyChild(sibling_order, depth_order, process)` applies `process` to the
/// first child (below the root) where it succeeds, controlling two independent
/// axes via [`SiblingOrder`] and [`DepthOrder`]:
///
/// ```text
///            Leftmost                       Rightmost
/// Outermost  AnyChild(Leftmost, Outermost)  AnyChild(Rightmost, Outermost)
/// Innermost  AnyChild(Leftmost, Innermost)  AnyChild(Rightmost, Innermost)
/// ```
///
/// Combining with [`TryOnePath`](RewriteProcess::TryOnePath):
///
/// ```text
/// outermost_step(r) = TryOnePath([ Rule(r), AnyChild(Leftmost, Outermost, Rule(r)) ])
/// innermost_step(r) = TryOnePath([ AnyChild(Leftmost, Innermost, Rule(r)), Rule(r) ])
/// ```
pub enum RewriteProcess<LOS: RewritableLanguageOperatorSymbol> {
    /// Apply a single rule at the root of the current term.
    /// Returns one result if the rule fires, nothing if it does not.
    Rule(Box<dyn RewriteRule<LOS>>),

    /// Apply the inner process to the first child (below the root) where it
    /// succeeds, according to the given [`SiblingOrder`] and [`DepthOrder`].
    /// Fails if no child admits a successful application.
    AnyChild(SiblingOrder, DepthOrder, Box<Self>),

    /// Sequential composition: apply `a`, then apply `b` to every result of `a`.
    /// Fails if `a` fails.
    Pipe(Box<Self>, Box<Self>),

    /// Apply the inner process repeatedly until it produces no result, then
    /// return the last successful state.  Never fails: if the inner process
    /// never fires, the original term is returned unchanged.
    Repeat(Box<Self>),

    /// Try each alternative in order; return the results of the first one that
    /// succeeds.  Corresponds to `or-else` / `<+` in Maude / Stratego.
    TryOnePath(Vec<Self>),

    /// Try all alternatives; return the union of all their results.
    /// Corresponds to `|` / `+` in Maude / Stratego.
    TryAllPaths(Vec<Self>),
}

// == evaluators ================================================================

/// Evaluate `this` on `term`, looping `Repeat` to fixpoint.
///
/// `context_term` is the root of the full term at the original call site and
/// never changes as we descend, giving context-sensitive rules an accurate view.
pub(crate) fn run_to_completion<LOS: RewritableLanguageOperatorSymbol>(
    this: &RewriteProcess<LOS>,
    term: &LanguageTerm<LOS>,
    context_term: &LanguageTerm<LOS>,
    position: &PositionInLanguageTerm,
    factory: &mut TermFactory<LOS>,
) -> Vec<LanguageTerm<LOS>> {
    match this {
        RewriteProcess::Rule(rule) => rule
            .try_apply(term, context_term, position, factory)
            .into_iter()
            .collect(),

        RewriteProcess::AnyChild(sibling_order, depth_order, process) => {
            let indices: Vec<usize> = match sibling_order {
                SiblingOrder::Leftmost => (0..term.sub_terms.len()).collect(),
                SiblingOrder::Rightmost => (0..term.sub_terms.len()).rev().collect(),
            };
            for n in indices {
                let child = &term.sub_terms[n];
                let child_pos = position.get_position_of_nth_child(n);
                let (first, second) = match depth_order {
                    DepthOrder::Outermost => (
                        run_to_completion(process, child, context_term, &child_pos, factory),
                        run_to_completion(this, child, context_term, &child_pos, factory),
                    ),
                    DepthOrder::Innermost => (
                        run_to_completion(this, child, context_term, &child_pos, factory),
                        run_to_completion(process, child, context_term, &child_pos, factory),
                    ),
                };
                if !first.is_empty() {
                    return first
                        .into_iter()
                        .map(|rw| rebuild_child(term, n, rw, factory))
                        .collect();
                }
                if !second.is_empty() {
                    return second
                        .into_iter()
                        .map(|rw| rebuild_child(term, n, rw, factory))
                        .collect();
                }
            }
            vec![]
        }

        RewriteProcess::Pipe(a, b) => run_to_completion(a, term, context_term, position, factory)
            .into_iter()
            .flat_map(|ti| {
                let new_ctx = replace_at_position(context_term, position, ti.clone(), factory);
                run_to_completion(b, &ti, &new_ctx, position, factory)
            })
            .collect(),

        RewriteProcess::Repeat(process) => {
            let results = run_to_completion(process, term, context_term, position, factory);
            if results.is_empty() {
                vec![term.clone()]
            } else {
                results
                    .into_iter()
                    .flat_map(|ti| {
                        let new_ctx =
                            replace_at_position(context_term, position, ti.clone(), factory);
                        run_to_completion(this, &ti, &new_ctx, position, factory)
                    })
                    .collect()
            }
        }

        RewriteProcess::TryOnePath(processes) => {
            for process in processes {
                let results = run_to_completion(process, term, context_term, position, factory);
                if !results.is_empty() {
                    return results;
                }
            }
            vec![]
        }

        RewriteProcess::TryAllPaths(processes) => processes
            .iter()
            .flat_map(|process| run_to_completion(process, term, context_term, position, factory))
            .collect(),
    }
}

/// Evaluate `this` on `term` for one step, returning the trace alongside each result.
///
/// Differs from [`run_to_completion`] in two ways:
/// - `Repeat` fires its inner process **once** instead of looping to fixpoint;
///   the fixpoint loop is driven externally by repeated calls to `progress`.
/// - Returns, alongside each result term, the ordered list of
///   `(strategy_position, term_position)` pairs for every `Rule` leaf that
///   fired (length > 1 only for `Pipe` chains).
#[allow(clippy::type_complexity)]
pub(crate) fn run_traced_step<LOS: RewritableLanguageOperatorSymbol>(
    this: &RewriteProcess<LOS>,
    term: &LanguageTerm<LOS>,
    context_term: &LanguageTerm<LOS>,
    term_position: &PositionInLanguageTerm,
    strategy_position: &PositionInRewriteProcess,
    factory: &mut TermFactory<LOS>,
) -> Vec<(
    Vec<(PositionInRewriteProcess, PositionInLanguageTerm)>,
    LanguageTerm<LOS>,
)> {
    match this {
        RewriteProcess::Rule(rule) => rule
            .try_apply(term, context_term, term_position, factory)
            .map(|result| {
                (
                    vec![(strategy_position.clone(), term_position.clone())],
                    result,
                )
            })
            .into_iter()
            .collect(),

        RewriteProcess::AnyChild(sibling_order, depth_order, process) => {
            let inner_sp = strategy_position.get_position_of_nth_child(0);
            let indices: Vec<usize> = match sibling_order {
                SiblingOrder::Leftmost => (0..term.sub_terms.len()).collect(),
                SiblingOrder::Rightmost => (0..term.sub_terms.len()).rev().collect(),
            };
            for n in indices {
                let child = &term.sub_terms[n];
                let child_tp = term_position.get_position_of_nth_child(n);
                let (first, second) = match depth_order {
                    DepthOrder::Outermost => (
                        run_traced_step(
                            process,
                            child,
                            context_term,
                            &child_tp,
                            &inner_sp,
                            factory,
                        ),
                        run_traced_step(
                            this,
                            child,
                            context_term,
                            &child_tp,
                            strategy_position,
                            factory,
                        ),
                    ),
                    DepthOrder::Innermost => (
                        run_traced_step(
                            this,
                            child,
                            context_term,
                            &child_tp,
                            strategy_position,
                            factory,
                        ),
                        run_traced_step(
                            process,
                            child,
                            context_term,
                            &child_tp,
                            &inner_sp,
                            factory,
                        ),
                    ),
                };
                if !first.is_empty() {
                    return first
                        .into_iter()
                        .map(|(chain, rw)| (chain, rebuild_child(term, n, rw, factory)))
                        .collect();
                }
                if !second.is_empty() {
                    return second
                        .into_iter()
                        .map(|(chain, rw)| (chain, rebuild_child(term, n, rw, factory)))
                        .collect();
                }
            }
            vec![]
        }

        RewriteProcess::Pipe(a, b) => {
            let sp_a = strategy_position.get_position_of_nth_child(0);
            let sp_b = strategy_position.get_position_of_nth_child(1);
            run_traced_step(a, term, context_term, term_position, &sp_a, factory)
                .into_iter()
                .flat_map(|(chain_a, ti)| {
                    let new_ctx =
                        replace_at_position(context_term, term_position, ti.clone(), factory);
                    run_traced_step(b, &ti, &new_ctx, term_position, &sp_b, factory)
                        .into_iter()
                        .map(move |(chain_b, result)| {
                            let mut full_chain = chain_a.clone();
                            full_chain.extend(chain_b);
                            (full_chain, result)
                        })
                })
                .collect()
        }

        RewriteProcess::Repeat(process) => run_traced_step(
            process,
            term,
            context_term,
            term_position,
            &strategy_position.get_position_of_nth_child(0),
            factory,
        ),

        RewriteProcess::TryOnePath(processes) => {
            for (i, process) in processes.iter().enumerate() {
                let sp_i = strategy_position.get_position_of_nth_child(i);
                let results =
                    run_traced_step(process, term, context_term, term_position, &sp_i, factory);
                if !results.is_empty() {
                    return results;
                }
            }
            vec![]
        }

        RewriteProcess::TryAllPaths(processes) => processes
            .iter()
            .enumerate()
            .flat_map(|(i, process)| {
                let sp_i = strategy_position.get_position_of_nth_child(i);
                run_traced_step(process, term, context_term, term_position, &sp_i, factory)
            })
            .collect(),
    }
}

// == helpers ===================================================================

fn rebuild_child<LOS: RewritableLanguageOperatorSymbol>(
    parent: &LanguageTerm<LOS>,
    child_index: usize,
    new_child: LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> LanguageTerm<LOS> {
    let mut new_sub_terms = parent.sub_terms.clone();
    new_sub_terms[child_index] = new_child;
    LanguageTermNode::build(parent.operator.clone(), new_sub_terms, factory)
}

/// Rebuild `context` with `replacement` substituted at `position`.
/// Used by `Pipe` and `Repeat` to keep the context current after a rule fires.
fn replace_at_position<LOS: RewritableLanguageOperatorSymbol>(
    context: &LanguageTerm<LOS>,
    position: &PositionInLanguageTerm,
    replacement: LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> LanguageTerm<LOS> {
    replace_rec(
        context,
        position.get_absolute_coordinates_from_root(),
        replacement,
        factory,
    )
}

fn replace_rec<LOS: RewritableLanguageOperatorSymbol>(
    term: &LanguageTerm<LOS>,
    coords: &[usize],
    replacement: LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> LanguageTerm<LOS> {
    match coords.first() {
        None => replacement,
        Some(&n) => {
            let new_child = replace_rec(&term.sub_terms[n], &coords[1..], replacement, factory);
            rebuild_child(term, n, new_child, factory)
        }
    }
}
