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



use graph_process_manager_core::process::filter::{AbstractNodePreFilter, AbstractStepFilter};
use crate::core::predicate::PredicateOnTerm;
use crate::core::term::RewritableLanguageOperatorSymbol;
use crate::process::conf::RewriteConfig;
use crate::process::context::RewritingProcessContextAndParameterization;
use crate::process::filtration::RewritingFiltrationResult;
use crate::process::node::RewriteNodeKind;
use crate::process::state::RewritingProcessState;
use crate::process::step::RewriteStepKind;

pub enum RewriteStepFilter {
    MaxNodeNumber(u32)
}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractStepFilter<RewriteConfig<LOS>> for RewriteStepFilter {
    fn apply_filter(
        &self,
        _context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        global_state : &RewritingProcessState<LOS>,
        _parent_node : &RewriteNodeKind<LOS>,
        _step : &RewriteStepKind<LOS>
    ) -> Option<RewritingFiltrationResult> {
        match self {
            RewriteStepFilter::MaxNodeNumber( max_node_number ) => {
                if global_state.node_count >= *max_node_number {
                    return Some( RewritingFiltrationResult::MaxNodeNumber );
                }
            }
        }
        None
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}





pub enum RewriteNodePreFilter<LOS : RewritableLanguageOperatorSymbol> {
    MustSatPredicate(Box<dyn PredicateOnTerm<LOS>>)
}




impl<LOS : RewritableLanguageOperatorSymbol> AbstractNodePreFilter<RewriteConfig<LOS>> for RewriteNodePreFilter<LOS> {

    fn apply_filter(
        &self,
        _context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        _global_state : &RewritingProcessState<LOS>,
        node : &RewriteNodeKind<LOS>,
    ) -> Option<RewritingFiltrationResult> {
        match self {
            RewriteNodePreFilter::MustSatPredicate( pred ) => {
                if !pred.term_satisfies(&node.term) {
                    return Some( RewritingFiltrationResult::PredicateUnsat(pred.get_desc()) );
                }
            }
        }
        None
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}