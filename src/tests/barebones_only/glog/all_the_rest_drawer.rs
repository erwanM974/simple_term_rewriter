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

use std::path::Path;
use ab_glyph::FontRef;
use graph_process_manager_loggers::graphviz::drawers::all_the_rest_drawer::CustomAllTheRestDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::item::{BuiltinGraphvizLoggerDefaultGvItemStyle, BuiltinGraphvizLoggerItemStyle};
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::style::GvNodeShape;

use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::filtration::RewritingFiltrationResult;
use crate::rewriting_process::loggers::glog::all_the_rest_drawer_utils::get_step_node_inner_style_as_image_paragraph;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::step::RewriteStepKind;

use crate::tests::boolean_logic::lang::SimplisticBooleanLogicOperators;
use crate::tests::common::util::new_image_with_colored_text;
use crate::tests::common::{DRAWING_GRAPHIC_FONT, SCALE};


pub const MY_COLOR_BLACK : [u8;3] = [0u8, 0u8, 0u8];
pub const MY_COLOR_RED : [u8;3] = [255u8, 0u8, 0u8];

pub struct MinimalRewritingStepDrawer {
    pub font : FontRef<'static>,
}

impl MinimalRewritingStepDrawer {
    pub fn new() -> Self {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        Self {font}
    }
}

impl CustomAllTheRestDrawerForGraphvizLogger<RewriteConfig<SimplisticBooleanLogicOperators>> for MinimalRewritingStepDrawer {

    fn get_step_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param: &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        step : &RewriteStepKind<SimplisticBooleanLogicOperators>,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        let para = get_step_node_inner_style_as_image_paragraph(
            context_and_param,
            step,
            MY_COLOR_BLACK,
            MY_COLOR_RED
        );
        new_image_with_colored_text(
            full_path,
            &para,
            &self.font,
            SCALE
        );
        BuiltinGraphvizLoggerItemStyle::CustomImage
    }
    
    fn get_step_edge_color(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        _step : &RewriteStepKind<SimplisticBooleanLogicOperators>,
    ) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        filtration_result: &RewritingFiltrationResult,
        _image_file_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        BuiltinGraphvizLoggerItemStyle::Default(
            BuiltinGraphvizLoggerDefaultGvItemStyle::new(
                GvNodeShape::Rectangle,
                filtration_result.to_string(), 
                18, 
                None,
                GraphvizColor::red, 
                GraphvizColor::red, 
                GraphvizColor::wheat
            )
        )
    }
    
    fn get_filter_edge_color(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        _filtration_result: &<RewriteConfig<SimplisticBooleanLogicOperators> as graph_process_manager_core::process::config::AbstractProcessConfiguration>::FiltrationResult,
    ) -> graphviz_dot_builder::colors::GraphvizColor {
        GraphvizColor::red
    }
    
    fn get_node_phase_id(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<SimplisticBooleanLogicOperators>,
        new_node: &RewriteNodeKind<SimplisticBooleanLogicOperators>
    ) -> Option<usize> {
        Some(new_node.concrete_rewrite_phase_index)
    }
    
    fn get_phase_color(&self, phase_id : usize) -> graphviz_dot_builder::colors::GraphvizColor {
        vec![
            GraphvizColor::lightskyblue,
            GraphvizColor::lightgoldenrod1,
            GraphvizColor::seagreen1,
            GraphvizColor::lightsalmon
        ].get(phase_id % 4).unwrap().clone()
    }

}



