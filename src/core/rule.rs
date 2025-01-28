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


use super::{interface::SimpleTermRewritingInterface, term::LanguageTerm};





/** 
 * A rewrite rule that may be applied at the root position of a term.
 * **/
pub trait RewriteRule<STRI : SimpleTermRewritingInterface> : std::fmt::Display {

    /** 
     * Returns an object that describes this rewrite rule.
     * **/
    fn get_transformation_kind(&self) -> STRI::TransformationKind;

    /** 
     * If the rule is applicable at the root position of the given term, then it returns the result of its application.
     * **/
    fn try_apply(&self, term : &LanguageTerm<STRI::LanguageOperator>) -> Option<LanguageTerm<STRI::LanguageOperator>>;

}


/** 
 * A predicate that may hold or not on terms of the language which we are considering.
 * **/
pub trait PredicateOnTerm<STRI : SimpleTermRewritingInterface> : std::fmt::Display {

    fn term_satisfies(&self, term : &LanguageTerm<STRI::LanguageOperator>) -> bool;

}




