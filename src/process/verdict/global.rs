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
use std::fmt;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;

use crate::core::term::LanguageTerm;

use crate::process::verdict::local::RewriteLocalVerdict;



pub struct RewriteGlobalVerdict<LanguageOperator : Clone + PartialEq + Eq + Hash>  {
    pub normalized_terms : Vec<LanguageTerm<LanguageOperator>>
}

impl<LanguageOperator : Clone + PartialEq + Eq + Hash> fmt::Display for RewriteGlobalVerdict<LanguageOperator> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl<LanguageOperator : Clone + PartialEq + Eq + Hash> AbstractGlobalVerdict<RewriteLocalVerdict<LanguageOperator>> for RewriteGlobalVerdict<LanguageOperator> {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        RewriteGlobalVerdict{normalized_terms:vec![]}
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &RewriteLocalVerdict<LanguageOperator>) -> Self {
        let mut terms = self.normalized_terms;
        terms.push(local_verdict.got_term.clone());
        Self{normalized_terms:terms}
    }

    fn is_goal_reached(&self,
                       _goal: &Option<Self>) -> bool {
        false
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }

}


