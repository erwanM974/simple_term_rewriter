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

use std::hash::Hash;

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol};

pub mod builtin;
pub use builtin::{dag_size, operator_count_by_symbol, term_depth, tree_size};

/// A countable, orderable metric that can be extracted from a term's operator symbols.
///
/// Implement this on an enum whose variants represent distinct measurement dimensions
/// (e.g. "uses `Add`", "uses `Mul`").  [`TermMetrics::extract_from_term`] collects
/// all metrics in one traversal.
pub trait TermSymbolMetric<LOS: RewritableLanguageOperatorSymbol>:
    Clone + PartialEq + Eq + PartialOrd + Copy + Ord + Hash + std::fmt::Display
{
    /// Returns `true` if the depth at which this metric is encountered should be tracked.
    ///
    /// When `true`, [`TermMetrics`] additionally records the maximum nesting depth for
    /// this metric across the entire term.
    fn measure_nested_depth(&self) -> bool;

    /// Returns the set of metrics contributed by a single occurrence of `op`.
    fn from_operator_symbol(op: &LOS) -> HashSet<Self>;
}

/// Aggregated metrics extracted from a single [`LanguageTerm`] traversal.
pub struct TermMetrics<LOS: RewritableLanguageOperatorSymbol, TSM: TermSymbolMetric<LOS>> {
    phantom: PhantomData<LOS>,
    /// How many times each metric symbol appears in the term.
    pub metrics_count: HashMap<TSM, u32>,
    /// Maximum depth of any node in the term tree (root = 1).
    pub term_depth: u32,
    /// For each metric with [`TermSymbolMetric::measure_nested_depth`] = `true`,
    /// the maximum consecutive nesting depth encountered.
    pub max_nested_metrics_depths: HashMap<TSM, u32>,
}

impl<LOS: RewritableLanguageOperatorSymbol, TSM: TermSymbolMetric<LOS>> TermMetrics<LOS, TSM> {
    /// Returns a human-readable list of metric lines, one per entry.
    pub fn string_summary(&self) -> Vec<String> {
        let mut mystrings = vec![];
        mystrings.push(format!("term depth : {:}", self.term_depth));
        for (metric, count) in &self.metrics_count {
            mystrings.push(format!("{:} : {:}", metric, count));
        }
        for (metric, nested_depth) in &self.max_nested_metrics_depths {
            mystrings.push(format!("{:}-max-nested-depth : {:}", metric, nested_depth));
        }
        mystrings
    }

    /// Traverses `term` once and collects all metrics.
    pub fn extract_from_term(term: &LanguageTerm<LOS>) -> Self {
        let mut metrics_count = HashMap::new();
        let (max_nested_metrics_depths, term_depth) =
            Self::extract_rec(term, &mut metrics_count, 1, &HashMap::new());

        Self {
            phantom: PhantomData,
            metrics_count,
            term_depth,
            max_nested_metrics_depths,
        }
    }

    fn extract_rec(
        term: &LanguageTerm<LOS>,
        metrics_count: &mut HashMap<TSM, u32>,
        parent_depth: u32,
        parent_nested_depths: &HashMap<TSM, u32>,
    ) -> (HashMap<TSM, u32>, u32) {
        let mut current_nested_depths = parent_nested_depths.clone();
        for metric in TSM::from_operator_symbol(&term.operator) {
            *metrics_count.entry(metric).or_insert(0) += 1;
            if metric.measure_nested_depth() {
                *current_nested_depths.entry(metric).or_insert(0) += 1;
            }
        }
        let mut max_depth = parent_depth;
        let mut max_nested_depths = current_nested_depths.clone();
        for sub_term in term.sub_terms.iter() {
            let (child_max_nested_depths, child_max_depth) = Self::extract_rec(
                sub_term,
                metrics_count,
                parent_depth + 1,
                &current_nested_depths,
            );
            max_depth = u32::max(max_depth, child_max_depth);
            for (metric, nested_depth) in child_max_nested_depths {
                let entry = max_nested_depths.entry(metric).or_insert(0);
                *entry = u32::max(*entry, nested_depth);
            }
        }

        (max_nested_depths, max_depth)
    }
}
