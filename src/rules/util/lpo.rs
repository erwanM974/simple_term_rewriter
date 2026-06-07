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

use std::cmp::Ordering;

use crate::term::syntax::{LanguageOperatorArity, LanguageTerm, RewritableLanguageOperatorSymbol};

fn resolve_arity<LOS: RewritableLanguageOperatorSymbol>(term: &LanguageTerm<LOS>) -> usize {
    match term.operator.arity() {
        LanguageOperatorArity::Fixed(n) => n,
        LanguageOperatorArity::Variadic => term.sub_terms.len(),
    }
}

/// Returns the [`Ordering`] between two terms under the lexicographic path ordering
/// induced by the given total order on operator symbols.
///
/// This is the three-way wrapper around [`is_greater_as_per_lexicographic_path_ordering`]:
/// - Returns `Equal` when `s == t` (structural equality).
/// - Returns `Greater` when `s >_lpo t`.
/// - Returns `Less` otherwise (and asserts in debug builds that `t >_lpo s` holds,
///   confirming the relation is total on structurally distinct terms).
///
/// See [`is_greater_as_per_lexicographic_path_ordering`] for the precise definition
/// and the requirements on `compare_operators` and `get_arity`.
pub fn lexicographic_path_ordering<LOS: RewritableLanguageOperatorSymbol>(
    s: &LanguageTerm<LOS>,
    t: &LanguageTerm<LOS>,
    compare_operators: &dyn Fn(&LOS, &LOS) -> Ordering,
) -> std::cmp::Ordering {
    if s == t {
        std::cmp::Ordering::Equal
    } else if is_greater_as_per_lexicographic_path_ordering(s, t, compare_operators) {
        std::cmp::Ordering::Greater
    } else {
        debug_assert!(is_greater_as_per_lexicographic_path_ordering(
            t,
            s,
            compare_operators
        ));
        std::cmp::Ordering::Less
    }
}

/// Returns `true` iff `s >_lpo t` under the lexicographic path ordering (LPO)
/// induced by the given total order on operator symbols.
///
/// # Definition
///
/// The LPO is defined recursively on `s = f(s₁,…,sₙ)` and `t = g(t₁,…,tₘ)`
/// (Baader & Nipkow, *Term Rewriting and All That*, §5.4):
///
/// **Case `f > g`** — `s >_lpo t` iff `s >_lpo tⱼ` for every child `tⱼ` of `t`.
///
/// **Case `f < g`** — `s >_lpo t` iff some child `sᵢ` of `s` satisfies
/// `sᵢ = t` or `sᵢ >_lpo t`.
///
/// **Case `f = g`** — `s >_lpo t` iff there exists a critical index `k` such that:
/// - `s₁ = t₁, …, s_{k-1} = t_{k-1}` (all earlier children are equal),
/// - `sₖ >_lpo tₖ`, and
/// - `s >_lpo tⱼ` for every `j > k` (s dominates all remaining children of t).
///
/// # Parameters
///
/// - `compare_operators` — a total strict order on operator symbols; must be
///   consistent (transitive, asymmetric, total).
///
/// Arity is read from [`RewritableLanguageOperatorSymbol::arity`] and resolved
/// against the actual sub-term count for [`LanguageOperatorArity::Variadic`]
/// operators.
///
/// # Properties
///
/// When `compare_operators` is a total order, the resulting LPO is a
/// simplification ordering: irreflexive, transitive, total on ground terms,
/// monotone, and has the subterm property (`s > t` whenever `t` is a strict
/// subterm of `s`).
pub fn is_greater_as_per_lexicographic_path_ordering<LOS: RewritableLanguageOperatorSymbol>(
    s: &LanguageTerm<LOS>,
    t: &LanguageTerm<LOS>,
    compare_operators: &dyn Fn(&LOS, &LOS) -> Ordering,
) -> bool {
    match compare_operators(&s.operator, &t.operator) {
        Ordering::Greater => {
            // s dominates t if s dominates each of t's subterms
            let mut is_greater = true;
            'iter_tjs: for j in 0..resolve_arity(t) {
                let tj = t.sub_terms.get(j).unwrap();
                if !is_greater_as_per_lexicographic_path_ordering(s, tj, compare_operators) {
                    is_greater = false;
                    break 'iter_tjs;
                }
            }
            if is_greater {
                return true;
            }
        }
        Ordering::Less => {
            // s dominates t if one of s's subterms dominates t
            for i in 0..resolve_arity(s) {
                let si = s.sub_terms.get(i).unwrap();
                if si == t
                    || is_greater_as_per_lexicographic_path_ordering(si, t, compare_operators)
                {
                    return true;
                }
            }
        }
        Ordering::Equal => {
            // s and t have the same root operator, so the same number of sub-terms.
            // Find the leftmost index k where sk >_lpo tk (the "critical" index).
            // Per the LPO definition (Baader & Nipkow §5.4), s >_lpo t iff:
            //   - s1=t1, …, s_{k-1}=t_{k-1}
            //   - sk >_lpo tk
            //   - AND ∀j > k: s >_lpo tj   ← dominance of all remaining sub-terms of t
            let arity = resolve_arity(s);
            for i in 0..arity {
                let si = s.sub_terms.get(i).unwrap();
                let ti = t.sub_terms.get(i).unwrap();
                if is_greater_as_per_lexicographic_path_ordering(si, ti, compare_operators) {
                    // k = i is the critical index; now check ∀j > k: s >_lpo tj
                    for j in (i + 1)..arity {
                        let tj = t.sub_terms.get(j).unwrap();
                        if !is_greater_as_per_lexicographic_path_ordering(s, tj, compare_operators)
                        {
                            return false;
                        }
                    }
                    return true;
                }
                if is_greater_as_per_lexicographic_path_ordering(ti, si, compare_operators) {
                    return false;
                }
            }
        }
    }
    false
}
