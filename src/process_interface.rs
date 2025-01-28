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



use graph_process_manager_core::{delegate::{delegate::GenericProcessDelegate, priorities::GenericProcessPriorities}, handler::filter::AbstractFilter, manager::{logger::AbstractProcessLogger, manager::GenericProcessManager}, queued_steps::queue::strategy::QueueSearchStrategy};

use crate::{core::{interface::SimpleTermRewritingInterface, term::LanguageTerm}, process::{conf::RewriteConfig, context::RewriteContext, filter::{elim::RewriteFilterEliminationKind, filter::RewriteFilterCriterion}, node::RewriteNodeKind, param::RewriteParameterization, priorities::RewritePriorities, step::RewriteStepKind}};





pub fn rewrite_term<STRI : SimpleTermRewritingInterface + 'static>(
    term : &LanguageTerm<STRI::LanguageOperator>,
    strategy : QueueSearchStrategy,
    priorities : GenericProcessPriorities<RewritePriorities<STRI::TransformationKind>>,
    param : RewriteParameterization<STRI>,
    filters : Vec<Box<dyn AbstractFilter<RewriteFilterCriterion,RewriteFilterEliminationKind>>>,
    loggers : Vec<Box< dyn AbstractProcessLogger<RewriteConfig<STRI>>>>
) -> LanguageTerm<STRI::LanguageOperator> {

    let rewrite_ctx = RewriteContext{};
    let delegate : GenericProcessDelegate<RewriteStepKind<STRI>,RewriteNodeKind<STRI::LanguageOperator>,RewritePriorities<STRI::TransformationKind>> =
        GenericProcessDelegate::new(
            strategy,
            priorities
        );

    let mut rewrite_manager : GenericProcessManager<RewriteConfig<STRI>> = GenericProcessManager::new(
        rewrite_ctx,
        param,
        delegate,
        filters,
        loggers,
        None,
        true
    );
    // ***
    let init_node = RewriteNodeKind::new(term.clone(),0);
    // ***
    let (_,mut verdict) = rewrite_manager.start_process(init_node);
    if verdict.normalized_terms.len() != 1 {
        println!("warning got {:} differents canonized interactions", verdict.normalized_terms.len());
    }
    verdict.normalized_terms.remove(0)
}