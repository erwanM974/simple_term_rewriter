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


use std::path::{Path, PathBuf};
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem};
use graph_process_manager_loggers::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;

use crate::draw_term::{draw_term_tree_with_graphviz, TermDrawingContext};
use crate::process::conf::RewriteConfig;
use crate::process::context::RewriteContext;
use crate::process::node::RewriteNodeKind;
use crate::process::param::RewriteParameterization;
use crate::tests::barebones_only::lang::{MinimalExampleInterface, MinimalExampleLangOperators};



pub struct MinimalRewritingNodeDrawer {}

impl TermDrawingContext<MinimalExampleLangOperators> for MinimalRewritingNodeDrawer {
    fn get_operator_representation_as_graphviz_node_style(
        &self, 
        operator : &MinimalExampleLangOperators
    ) -> GraphvizNodeStyle {
        vec![
            GraphvizNodeStyleItem::Label(format!("{:?}", operator))
        ]
    }
}

impl CustomNodeDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>> for MinimalRewritingNodeDrawer {

    fn draw(&self,
            node : &RewriteNodeKind<MinimalExampleLangOperators>,
            _context: &RewriteContext,
            _parameterization: &RewriteParameterization<MinimalExampleInterface>,
            full_path : &Path) {
        // ***
        let temp_file_name = "temp.dot";
        let temp_path : PathBuf = [&temp_file_name].iter().collect();
        // ***
        draw_term_tree_with_graphviz::<MinimalExampleLangOperators,MinimalRewritingNodeDrawer>(
            self,&node.term,&temp_path.as_path(),full_path
        );
    }

}




