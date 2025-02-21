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



use crate::core::{rule::RewriteRule, term::RewritableLanguageOperatorSymbol};


pub struct RewritingProcessPhase<LOS : RewritableLanguageOperatorSymbol> {
    pub rules : Vec<Box<dyn RewriteRule<LOS>>>,
    pub keep_only_one : bool
}

impl<LOS : RewritableLanguageOperatorSymbol> RewritingProcessPhase<LOS> {
    pub fn new(
        rules: Vec<Box<dyn RewriteRule<LOS>>>, 
        keep_only_one: bool
    ) -> Self {
        Self { rules, keep_only_one }
    }
}

pub struct RewritingProcessContextAndParameterization<LOS : RewritableLanguageOperatorSymbol> {
    pub phases : Vec<RewritingProcessPhase<LOS>>
}

impl<LOS : RewritableLanguageOperatorSymbol> RewritingProcessContextAndParameterization<LOS> {
    pub fn new(phases: Vec<RewritingProcessPhase<LOS>>) -> Self {
        Self { phases }
    }
}

