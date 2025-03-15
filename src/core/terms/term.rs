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
use std::fmt::Debug;

use super::position::PositionInLanguageTerm;

pub trait RewritableLanguageOperatorSymbol : Clone + PartialEq + Eq + Hash + Debug + 'static {}



/** 
 * A concrete term in the Language which we are considering.
 * **/
 #[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LanguageTerm<LOS : RewritableLanguageOperatorSymbol> {
    pub operator : LOS,
    pub sub_terms : Vec<LanguageTerm<LOS>>
}


impl<LOS : RewritableLanguageOperatorSymbol> LanguageTerm<LOS> {

    pub fn new(operator : LOS,sub_terms : Vec<LanguageTerm<LOS>>) -> Self {
        Self{operator,sub_terms}
    }

    pub fn get_sub_term_at_position<'a>(&'a self, pos : &PositionInLanguageTerm) -> Option<&'a Self> {
        self.get_sub_term_at_position_rec(pos.get_absolute_coordinates_from_root())
    }

    fn get_sub_term_at_position_rec<'a>(&'a self, abs_pos : &[usize]) -> Option<&'a Self> {
        if abs_pos.is_empty() {
            Some(self)
        } else {
            let child_index = abs_pos.first().unwrap();
            match self.sub_terms.get(*child_index) {
                None => {
                    None 
                },
                Some(sub_term) => {
                    sub_term.get_sub_term_at_position_rec(&abs_pos[1..])
                }
            }
        }
    }

}








