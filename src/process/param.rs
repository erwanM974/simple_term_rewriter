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


use graph_process_manager_core::manager::config::AbstractProcessParameterization;

use crate::core::{interface::BarebonesTermRewritingInterface, rule::RewriteRule};



pub struct RewriteParameterization<STRI : BarebonesTermRewritingInterface> {
    pub phases : Vec<Vec<Box<dyn RewriteRule<STRI>>>>,
    pub keep_only_one : bool
}

impl<STRI : BarebonesTermRewritingInterface> RewriteParameterization<STRI> {
    pub fn new(phases : Vec<Vec<Box<dyn RewriteRule<STRI>>>>, keep_only_one: bool) -> Self {
        Self { phases, keep_only_one }
    }
}

impl<STRI : BarebonesTermRewritingInterface> AbstractProcessParameterization for RewriteParameterization<STRI> {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut strs = vec!["process = rewriting;".to_string()];
        strs.push(format!("keep_only_one = {:}", self.keep_only_one));
        for (x,phase) in self.phases.iter().enumerate() {
            strs.push(format!("phase {:} = [", x+1));
            for rule in phase {
                strs.push(format!("         {:},", rule));
            }
            strs.push("      ];".to_string());
        }
        strs
    }
}