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
use graph_process_manager_core::process::handler::AbstractAlgorithmOperationHandler;
use crate::process::conf::RewriteConfig;
use crate::process::node::RewriteNodeKind;
use crate::process::step::RewriteStepKind;

use crate::core::apply::get_transformations;
use crate::process::context::RewritingProcessContextAndParameterization;

pub struct RewriteProcessHandler {}

impl<LangOp: Clone + PartialEq + Eq + Hash> AbstractAlgorithmOperationHandler<RewriteConfig<LangOp>> for RewriteProcessHandler {

    fn process_new_step(
        _context_and_param : &RewritingProcessContextAndParameterization<LangOp>,
        parent_node : &RewriteNodeKind<LangOp>,
        step_to_process : &RewriteStepKind<LangOp>
    ) -> RewriteNodeKind<LangOp> {
        match step_to_process {
            RewriteStepKind::GoToPhase(phase_id) => {
                RewriteNodeKind::new(
                    parent_node.term.clone(),
                    *phase_id
                )
            },
            RewriteStepKind::Transform(ref result) => {
                RewriteNodeKind::new(
                    result.result.clone(),
                    parent_node.rewrite_system_index
                )
            }
        }
    }

    fn collect_next_steps(
        context_and_param : &RewritingProcessContextAndParameterization<LangOp>,
        parent_node : &RewriteNodeKind<LangOp>
    ) -> Vec<RewriteStepKind<LangOp>> {
        match context_and_param.phases.get(parent_node.rewrite_system_index) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = get_transformations(
                    parent_node.rewrite_system_index,
                    &phase.rules,
                    &parent_node.term,
                    phase.keep_only_one
                );
                if transfos.is_empty() {
                    match phase.goto_at_end {
                        Some(target_phase_id) => {
                            vec![RewriteStepKind::GoToPhase(target_phase_id)]
                        },
                        None => {
                            vec![]
                        },
                    }
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

}