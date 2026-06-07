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

//! Consistency tests: [`RewriteProcessTracedExecutor`] vs [`RewriteProcessUntracedExecutor`]
//! on randomly generated regex terms.
//!
//! For each randomly generated term the same normalisation strategy is applied
//! by both executors.  The test asserts that the set of normal forms produced
//! by looping `progress` to completion equals the set returned directly by
//! `RewriteProcessUntracedExecutor::rewrite`.
//!
//! The strategy used is `normalization_strategy()` from `common::regex::rules`:
//! outermost single-step repeated to fixpoint over all ten built-in rules.
//!
//! All terms and all rewriting sessions share a single [`TermFactory`] so that
//! hash-consing uid comparisons (used by e.g. `rule_alt_idempotent`) remain
//! valid across generation and rewriting.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::process::traced::RewriteProcessTracedExecutor;
use simple_term_rewriter::process::untraced::RewriteProcessUntracedExecutor;
use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

use crate::common::regex::generation::generate_regex_terms;
use crate::common::regex::lang::RegexOp;
use crate::common::regex::rules::*;

// == comparison helpers ====================================================

/// Run the traced executor to completion.
///
/// Returns the sorted Debug strings of the completed terms together with the
/// factory so the caller can reuse it for subsequent sessions.
fn traced_normal_forms(
    term: LanguageTerm<RegexOp>,
    factory: TermFactory<RegexOp>,
) -> (Vec<String>, TermFactory<RegexOp>) {
    let mut executor = RewriteProcessTracedExecutor::new(normalization_strategy(), term, factory);
    while !executor.get_current_terms().is_empty() {
        executor.progress();
    }
    let mut forms: Vec<String> = executor
        .get_completed_terms()
        .iter()
        .map(|t| format!("{t:?}"))
        .collect();
    forms.sort();
    let factory = executor.into_factory();
    (forms, factory)
}

/// Run the untraced executor to fixpoint and return the sorted Debug strings.
fn untraced_normal_forms(
    term: &LanguageTerm<RegexOp>,
    factory: &mut TermFactory<RegexOp>,
) -> Vec<String> {
    let mut forms: Vec<String> =
        RewriteProcessUntracedExecutor::rewrite(&normalization_strategy(), term, factory)
            .into_iter()
            .map(|t| format!("{t:?}"))
            .collect();
    forms.sort();
    forms
}

// == tests =================================================================

/// Traced and untraced executors must agree on 200 random terms (seed 0).
#[test]
fn traced_and_untraced_agree_seed_0() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    for term in generate_regex_terms(200, 0, &mut f) {
        let expected = untraced_normal_forms(&term, &mut f);
        let (got, returned_f) = traced_normal_forms(term, f);
        f = returned_f;
        assert_eq!(expected, got);
    }
}

/// Traced and untraced executors must agree on 200 random terms (seed 1).
#[test]
fn traced_and_untraced_agree_seed_1() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    for term in generate_regex_terms(200, 1, &mut f) {
        let expected = untraced_normal_forms(&term, &mut f);
        let (got, returned_f) = traced_normal_forms(term, f);
        f = returned_f;
        assert_eq!(expected, got);
    }
}

/// The normalisation strategy is single-path (`TryOnePath` throughout), so
/// every input term must reduce to exactly one normal form.
#[test]
fn traced_produces_exactly_one_normal_form_per_term() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    for term in generate_regex_terms(200, 42, &mut f) {
        let (forms, returned_f) = traced_normal_forms(term, f);
        f = returned_f;
        assert_eq!(
            forms.len(),
            1,
            "expected exactly one normal form, got {:?}",
            forms
        );
    }
}
