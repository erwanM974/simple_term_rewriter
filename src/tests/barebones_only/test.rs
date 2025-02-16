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


use graph_process_manager_core::{process::{filter::GenericFiltersManager, manager::GenericProcessManager}, queue::{priorities::GenericProcessPriorities, strategy::QueueSearchStrategy}};
use graph_process_manager_loggers::graphviz::{format::GraphVizProcessLoggerLayout, logger::{GenericGraphVizLogger, GenericGraphVizLoggerConfiguration}};
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::{core::term::LanguageTerm, process::{conf::RewriteConfig, context::{RewritingProcessContextAndParameterization, RewritingProcessPhase}, node::RewriteNodeKind, priorities::RewritePriorities}, tests::barebones_only::glog::{all_the_rest_drawer::MinimalRewritingStepDrawer, legend_writer::MinimalLegendWriter, node_drawer::MinimalRewritingNodeDrawer}};


use super::lang::{MinimalExampleLangOperators, MinimalExampleTransformationKind};


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

    let term = get_term_1();
    let phase = RewritingProcessPhase::new(
        vec![
            Box::new(MinimalExampleTransformationKind::DoubleNegation),
            Box::new(MinimalExampleTransformationKind::EvaluateNeg),
            Box::new(MinimalExampleTransformationKind::EvaluateAnd),
            Box::new(MinimalExampleTransformationKind::EvaluateOr),
        ],
        false
    );
    let context_and_param = RewritingProcessContextAndParameterization::new(vec![phase]);
    let graphviz_logger : GenericGraphVizLogger<RewriteConfig<MinimalExampleLangOperators>> = {
        let gv_conf = GenericGraphVizLoggerConfiguration::new(
            GraphVizOutputFormat::svg, 
            true, 
            "temp".to_string(), 
            "".to_string(), 
            "barebones".to_string()
        );
        GenericGraphVizLogger::new(
            gv_conf,
            Box::new(MinimalLegendWriter{}),
            vec![Box::new(MinimalRewritingNodeDrawer{})],
            Box::new(MinimalRewritingStepDrawer::new()),
            GraphVizProcessLoggerLayout::Vertical
        )
    };

    // ***

    let mut manager : GenericProcessManager<RewriteConfig<MinimalExampleLangOperators>> = GenericProcessManager::new(
        context_and_param,
        QueueSearchStrategy::DFS,
        GenericProcessPriorities::new(RewritePriorities::default(),false),
        GenericFiltersManager::new(
            vec![], 
            vec![], 
            vec![]
        ),
        vec![Box::new(graphviz_logger)],
        true
    );

    manager.start_process(RewriteNodeKind::new(term.clone(),0));

    let x = manager.global_state.irreducible_terms_per_phase.get(&0).unwrap();
    let result = x.first().unwrap();
    
    assert_eq!(result.operator, MinimalExampleLangOperators::TRUE);
}