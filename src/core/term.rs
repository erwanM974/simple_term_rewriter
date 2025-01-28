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




/** 
 * A concrete term in the Language which we are considering.
 * **/
 #[derive(Clone, PartialEq, Eq, Hash)]
pub struct LanguageTerm<LanguageOperator : Clone + PartialEq + Eq + Hash> {
    pub operator : LanguageOperator,
    pub sub_terms : Vec<LanguageTerm<LanguageOperator>>
}


impl<LanguageOperator : Clone + PartialEq + Eq + Hash> LanguageTerm<LanguageOperator> {

    pub fn new(operator : LanguageOperator,sub_terms : Vec<LanguageTerm<LanguageOperator>>) -> Self {
        Self{operator,sub_terms}
    }

}








