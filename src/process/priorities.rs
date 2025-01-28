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
use std::{collections::HashMap, fmt};
use graph_process_manager_core::delegate::priorities::AbstractPriorities;

use crate::core::interface::SimpleTermRewritingInterface;

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
pub struct RewritePriorities<TransformationKind : Clone + PartialEq + Eq + Hash> {
    pub transfo_kind_priorities : HashMap<TransformationKind,i32>,
    pub depth_modifiers : HashMap<TransformationKind,i32>,
    pub next_phase_priority : i32 
}

impl<TransformationKind : Clone + PartialEq + Eq + Hash> std::default::Default for RewritePriorities<TransformationKind> {
    fn default() -> Self {
        RewritePriorities::new(HashMap::new(), HashMap::new(), 1)
    }
}

impl<TransformationKind : Clone + PartialEq + Eq + Hash> RewritePriorities<TransformationKind> {

    pub fn new(
        transfo_kind_priorities : HashMap<TransformationKind,i32>, 
        depth_modifiers : HashMap<TransformationKind,i32>, 
        next_phase_priority : i32
    ) -> Self {
        RewritePriorities{transfo_kind_priorities,depth_modifiers,next_phase_priority}
    }

}

impl<TransformationKind : Clone + PartialEq + Eq + Hash> fmt::Display for RewritePriorities<TransformationKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO : properly print the priorities
        write!(f,"[]")
    }
}

impl<STRI : SimpleTermRewritingInterface> AbstractPriorities<RewriteStepKind<STRI>> for RewritePriorities<STRI::TransformationKind>{
    fn get_priority_of_step(&self, step: &RewriteStepKind<STRI>) -> i32 {
        match step {
            RewriteStepKind::Transform(transfo) => {
                let mut score = 0;
                if let Some(kind_priority) = self.transfo_kind_priorities.get(&transfo.kind) {
                    score += kind_priority;
                }
                score
            },
            RewriteStepKind::GoToNextPhase => {
                self.next_phase_priority
            } 
        }
    }
}

