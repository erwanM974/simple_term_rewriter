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

use crate::core::term::LanguageTerm;


pub trait FromDomainSpecificTermToRewritableTerm<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> : Sized + Clone {

    /** 
     * Returns the operator at the root of the domain specific term.
     * **/
    fn get_operator_at_root(&self) -> LanguageOperatorSymbol;

    /** 
     * Returns the sub-terms of the domain specific term.
     * **/
     fn get_subterms<'a>(&'a self) -> Vec<&'a Self>;

    /** 
     * Conversion from the domain specific (outside of this crate) term language to this crate's rewritable term language.
     * **/
     fn to_rewritable_term(&self) -> LanguageTerm<LanguageOperatorSymbol> {
        let mut sub_terms = vec![];
        for domain_specific_sub_term in self.get_subterms() {
            sub_terms.push(
                domain_specific_sub_term.to_rewritable_term()
            );
        }
        LanguageTerm::new(
            self.get_operator_at_root(), 
            sub_terms
        )
     }

}















