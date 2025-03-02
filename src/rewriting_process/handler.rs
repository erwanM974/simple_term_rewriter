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


use graph_process_manager_core::process::handler::AbstractAlgorithmOperationHandler;
use crate::core::term::RewritableLanguageOperatorSymbol;
use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::step::RewriteStepKind;

use crate::core::apply::get_transformations;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;

pub struct RewriteProcessHandler {}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractAlgorithmOperationHandler<RewriteConfig<LOS>> for RewriteProcessHandler {

    fn process_new_step(
        _context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        parent_node : &RewriteNodeKind<LOS>,
        step_to_process : &RewriteStepKind<LOS>
    ) -> RewriteNodeKind<LOS> {
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
        context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        parent_node : &RewriteNodeKind<LOS>
    ) -> Vec<RewriteStepKind<LOS>> {
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
                    // the term is irreducible
                    if parent_node.rewrite_system_index < context_and_param.phases.len() - 1 {
                        // there remains at least another phase after
                        vec![RewriteStepKind::GoToPhase(parent_node.rewrite_system_index + 1)]
                    } else {
                        // the last phase is over (the term is irreducible in the last phase)
                        vec![]
                    }
                } else {
                    // the term is not irreducible, some rewrite rules can be applied
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