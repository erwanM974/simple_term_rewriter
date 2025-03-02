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

use std::marker::PhantomData;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use crate::core::term::RewritableLanguageOperatorSymbol;
use crate::rewriting_process::context::RewritingProcessContextAndParameterization;
use crate::rewriting_process::filtration::RewritingFiltrationResult;
use crate::rewriting_process::handler::RewriteProcessHandler;
use crate::rewriting_process::node::RewriteNodeKind;
use crate::rewriting_process::priorities::RewritePriorities;
use crate::rewriting_process::state::RewritingProcessState;
use crate::rewriting_process::step::RewriteStepKind;


pub struct RewriteConfig<LOS : RewritableLanguageOperatorSymbol> {
    phantom: PhantomData<LOS>
}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractProcessConfiguration for RewriteConfig<LOS> {
    type ContextAndParameterization = RewritingProcessContextAndParameterization<LOS>;
    // ***
    type AlgorithmOperationHandler = RewriteProcessHandler;
    // ***
    type DomainSpecificNode = RewriteNodeKind<LOS>;
    type DomainSpecificStep = RewriteStepKind<LOS>;
    type Priorities = RewritePriorities;
    // ***
    type MutablePersistentState = RewritingProcessState<LOS>;
    // ***
    type FiltrationResult = RewritingFiltrationResult;
}
