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
use image::Rgb;
use image_colored_text::text::line::ColoredTextLine;
use image_colored_text::text::paragraph::{ColoredTextParagraph, MultiLineTextAlignment};
use graph_process_manager_loggers::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;

use crate::process::conf::RewriteConfig;
use crate::process::context::RewriteContext;
use crate::process::param::RewriteParameterization;
use crate::process::step::RewriteStepKind;
use crate::tests::lang::MinimalExampleInterface;

use super::common::{DRAWING_GRAPHIC_FONT, SCALE};
use super::util::new_image_with_colored_text;



pub const MY_COLOR_WHITE : [u8;3] = [255u8,  255u8,  255u8];
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

impl CustomStepDrawerForGraphvizLogger<RewriteConfig<MinimalExampleInterface>> for MinimalRewritingStepDrawer {

    fn draw(&self,
            step : &RewriteStepKind<MinimalExampleInterface>,
            _context: &RewriteContext,
            _parameterization: &RewriteParameterization<MinimalExampleInterface>,
            full_path : &Path) {
        let line = match step {
            RewriteStepKind::Transform(term_transformation_result) => {
                ColoredTextLine::new(
                    vec![
                        (format!("{:}", term_transformation_result.kind), Rgb(MY_COLOR_BLACK)),
                        (format!("@"), Rgb(MY_COLOR_RED)),
                        (format!("{:}", term_transformation_result.position), Rgb(MY_COLOR_BLACK)),
                    ]
                )
            },
            RewriteStepKind::GoToNextPhase => {
                ColoredTextLine::new(
                    vec![
                        (format!("→phase→"), Rgb(MY_COLOR_RED))
                    ]
                )
            },
        };
        let para = ColoredTextParagraph::new(
            vec!(line),
            MultiLineTextAlignment::Center,
            None,
            None
        );
        new_image_with_colored_text(
            full_path,
            &para,
            &self.font,
            SCALE
        );
    }

}



