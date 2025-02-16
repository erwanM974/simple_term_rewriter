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
use crate::core::rule::RewriteRule;


pub struct RewritingProcessPhase<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    pub rules : Vec<Box<dyn RewriteRule<LanguageOperatorSymbol>>>,
    pub keep_only_one : bool
}

impl<LanguageOperatorSymbol: Clone + PartialEq + Eq + Hash> RewritingProcessPhase<LanguageOperatorSymbol> {
    pub fn new(
        rules: Vec<Box<dyn RewriteRule<LanguageOperatorSymbol>>>, 
        keep_only_one: bool
    ) -> Self {
        Self { rules, keep_only_one }
    }
}

pub struct RewritingProcessContextAndParameterization<LangOp : Clone + PartialEq + Eq + Hash> {
    pub phases : Vec<RewritingProcessPhase<LangOp>>
}

impl<LangOp: Clone + PartialEq + Eq + Hash> RewritingProcessContextAndParameterization<LangOp> {
    pub fn new(phases: Vec<RewritingProcessPhase<LangOp>>) -> Self {
        Self { phases }
    }
}

