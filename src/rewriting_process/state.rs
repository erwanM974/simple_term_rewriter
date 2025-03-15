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


use std::collections::{HashMap, HashSet};
use graph_process_manager_core::process::persistent_state::AbstractProcessMutablePersistentState;
use maplit::hashset;
use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};
use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::filtration::RewritingFiltrationResult;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::step::RewriteStepKind;



/** 
 * A concrete phase in the rewriting process.
 * **/
pub struct ConcreteRewritingPhaseInformation<LOS : RewritableLanguageOperatorSymbol> {
    /// the id of the abstract phase which determines the rewrite rules that are applied during this concrete phase
    pub model_abstract_phase_id : usize,
    /// the initial terms before applying rewriting in this phase (should only be a single one if the previous phase is convergent)
    pub initial_input_terms : HashSet<LanguageTerm<LOS>>,
    /// the final irreducible terms after applying the rewriting in this phase (should only be a single one if the rewrite system is convergent)
    pub final_irreducible_terms : HashSet<LanguageTerm<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> ConcreteRewritingPhaseInformation<LOS> {
    pub fn new(model_abstract_phase_id: usize, initial_input_terms: HashSet<LanguageTerm<LOS>>) -> Self {
        Self { 
            model_abstract_phase_id, 
            initial_input_terms, 
            final_irreducible_terms: HashSet::new() 
        }
    }
}

/** 
 * The persistent global state of the rewriting process.
 * **/
pub struct RewritingProcessState<LOS : RewritableLanguageOperatorSymbol> {
    /// keeps track of the concrete phases (each one being a classical TRS) that can be applied successively to rewrite the initial term
    pub concrete_phases : Vec<ConcreteRewritingPhaseInformation<LOS>>,
    pub successors_on_changed : HashMap<usize,usize>,
    pub successors_on_unchanged : HashMap<usize,usize>,
    pub node_count : u32
}

impl<LOS : RewritableLanguageOperatorSymbol> RewritingProcessState<LOS> {
    pub fn new(
        concrete_phases : Vec<ConcreteRewritingPhaseInformation<LOS>>,
        node_count : u32
    ) -> Self {
        Self {
            concrete_phases,
            successors_on_changed : HashMap::new(),
            successors_on_unchanged : HashMap::new(),
            node_count
        }
    }
}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractProcessMutablePersistentState<RewriteConfig<LOS>> for RewritingProcessState<LOS> {
    fn get_initial_state(
        _context_and_param: &RewritingProcessContextAndParameterization<LOS>,
        initial_node : &RewriteNodeKind<LOS>
    ) -> Self {
        let concrete_phases = vec![
            ConcreteRewritingPhaseInformation::new(
                0,
                hashset!{initial_node.term.clone()}
            )
        ];
        Self::new(
            concrete_phases,
            0
        )
    }

    fn update_on_node_reached(
        &mut self,
        _context_and_param: &RewritingProcessContextAndParameterization<LOS>,
        _node: &RewriteNodeKind<LOS>
    ) {
        self.node_count += 1;
    }

    fn update_on_next_steps_collected_reached(
        &mut self,
        _context_and_param: &RewritingProcessContextAndParameterization<LOS>,
        _node: &RewriteNodeKind<LOS>,
        _steps: &[RewriteStepKind<LOS>]
    ) {
        // nothing
    }

    fn update_on_filtered(
        &mut self,
        _context_and_param: &RewritingProcessContextAndParameterization<LOS>,
        _parent_node: &RewriteNodeKind<LOS>,
        _filtration_result: &RewritingFiltrationResult
    ) {
        // nothing
    }

    fn warrants_termination_of_the_process(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<LOS>
    ) -> bool {
        // termination here corresponds to reaching (one/all) irreducible terms
        // in the last rewrite phase
        false
    }
}