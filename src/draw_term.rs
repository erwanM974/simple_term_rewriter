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

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use graphviz_dot_builder::traits::DotTranslatable;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadStyle};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyle;
use graphviz_dot_builder::traits::DotBuildable;

use crate::core::position::PositionInLanguageTerm;
use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};



pub trait TermDrawingContext<LOS : RewritableLanguageOperatorSymbol> {

    fn get_operator_representation_as_graphviz_node_style(
        &self, 
        operator : &LOS
    ) -> GraphvizNodeStyle;

}



pub fn draw_term_tree_with_graphviz<
        LOS : RewritableLanguageOperatorSymbol,
        TDC : TermDrawingContext<LOS>
>(
    tdc : &TDC,
    term : &LanguageTerm<LOS>,
    temp_file_path : &Path,
    output_file_path : &Path,
) 
{
    // ***
    let mut temp_file = File::create(temp_file_path).unwrap();
    let _ = temp_file.write( term_gv_repr::<LOS,TDC>(tdc,term).to_dot_string().as_bytes() );
    // ***
    let _ = Command::new("dot")
        .arg("-Tpng")
        .arg(temp_file_path)
        .arg("-o")
        .arg(output_file_path)
        .output();
}


pub fn term_gv_repr<
LOS : RewritableLanguageOperatorSymbol,
TDC : TermDrawingContext<LOS>
>(
    tdc : &TDC,
    term : &LanguageTerm<LOS>) -> GraphVizDiGraph 
{
    let mut digraph = GraphVizDiGraph::new(vec![]);
    term_gv_repr_rec::<LOS,TDC>(tdc,term,PositionInLanguageTerm::get_root_position(), &mut digraph);
    digraph
}


fn term_gv_repr_rec<
LOS : RewritableLanguageOperatorSymbol,
TDC : TermDrawingContext<LOS>
>(
    tdc : &TDC,
    term : &LanguageTerm<LOS>,
    current_pos : PositionInLanguageTerm,
    gv_graph : &mut GraphVizDiGraph) -> String 
{
    let node_name = format!("p{:}",current_pos);
    // the parent node
    {
        let parent_node_gv_options = tdc.get_operator_representation_as_graphviz_node_style(&term.operator);
        gv_graph.add_node( GraphVizNode::new(node_name.clone(), parent_node_gv_options) );
    }
    // the child nodes
    for (n,sub_term) in term.sub_terms.iter().enumerate() {
        let child_pos = current_pos.clone().position_as_nth_sub_term(n);
        let child_node_name = term_gv_repr_rec(
            tdc,
            sub_term,
            child_pos,
            gv_graph
        );
        let gv_edge = GraphVizEdge::new(node_name.clone(),
                                        None,
                                        child_node_name,
                                        None,
                                        vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
        gv_graph.add_edge(gv_edge);
    }
    node_name
}