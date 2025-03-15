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
use maplit::hashset;

use crate::{core::terms::term::LanguageTerm, rewriting_process::context::AbstractRewritingPhase, tests::{barebones_only::rules::MinimalExampleTransformationKind, boolean_logic::lang::SimplisticBooleanLogicOperators}};
use crate::rewriting_process::conf::RewriteConfig;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::priorities::RewritePriorities; 
use crate::tests::barebones_only::glog::{all_the_rest_drawer::MinimalRewritingStepDrawer, legend_writer::MinimalLegendWriter, node_drawer::MinimalRewritingNodeDrawer};




pub fn get_term_1() -> LanguageTerm<SimplisticBooleanLogicOperators> {

    LanguageTerm::new(
        SimplisticBooleanLogicOperators::AND, 
        vec![
            LanguageTerm::new(
                SimplisticBooleanLogicOperators::NEG,
                vec![
                    LanguageTerm::new(
                        SimplisticBooleanLogicOperators::NEG,
                        vec![
                            LanguageTerm::new(
                                SimplisticBooleanLogicOperators::TRUE,
                                vec![]
                            )
                        ]
                    )
                ]
            ),
            LanguageTerm::new(
                SimplisticBooleanLogicOperators::OR,
                vec![
                    LanguageTerm::new(
                        SimplisticBooleanLogicOperators::AND,
                        vec![
                            LanguageTerm::new(
                                SimplisticBooleanLogicOperators::FALSE,
                                vec![]
                            ),
                            LanguageTerm::new(
                                SimplisticBooleanLogicOperators::FALSE,
                                vec![]
                            ),
                        ]
                    ),
                    LanguageTerm::new(
                        SimplisticBooleanLogicOperators::TRUE,
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
    let phase = AbstractRewritingPhase::new(
        vec![
            Box::new(MinimalExampleTransformationKind::DoubleNegation),
            Box::new(MinimalExampleTransformationKind::EvaluateNeg),
            Box::new(MinimalExampleTransformationKind::EvaluateAnd),
            Box::new(MinimalExampleTransformationKind::EvaluateOr),
        ],
        None,
        None
    );
    let context_and_param = RewritingProcessContextAndParameterization::new(
        vec![phase],
        false
    );
    let graphviz_logger : GenericGraphVizLogger<RewriteConfig<SimplisticBooleanLogicOperators>> = {
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

    let mut manager : GenericProcessManager<RewriteConfig<SimplisticBooleanLogicOperators>> = GenericProcessManager::new(
        context_and_param,
        QueueSearchStrategy::DFS,
        GenericProcessPriorities::new(RewritePriorities{},false),
        GenericFiltersManager::new(
            vec![], 
            vec![], 
            vec![]
        ),
        vec![Box::new(graphviz_logger)],
        true,
        RewriteNodeKind::new(term.clone(),0)
    );

    manager.start_process();

    let irreducible_terms = &manager.global_state.concrete_phases.last().unwrap().final_irreducible_terms;
    
    let expected_term = LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE,vec![]);
    assert_eq!(irreducible_terms, &hashset!{expected_term});
}