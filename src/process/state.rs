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


use std::collections::HashMap;
use graph_process_manager_core::process::persistent_state::AbstractProcessMutablePersistentState;
use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};
use crate::process::conf::RewriteConfig;
use crate::process::context::RewritingProcessContextAndParameterization;
use crate::process::filtration::RewritingFiltrationResult;
use crate::process::node::RewriteNodeKind;
use crate::process::step::RewriteStepKind;

pub struct RewritingProcessState<LOS : RewritableLanguageOperatorSymbol> {
    /// keeps track of the irreducible terms encountered in each phase of the rewriting process
    pub irreducible_terms_per_phase : HashMap<usize,Vec<LanguageTerm<LOS>>>,
    pub node_count : u32
}

impl<LOS : RewritableLanguageOperatorSymbol> RewritingProcessState<LOS> {
    pub fn new(
        irreducible_terms_per_phase: HashMap<usize, Vec<LanguageTerm<LOS>>>,
        node_count : u32
    ) -> Self {
        Self {
            irreducible_terms_per_phase,
            node_count
        }
    }
}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractProcessMutablePersistentState<RewriteConfig<LOS>> for RewritingProcessState<LOS> {
    fn get_initial_state(
        context_and_param: &RewritingProcessContextAndParameterization<LOS>
    ) -> Self {
        let mut irreducible_terms_per_phase = HashMap::new();
        for x in 0..context_and_param.phases.len() {
            irreducible_terms_per_phase.insert(
                x,
                vec![]
            );
        }
        Self::new(irreducible_terms_per_phase,0)
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
        node: &RewriteNodeKind<LOS>,
        steps: &[RewriteStepKind<LOS>]
    ) {
        let reached_term_is_irreducible = match steps.len() {
            0 => {
                true
            },
            1 => {
                let only_step = steps.first().unwrap();
                matches!(only_step, RewriteStepKind::GoToPhase(_))
            },
            _ => {
                false
            }
        };
        if reached_term_is_irreducible {
            let irrs = self.irreducible_terms_per_phase.get_mut(
                &node.rewrite_system_index
            ).unwrap();
            if !irrs.contains(&node.term) {
                irrs.push(node.term.clone());
            }
        }
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