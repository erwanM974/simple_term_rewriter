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

use std::{collections::HashSet, hash::Hash};

use crate::process::{context::RewritingProcessContextAndParameterization, filter::{RewriteNodePreFilter, RewriteStepFilter}, priorities::RewritePriorities};


pub fn get_rewrite_parameters_description<LangOp : Clone + PartialEq + Eq + Hash>(
    context_and_param : &RewritingProcessContextAndParameterization<LangOp>
) -> Vec<Vec<String>> {
    let mut params_descs = vec![];
    for (phase_id,phase) in context_and_param.phases.iter().enumerate() {
        let mut param_desc = vec![];
        param_desc.push(format!("phase {} : [",phase_id));
        // ***
        param_desc.push("  rules = [".to_string());
        let rules_descs : Vec<String> = phase.rules.iter().map(|x| x.get_desc()).collect();
        let rules_descs_len = rules_descs.len();
        for (r_id,r_line) in rules_descs.into_iter().enumerate() {
            if r_id < rules_descs_len - 1 {
                param_desc.push( format!("    {},", r_line));
            } else {
                param_desc.push( format!("    {}", r_line));
            }
        }
        param_desc.push("  ];".to_string());
        // ***
        if let Some(target_phase) = &phase.goto_at_end {
            param_desc.push(format!("  goto_at_end = {:};", target_phase));
        }
        // ***
        param_desc.push(format!("  keep_only_one = {:};", phase.keep_only_one));
        // ***
        param_desc.push("]".to_string());
        // ***
        params_descs.push(param_desc);
    }
    params_descs
}



pub fn get_rewrite_priorities_description(priorities : &RewritePriorities) -> Vec<Vec<String>> {
    let mut priorities_descs = vec![];
    let sorted_phases = {
        let all_phases : HashSet<usize> = priorities.rewrite_rules_priorities.keys().chain(priorities.depth_modifiers.keys()).cloned().collect();
        let mut x : Vec<usize> = all_phases.into_iter().collect();
        x.sort();
        x  
    };
    for phase_id in sorted_phases {
        let mut p_desc = vec![];
        p_desc.push(format!("phase {} : [",phase_id));
        // ***
        if let Some(rules_priorities) = priorities.rewrite_rules_priorities.get(&phase_id) {
            let sorted_rules = {
                let all_rules : HashSet<usize> = rules_priorities.keys().cloned().collect();
                let mut x : Vec<usize> = all_rules.into_iter().collect();
                x.sort();
                x  
            };
            let rules_descs : Vec<String> = sorted_rules.into_iter()
            .map(|x| format!("rule {} = {}", x, rules_priorities.get(&x).unwrap())).collect();
            p_desc.push(format!("  rules = [{}]", rules_descs.join(", ")));
        }
        // ***
        if let Some(depth_mod) = priorities.depth_modifiers.get(&phase_id) {
            p_desc.push(format!("  depth_modifier = {}", depth_mod));
        }
        // ***
        p_desc.push("]".to_string());
        priorities_descs.push(p_desc);
    }
    priorities_descs.push(vec![format!("next_phase_priority = {}", priorities.next_phase_priority)]);
    // ***
    priorities_descs
}

pub fn get_rewrite_step_filter_description(filter : &RewriteStepFilter) -> Vec<String> {
    match filter {
        RewriteStepFilter::MaxNodeNumber(num) => {
            vec![format!("MaxNum={}",num)]
        }
    }
}


pub fn get_rewrite_node_pre_filter_description<LangOp: Clone + PartialEq + Eq + Hash>(filter : &RewriteNodePreFilter<LangOp>) -> Vec<String> {
    match filter {
        RewriteNodePreFilter::MustSatPredicate(pred) => {
            vec![format!("predicate={}",pred.get_desc())]
        }
    }
}

