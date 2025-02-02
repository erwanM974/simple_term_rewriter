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



use graphviz_dot_builder::colors::GraphvizColor;
use graph_process_manager_loggers::graphviz::builtin::builtin_process_drawer_trait::BuiltinProcessDrawer;
use graph_process_manager_loggers::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::builtin::proof_drawer::CustomProofDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;

use crate::process::conf::RewriteConfig;
use crate::process::context::RewriteContext;
use crate::process::node::RewriteNodeKind;
use crate::process::param::RewriteParameterization;
use crate::process::verdict::local::RewriteLocalVerdict;
use crate::tests::barebones_only::lang::{MinimalExampleInterface, MinimalExampleLangOperators};

use crate::tests::barebones_only::glog::node_drawer::MinimalRewritingNodeDrawer;
use crate::tests::barebones_only::glog::step_drawer::MinimalRewritingStepDrawer;




pub struct MinimalRewritingProcessDrawer {
    pub temp_folder : String,
    pub node_drawers : Vec<Box<
        dyn CustomNodeDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>
    >>,
    pub step_drawer : Box<
        dyn CustomStepDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>
    >,
}

impl MinimalRewritingProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        let drawer : Box<dyn CustomNodeDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>>
         = Box::new(MinimalRewritingNodeDrawer{});
        let node_drawers = vec![drawer];
        let step_drawer : Box<dyn CustomStepDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>> = Box::new(
            MinimalRewritingStepDrawer::new()
        ) ;
        MinimalRewritingProcessDrawer { temp_folder, node_drawers, step_drawer }
    }
}


impl BuiltinProcessDrawer<RewriteConfig<MinimalExampleInterface>> for MinimalRewritingProcessDrawer {
    fn get_node_drawers(&self) -> &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>>> {
        &self.node_drawers
    }

    fn get_step_drawer(&self) -> &Box<dyn CustomStepDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>> {
        &self.step_drawer
    }

    fn get_proof_drawer(&self) -> Option<&Box<dyn CustomProofDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>>>> {
        None
    }

    fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    fn get_verdict_color(&self, _local_verdict: &RewriteLocalVerdict<MinimalExampleLangOperators>) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_node_phase_id(&self,
        _context: &RewriteContext,
        _param: &RewriteParameterization<MinimalExampleInterface>,
        _new_node: &RewriteNodeKind<MinimalExampleLangOperators>) -> Option<u32> {
        None 
    }
    
    fn get_phase_color(&self, _phase_id : u32) -> GraphvizColor {
        GraphvizColor::black
    }
}