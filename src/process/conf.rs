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

use std::hash::Hash;
use std::marker::PhantomData;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use crate::process::context::RewritingProcessContextAndParameterization;
use crate::process::filtration::RewritingFiltrationResult;
use crate::process::handler::RewriteProcessHandler;
use crate::process::node::RewriteNodeKind;
use crate::process::priorities::RewritePriorities;
use crate::process::state::RewritingProcessState;
use crate::process::step::RewriteStepKind;


pub struct RewriteConfig<LangOp: Clone + PartialEq + Eq + Hash> {
    phantom: PhantomData<LangOp>
}

impl<LangOp: Clone + PartialEq + Eq + Hash> AbstractProcessConfiguration for RewriteConfig<LangOp> {
    type ContextAndParameterization = RewritingProcessContextAndParameterization<LangOp>;
    // ***
    type AlgorithmOperationHandler = RewriteProcessHandler;
    // ***
    type DomainSpecificNode = RewriteNodeKind<LangOp>;
    type DomainSpecificStep = RewriteStepKind<LangOp>;
    type Priorities = RewritePriorities;
    // ***
    type MutablePersistentState = RewritingProcessState<LangOp>;
    // ***
    type FiltrationResult = RewritingFiltrationResult;
}
