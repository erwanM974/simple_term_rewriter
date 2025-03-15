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
use maplit::hashset;
use crate::core::terms::term::RewritableLanguageOperatorSymbol;
use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::step::RewriteStepKind;

use crate::rewriting_process::apply::get_transformations;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;

use crate::rewriting_process::state::RewritingProcessState;

use crate::rewriting_process::state::ConcreteRewritingPhaseInformation;

pub struct RewriteProcessHandler {}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractAlgorithmOperationHandler<RewriteConfig<LOS>> for RewriteProcessHandler {

    fn process_new_step(
        context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        global_state : &mut RewritingProcessState<LOS>,
        parent_node : &RewriteNodeKind<LOS>,
        step_to_process : &mut RewriteStepKind<LOS>
    ) -> RewriteNodeKind<LOS> {
        match step_to_process {
            RewriteStepKind::GoToSuccessorPhase(term_was_changed) => {
                // ***
                let previous_concrete_phase = global_state.concrete_phases.get(
                    parent_node.concrete_rewrite_phase_index
                ).unwrap();
                let previous_abstract_phase = context_and_param.phases.get(
                    previous_concrete_phase.model_abstract_phase_id
                ).unwrap();
                // ***
                let next_concrete_phase_id = if *term_was_changed {
                    if let Some(x) = global_state.successors_on_changed.get(&parent_node.concrete_rewrite_phase_index) {
                        *x
                    } else {
                        let next_abstract_phase_id = previous_abstract_phase.next_phase_id_on_changed.unwrap();
                        let next_concrete_phase_id = global_state.concrete_phases.len();
                        global_state.concrete_phases.push(
                            ConcreteRewritingPhaseInformation::new(
                                next_abstract_phase_id,
                                hashset!{
                                    parent_node.term.clone()
                                }
                            )
                        );
                        global_state.successors_on_changed.insert(parent_node.concrete_rewrite_phase_index, next_concrete_phase_id);
                        next_concrete_phase_id
                    }
                } else {
                    if let Some(x) = global_state.successors_on_unchanged.get(&parent_node.concrete_rewrite_phase_index) {
                        *x
                    } else {
                        let next_abstract_phase_id = previous_abstract_phase.next_phase_id_on_unchanged.unwrap();
                        let next_concrete_phase_id = global_state.concrete_phases.len();
                        global_state.concrete_phases.push(
                            ConcreteRewritingPhaseInformation::new(
                                next_abstract_phase_id,
                                hashset!{
                                    parent_node.term.clone()
                                }
                            )
                        );
                        global_state.successors_on_unchanged.insert(parent_node.concrete_rewrite_phase_index, next_concrete_phase_id);
                        next_concrete_phase_id
                    }
                };
                eprintln!("process step from node in phase {:} to succesor phase {:}", parent_node.concrete_rewrite_phase_index,next_concrete_phase_id);
                RewriteNodeKind::new(
                    parent_node.term.clone(),
                    next_concrete_phase_id
                )
            },
            RewriteStepKind::TransformInSamePhase(ref mut result) => {
                eprintln!("process step from node in phase {:} : rule {:} applied", parent_node.concrete_rewrite_phase_index,result.rule_index_in_phase);
                RewriteNodeKind::new(
                    result.result.take().unwrap(),
                    parent_node.concrete_rewrite_phase_index
                )
            }
        }
    }


    fn collect_next_steps(
        context_and_param : &RewritingProcessContextAndParameterization<LOS>,
        global_state : &mut RewritingProcessState<LOS>,
        parent_node : &RewriteNodeKind<LOS>
    ) -> Vec<RewriteStepKind<LOS>> {
        eprintln!("collecting steps from node in phase {:}", parent_node.concrete_rewrite_phase_index);
        let concrete_phase = global_state.concrete_phases.get_mut(
            parent_node.concrete_rewrite_phase_index
        ).unwrap();
        let abstract_phase = context_and_param.phases.get(
            concrete_phase.model_abstract_phase_id
        ).unwrap();
        // ***
        let transfos = get_transformations(
            parent_node.concrete_rewrite_phase_index,
            &abstract_phase.rules,
            &parent_node.term,
            context_and_param.keep_only_one
        );
        if transfos.is_empty() {
            // the term is irreducible
            if !concrete_phase.final_irreducible_terms.contains(&parent_node.term) {
                concrete_phase.final_irreducible_terms.insert(parent_node.term.clone());
            }
            if concrete_phase.initial_input_terms.contains(&parent_node.term) {
                // the term was not changed by the rewriting done in the phase
                if abstract_phase.next_phase_id_on_unchanged.is_some() {
                    vec![
                        RewriteStepKind::GoToSuccessorPhase(false)
                    ]
                } else {
                    vec![]
                }
            } else {
                // the term was changed by the rewriting done in the phase
                if abstract_phase.next_phase_id_on_changed.is_some() {
                    vec![
                        RewriteStepKind::GoToSuccessorPhase(true)
                    ]
                } else {
                    vec![]
                }
            }
        } else {
            // the term is not irreducible, some rewrite rules can be applied
            transfos.into_iter()
                .map(|r|
                    RewriteStepKind::TransformInSamePhase(r)
                )
                .collect()
        }
        
    }

}