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

use crate::position::PositionInLanguageTerm;
use crate::process::strategy::{run_to_completion, RewriteProcess};
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// One-shot, untraced executor for a [`RewriteProcess`](super::strategy::RewriteProcess).
///
/// Applies the strategy to a term and returns all possible results without
/// recording which rules fired or where.  For step-by-step traces use
/// [`RewriteProcessTracedExecutor`](super::traced::RewriteProcessTracedExecutor).
pub struct RewriteProcessUntracedExecutor {}

impl RewriteProcessUntracedExecutor {
    /// Apply the strategy to `term` and return all possible resulting terms.
    /// For a step-by-step traced execution see [`RewriteProcessTracedExecutor`](super::traced::RewriteProcessTracedExecutor).
    pub fn rewrite<LOS: RewritableLanguageOperatorSymbol>(
        strategy: &RewriteProcess<LOS>,
        term: &LanguageTerm<LOS>,
        factory: &mut TermFactory<LOS>,
    ) -> Vec<LanguageTerm<LOS>> {
        run_to_completion(
            strategy,
            term,
            term,
            &PositionInLanguageTerm::get_root_position(),
            factory,
        )
    }
}
