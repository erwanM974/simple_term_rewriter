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
use std::collections::HashMap;
use graph_process_manager_core::queue::priorities::AbstractPriorities;

use super::step::RewriteStepKind;



/** 
 * Specifies how individual rewrite rules and their applications should be prioritized.
 * For each kind of rule, a score modifier is given:
 * - if it is high, the rule is of higher priority w.r.t. the others rules
 * - if it is low, the rule is of lower priority w.r.t. the others rules
 * 
 * A depth modifier specifies, for a given rule, whether we should prefer to apply it closer to the root position, or closer to the leaves:
 * - If it has value zero, it is indifferent
 * - If it has a value lower than zero, applying the rule closer to the root is prefered
 * - If it has a valye greater than zero, applying the rule deeper in the term is preferred
 * **/
pub struct RewritePriorities {
    // at each phase,
    pub rewrite_rules_priorities : HashMap<usize,HashMap<usize,i32>>,
    // at each phase, either innermost or outermost
    pub depth_modifiers : HashMap<usize,i32>,
    // priority of the special rule to go to the next phase
    pub next_phase_priority : i32 
}



impl RewritePriorities {
    pub fn new(rewrite_rules_priorities: HashMap<usize, HashMap<usize, i32>>, depth_modifiers: HashMap<usize, i32>, next_phase_priority: i32) -> Self {
        Self { rewrite_rules_priorities, depth_modifiers, next_phase_priority }
    }
}

impl std::default::Default for RewritePriorities {
    fn default() -> Self {
        RewritePriorities::new(
            HashMap::new(),
            HashMap::new(),
            1
        )
    }
}



impl<LangOp: Clone + PartialEq + Eq + Hash> AbstractPriorities<RewriteStepKind<LangOp>> for RewritePriorities {
    fn get_priority_of_step(
        &self,
        step: &RewriteStepKind<LangOp>
    ) -> i32 {
        match step {
            RewriteStepKind::Transform(transfo) => {
                let mut score = 0;
                if let Some(phase_priorities) = self.rewrite_rules_priorities.get(
                    &transfo.phase_index
                ) {
                    if let Some(rule_priority) = phase_priorities.get(
                        &transfo.rule_index_in_phase
                    ) {
                        score += rule_priority;
                    }
                }
                if let Some(depth_modifier) = self.depth_modifiers.get(
                    &transfo.phase_index
                ) {
                    score+=(transfo.position.get_depth() as i32) *depth_modifier;
                }
                score
            },
            RewriteStepKind::GoToPhase(_) => {
                self.next_phase_priority
            } 
        }
    }

}

