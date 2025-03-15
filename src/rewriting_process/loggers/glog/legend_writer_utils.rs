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


use crate::core::terms::term::RewritableLanguageOperatorSymbol;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::filter::{RewriteNodePreFilter, RewriteStepFilter};


pub fn get_rewrite_parameters_description<LOS : RewritableLanguageOperatorSymbol>(
    context_and_param : &RewritingProcessContextAndParameterization<LOS>
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
        match phase.next_phase_id_on_unchanged {
            None => {
                param_desc.push("  next_phase_on_unchanged = None;".to_string());
            },
            Some(p_id) => {
                param_desc.push(format!("  next_phase_on_unchanged = phase {:};", p_id));
            }
        }
        // ***
        match phase.next_phase_id_on_changed {
            None => {
                param_desc.push("  next_phase_on_changed = None;".to_string());
            },
            Some(p_id) => {
                param_desc.push(format!("  next_phase_on_changed = phase {:};", p_id));
            }
        }
        // ***
        param_desc.push("]".to_string());
        // ***
        params_descs.push(param_desc);
    }
    params_descs
}



pub fn get_rewrite_step_filter_description(filter : &RewriteStepFilter) -> Vec<String> {
    match filter {
        RewriteStepFilter::MaxNodeNumber(num) => {
            vec![format!("MaxNum={}",num)]
        }
    }
}


pub fn get_rewrite_node_pre_filter_description<LOS : RewritableLanguageOperatorSymbol>(filter : &RewriteNodePreFilter<LOS>) -> Vec<String> {
    match filter {
        RewriteNodePreFilter::MustSatPredicate(pred) => {
            vec![format!("predicate={}",pred.get_desc())]
        }
    }
}

