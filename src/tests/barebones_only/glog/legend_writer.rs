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

use graph_process_manager_core::process::filter::{AbstractNodePostFilter, AbstractNodePreFilter, AbstractStepFilter};
use graph_process_manager_loggers::graphviz::drawers::legend_writer::ProcessLegendWriter;

use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::filter::{RewriteNodePreFilter, RewriteStepFilter};
use crate::rewriting_process::loggers::glog::legend_writer_utils::{get_rewrite_node_pre_filter_description, get_rewrite_parameters_description, get_rewrite_step_filter_description};
use crate::rewriting_process::priorities::RewritePriorities;
use crate::rewriting_process::state::RewritingProcessState;
use crate::tests::boolean_logic::lang::SimplisticBooleanLogicOperators;




pub struct MinimalLegendWriter {}


impl ProcessLegendWriter<RewriteConfig<SimplisticBooleanLogicOperators>> for MinimalLegendWriter {
    fn get_process_description(&self) -> String {
        "rewriting minimal language".to_string()
    }

    fn get_parameters_description(&self, context_and_param : &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>) -> Vec<Vec<String>> {
        get_rewrite_parameters_description(context_and_param)
    }

    fn get_priorities_description(&self, _priorities : &RewritePriorities) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_step_filter_description(&self, filter : &dyn AbstractStepFilter<RewriteConfig<SimplisticBooleanLogicOperators>>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<RewriteStepFilter>() {
            Some(x) => {
                Some(get_rewrite_step_filter_description(x))
            }
            None => {
                None
            }
        }
    }

    fn get_node_pre_filter_description(&self, filter : &dyn AbstractNodePreFilter<RewriteConfig<SimplisticBooleanLogicOperators>>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<RewriteNodePreFilter<SimplisticBooleanLogicOperators>>() {
            Some(x) => {
                Some(get_rewrite_node_pre_filter_description(x))
            }
            None => {
                None
            }
        }
    }

    fn get_node_post_filter_description(&self, _filter : &dyn AbstractNodePostFilter<RewriteConfig<SimplisticBooleanLogicOperators>>) -> Option<Vec<String>> {
        None
    }

    fn get_final_global_state_description_for_legend(
        &self, 
        _context_and_param : &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        _final_state : &RewritingProcessState<SimplisticBooleanLogicOperators>
    ) -> Vec<String> {
        vec![]
    }
}