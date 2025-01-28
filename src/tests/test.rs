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


use std::path::PathBuf;

use graph_process_manager_core::{delegate::priorities::GenericProcessPriorities, queued_steps::queue::strategy::QueueSearchStrategy};
use graph_process_manager_loggers::graphviz::{format::GraphVizProcessLoggerLayout, logger::GenericGraphVizLogger};
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::{core::{rule::RewriteRule, term::LanguageTerm}, process::{conf::RewriteConfig, param::RewriteParameterization, priorities::RewritePriorities}, process_interface::rewrite_term, tests::glog::drawer::MinimalRewritingProcessDrawer};

use super::lang::{MinimalExampleInterface, MinimalExampleLangOperators, MinimalExampleTransformationKind};


pub fn get_term_1() -> LanguageTerm<MinimalExampleLangOperators> {

    LanguageTerm::new(
        MinimalExampleLangOperators::AND, 
        vec![
            LanguageTerm::new(
                MinimalExampleLangOperators::NEG,
                vec![
                    LanguageTerm::new(
                        MinimalExampleLangOperators::NEG,
                        vec![
                            LanguageTerm::new(
                                MinimalExampleLangOperators::TRUE,
                                vec![]
                            )
                        ]
                    )
                ]
            ),
            LanguageTerm::new(
                MinimalExampleLangOperators::OR,
                vec![
                    LanguageTerm::new(
                        MinimalExampleLangOperators::AND,
                        vec![
                            LanguageTerm::new(
                                MinimalExampleLangOperators::FALSE,
                                vec![]
                            ),
                            LanguageTerm::new(
                                MinimalExampleLangOperators::FALSE,
                                vec![]
                            ),
                        ]
                    ),
                    LanguageTerm::new(
                        MinimalExampleLangOperators::TRUE,
                        vec![]
                    )
                ] 
            )
        ]
    )
}





#[test]
pub fn test() {


    let res_buf : PathBuf = [".", "res"].iter().collect();
    let temp_buf : PathBuf = [".", "temp"].iter().collect();

    let term = get_term_1();
    let phase : Vec<Box<dyn RewriteRule<MinimalExampleInterface>>> = vec![
        Box::new(MinimalExampleTransformationKind::DoubleNegation),
        Box::new(MinimalExampleTransformationKind::EvaluateNeg),
        Box::new(MinimalExampleTransformationKind::EvaluateAnd),
        Box::new(MinimalExampleTransformationKind::EvaluateOr),
    ];
    let param = RewriteParameterization::new(
vec![phase],
false
    );
    let drawer = MinimalRewritingProcessDrawer::new(temp_buf.into_os_string().into_string().unwrap());
    let graphviz_logger : GenericGraphVizLogger<RewriteConfig<MinimalExampleInterface>> = GenericGraphVizLogger::new(
        Box::new(drawer),
        GraphVizOutputFormat::svg,
        GraphVizProcessLoggerLayout::Vertical,
        true,
        res_buf.clone().into_os_string().into_string().unwrap(),
        format!("rewrite"));
    let result = rewrite_term::<MinimalExampleInterface>(
        &term,
        QueueSearchStrategy::BFS,
        GenericProcessPriorities::new(RewritePriorities::default(),false),
        param,
        vec![],
        vec![Box::new(graphviz_logger)]
    );
    assert_eq!(result.operator, MinimalExampleLangOperators::TRUE);
}