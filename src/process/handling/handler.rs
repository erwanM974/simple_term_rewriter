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


use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;


use crate::core::interface::SimpleTermRewritingInterface;
use crate::process::conf::{RewriteConfig, RewriteStaticLocalVerdictAnalysisProof};
use crate::process::context::RewriteContext;
use crate::process::filter::filter::RewriteFilterCriterion;
use crate::process::node::RewriteNodeKind;
use crate::process::param::RewriteParameterization;
use crate::process::step::RewriteStepKind;
use crate::process::verdict::local::RewriteLocalVerdict;

use crate::core::apply::get_transformations;

pub struct RewriteProcessHandler {}

impl<STRI : SimpleTermRewritingInterface> AbstractProcessHandler<RewriteConfig<STRI>> for RewriteProcessHandler {

    fn process_new_step(_context: &RewriteContext,
                        _param : &RewriteParameterization<STRI>,
                        parent_state: &GenericNode<RewriteNodeKind<STRI::LanguageOperator>>,
                        step_to_process: &GenericStep<RewriteStepKind<STRI>>,
                        _new_state_id: u32,
                        _node_counter: u32) -> RewriteNodeKind<STRI::LanguageOperator> {
        match step_to_process.kind {
            RewriteStepKind::GoToNextPhase => {
                RewriteNodeKind::new(parent_state.kind.term.clone(),parent_state.kind.rewrite_system_index + 1)
            },
            RewriteStepKind::Transform(ref result) => {
                RewriteNodeKind::new(result.result.clone(),parent_state.kind.rewrite_system_index)
            }
        }
    }

    fn get_criterion(_context: &RewriteContext,
                     _param : &RewriteParameterization<STRI>,
                     _parent_state: &GenericNode<RewriteNodeKind<STRI::LanguageOperator>>,
                     _step_to_process: &GenericStep<RewriteStepKind<STRI>>,
                     _new_state_id: u32,
                     _node_counter: u32) -> RewriteFilterCriterion {
        RewriteFilterCriterion{}
    }

    fn collect_next_steps(_context: &RewriteContext,
                          param : &RewriteParameterization<STRI>,
                          parent_node_kind: &RewriteNodeKind<STRI::LanguageOperator>)
                -> Vec<RewriteStepKind<STRI>> {
        match param.phases.get(parent_node_kind.rewrite_system_index) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = get_transformations(
                    phase,
                    &parent_node_kind.term,
                    param.keep_only_one
                );
                if transfos.is_empty() {
                    vec![RewriteStepKind::GoToNextPhase]
                } else {
                    transfos.into_iter()
                        .map(|r|
                            RewriteStepKind::Transform(r)
                        )
                        .collect()
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &RewriteContext,
                                       _param : &RewriteParameterization<STRI>,
                                       node_kind: &RewriteNodeKind<STRI::LanguageOperator>) -> RewriteLocalVerdict<STRI::LanguageOperator> {
        RewriteLocalVerdict{got_term:node_kind.term.clone()}
    }

    fn get_local_verdict_from_static_analysis(_context: &RewriteContext,
                                              _param : &RewriteParameterization<STRI>,
                                              _node_kind: &mut RewriteNodeKind<STRI::LanguageOperator>)
                -> Option<(RewriteLocalVerdict<STRI::LanguageOperator>,RewriteStaticLocalVerdictAnalysisProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &RewriteContext,
                                           _param : &RewriteParameterization<STRI>,
                                           _loc_verd: &RewriteLocalVerdict<STRI::LanguageOperator>) -> bool {
        true
    }
}