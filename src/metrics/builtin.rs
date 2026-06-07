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

use std::collections::{HashMap, HashSet};

use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol};

/// Returns the number of nodes in the term tree (the root counts as 1).
pub fn tree_size<LOS: RewritableLanguageOperatorSymbol>(term: &LanguageTerm<LOS>) -> usize {
    1 + term.sub_terms.iter().map(tree_size).sum::<usize>()
}

/// Returns the number of structurally distinct sub-terms (including the root).
///
/// Two sub-terms are identical when they have the same operator and the same
/// children recursively.  This is the size of the maximally-shared DAG that
/// the term would occupy under hash-consing.
pub fn dag_size<LOS: RewritableLanguageOperatorSymbol>(term: &LanguageTerm<LOS>) -> usize {
    let mut seen: HashSet<&LanguageTerm<LOS>> = HashSet::new();
    collect_distinct(term, &mut seen);
    seen.len()
}

fn collect_distinct<'a, LOS: RewritableLanguageOperatorSymbol>(
    term: &'a LanguageTerm<LOS>,
    seen: &mut HashSet<&'a LanguageTerm<LOS>>,
) {
    if seen.insert(term) {
        for sub in &term.sub_terms {
            collect_distinct(sub, seen);
        }
    }
}

/// Returns the depth of the term tree (a single leaf has depth 1).
pub fn term_depth<LOS: RewritableLanguageOperatorSymbol>(term: &LanguageTerm<LOS>) -> usize {
    if term.sub_terms.is_empty() {
        1
    } else {
        1 + term.sub_terms.iter().map(term_depth).max().unwrap()
    }
}

/// Returns a map from each operator symbol to the number of times it appears
/// in the term tree.
pub fn operator_count_by_symbol<LOS: RewritableLanguageOperatorSymbol>(
    term: &LanguageTerm<LOS>,
) -> HashMap<LOS, usize> {
    let mut counts = HashMap::new();
    count_operators(term, &mut counts);
    counts
}

fn count_operators<LOS: RewritableLanguageOperatorSymbol>(
    term: &LanguageTerm<LOS>,
    counts: &mut HashMap<LOS, usize>,
) {
    *counts.entry(term.operator.clone()).or_insert(0) += 1;
    for sub in &term.sub_terms {
        count_operators(sub, counts);
    }
}
