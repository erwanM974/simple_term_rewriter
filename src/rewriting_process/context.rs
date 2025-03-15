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



use crate::core::rule::RewriteRule;
use crate::core::terms::term::RewritableLanguageOperatorSymbol;


/** 
 * An abstract rewriting phase that specifies how input terms may be rewritten via a classical TRS.
 * It is abstract as it can be concretized several times.
 * Once that term is rewritten and is irreducible, it may be forwarded to another rewriting phase.
 * The identifier of these target "successor" phases are given in two cases:
 * - in the case some rewriting has been performed and the irreducible term is different from the initial
 * - and in the other case 
 * **/
pub struct AbstractRewritingPhase<LOS : RewritableLanguageOperatorSymbol> {
    pub rules : Vec<Box<dyn RewriteRule<LOS>>>,
    pub next_phase_id_on_changed : Option<usize>,
    pub next_phase_id_on_unchanged : Option<usize>
}

impl<LOS : RewritableLanguageOperatorSymbol> AbstractRewritingPhase<LOS> {
    pub fn new(
        rules: Vec<Box<dyn RewriteRule<LOS>>>, 
        next_phase_id_on_changed : Option<usize>,
        next_phase_id_on_unchanged : Option<usize>
    ) -> Self {
        Self { rules, next_phase_id_on_changed, next_phase_id_on_unchanged }
    }
}






pub struct RewritingProcessContextAndParameterization<LOS : RewritableLanguageOperatorSymbol> {
    pub phases : Vec<AbstractRewritingPhase<LOS>>,
    pub keep_only_one : bool,
}

impl<LOS : RewritableLanguageOperatorSymbol> RewritingProcessContextAndParameterization<LOS> {
    pub fn new(
        phases : Vec<AbstractRewritingPhase<LOS>>,
        keep_only_one : bool,
    ) -> Self {
        Self { phases, keep_only_one }
    }
}

