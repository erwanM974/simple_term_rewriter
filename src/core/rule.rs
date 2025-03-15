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



use crate::core::terms::{position::PositionInLanguageTerm, term::{LanguageTerm, RewritableLanguageOperatorSymbol}};





/** 
 * A rewrite rule that may be applied at the root position of a term.
 * **/
pub trait RewriteRule<LOS : RewritableLanguageOperatorSymbol> {

    /** 
     * Returns a description this rewrite rule.
     * **/
    fn get_desc(&self) -> String;

    /** 
     * If the rule is applicable on the given term, then it returns the result of its application.
     * Additional contextual information is given by "context_term" and "position_in_context_term"
     * **/
    fn try_apply(
        &self,
        term : &LanguageTerm<LOS>,
        context_term : &LanguageTerm<LOS>,
        position_in_context_term : &PositionInLanguageTerm
    ) -> Option<LanguageTerm<LOS>>;

}






