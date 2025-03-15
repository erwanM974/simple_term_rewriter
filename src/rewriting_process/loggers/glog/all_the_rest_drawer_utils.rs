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

use image::Rgb;
use image_colored_text::text::line::ColoredTextLine;
use image_colored_text::text::paragraph::{ColoredTextParagraph, MultiLineTextAlignment};

use crate::core::terms::term::RewritableLanguageOperatorSymbol;
use crate::rewriting_process::{context::RewritingProcessContextAndParameterization, step::RewriteStepKind};



pub fn get_step_node_inner_style_as_image_paragraph<LOS : RewritableLanguageOperatorSymbol>(
    context_and_param: &RewritingProcessContextAndParameterization<LOS>,
    step : &RewriteStepKind<LOS>,
    base_color : [u8;3],
    highlight_color : [u8;3],
) -> ColoredTextParagraph {
    let line = match step {
        RewriteStepKind::TransformInSamePhase(term_transformation_result) => {
            let phase = context_and_param.phases.get(term_transformation_result.phase_index).unwrap();
            let rule = phase.rules.get(term_transformation_result.rule_index_in_phase).unwrap();
            ColoredTextLine::new(
                vec![
                    (rule.get_desc(), Rgb(base_color)),
                    ("@".to_string(), Rgb(highlight_color)),
                    (format!("{:}", term_transformation_result.position), Rgb(base_color)),
                ]
            )
        },
        RewriteStepKind::GoToSuccessorPhase(changed) => {
            let mut line = vec![
                ("→phase→".to_string(), Rgb(base_color))
            ];
            if *changed {
                line.push(
                    ("⊤".to_string(), Rgb(highlight_color))
                );
            } else {
                line.push(
                    ("⊥".to_string(), Rgb(highlight_color))
                );
            }
            ColoredTextLine::new(line)
        },
    };
    ColoredTextParagraph::new(
        vec!(line),
        MultiLineTextAlignment::Center,
        None,
        None
    )
}