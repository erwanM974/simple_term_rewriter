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

use graph_process_manager_core::process::config::AbstractNodeKind;


use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RewriteNodeKind<LOS : RewritableLanguageOperatorSymbol> {
    /// the term that is being rewritten
    pub term : LanguageTerm<LOS>,
    /// the identifier of the concrete rewrite phase as part of which the term is being rewritten
    pub concrete_rewrite_phase_index : usize
}

impl<LOS : RewritableLanguageOperatorSymbol> RewriteNodeKind<LOS> {
    pub fn new(
        term : LanguageTerm<LOS>,
        concrete_rewrite_phase_index : usize
    ) -> Self {
        Self { term, concrete_rewrite_phase_index }
    }
}


impl<LOS : RewritableLanguageOperatorSymbol> AbstractNodeKind for RewriteNodeKind<LOS> {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        self.term == memoized_node.term && self.concrete_rewrite_phase_index == memoized_node.concrete_rewrite_phase_index
    }
}

