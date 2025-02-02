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

use graph_process_manager_core::manager::config::AbstractNodeKind;



use crate::core::term::LanguageTerm;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RewriteNodeKind<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    pub term : LanguageTerm<LanguageOperatorSymbol>,
    pub rewrite_system_index : usize
}

impl<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> RewriteNodeKind<LanguageOperatorSymbol> {
    pub fn new(
        term : LanguageTerm<LanguageOperatorSymbol>,
        rewrite_system_index : usize) -> Self {
            Self { term, rewrite_system_index }
    }
}


impl<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> AbstractNodeKind for RewriteNodeKind<LanguageOperatorSymbol> {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        self.term == memoized_node.term && self.rewrite_system_index == memoized_node.rewrite_system_index
    }
}

