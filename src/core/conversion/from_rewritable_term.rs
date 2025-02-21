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




use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};


pub trait FromRewritableTermToDomainSpecificTerm<LOS : RewritableLanguageOperatorSymbol> : Sized + Clone {

     fn instantiate_term_under_operator(
        operator : &LOS, 
        sub_terms : &mut Vec<Self>
    ) -> Self;

     /** 
      * Conversion from this crate's rewritable term language to the domain specific (outside of this crate) term language.
      * **/
     fn from_rewritable_term(rewritable_term : &LanguageTerm<LOS>) -> Self {
        let mut sub_terms = vec![];
        for rewr_sub_term in &rewritable_term.sub_terms {
            sub_terms.push(Self::from_rewritable_term(rewr_sub_term));
        }
        Self::instantiate_term_under_operator(
            &rewritable_term.operator, 
            &mut sub_terms
        )
     }

}








