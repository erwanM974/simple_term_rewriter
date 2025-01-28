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
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::core::interface::SimpleTermRewritingInterface;
use crate::process::context::RewriteContext;
use crate::process::filter::elim::RewriteFilterEliminationKind;
use crate::process::filter::filter::RewriteFilterCriterion;
use crate::process::handling::handler::RewriteProcessHandler;
use crate::process::node::RewriteNodeKind;
use crate::process::param::RewriteParameterization;
use crate::process::priorities::RewritePriorities;
use crate::process::step::RewriteStepKind;
use crate::process::verdict::global::RewriteGlobalVerdict;
use crate::process::verdict::local::RewriteLocalVerdict;


pub struct RewriteConfig<STRI : SimpleTermRewritingInterface> {
    phantom: PhantomData<STRI>
}

pub struct RewriteStaticLocalVerdictAnalysisProof{}

impl<STRI : SimpleTermRewritingInterface> AbstractProcessConfiguration for RewriteConfig<STRI> {
    type Context = RewriteContext;
    type Parameterization = RewriteParameterization<STRI>;
    type NodeKind = RewriteNodeKind<STRI::LanguageOperator>;
    type StepKind = RewriteStepKind<STRI>;
    type Priorities = RewritePriorities<STRI::TransformationKind>;
    type FilterCriterion = RewriteFilterCriterion;
    type FilterEliminationKind = RewriteFilterEliminationKind;
    type LocalVerdict = RewriteLocalVerdict<STRI::LanguageOperator>;
    type StaticLocalVerdictAnalysisProof = RewriteStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = RewriteGlobalVerdict<STRI::LanguageOperator>;
    type ProcessHandler = RewriteProcessHandler;
}
