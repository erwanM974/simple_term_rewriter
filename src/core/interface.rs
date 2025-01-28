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
use std::fmt;





pub trait SimpleTermRewritingInterface {

    type LanguageOperator : Clone + PartialEq + Eq + Hash;

    type TransformationKind : Clone + PartialEq + Eq + Hash + std::fmt::Display;

}




/** 
 * An object that keeps track of positions in a concrete 
 * term of thet Language which we are considering.
 * **/
 #[derive(Clone, PartialEq, Eq, Hash)]
pub enum PositionInLanguageTerm {
    Root,
    Child(usize,Box<PositionInLanguageTerm>)
}


impl PositionInLanguageTerm {
    /** 
     * Get position epsilon i.e. that of the toot of the term.
     * **/
     pub fn get_root_position() -> Self {
        PositionInLanguageTerm::Root
     }

     /** 
      * If x is at position p in y, Then x is at position:
      * - 1.p in z(y,.,.)
      * - 2.p in z(.,y,.)
      * - etc
      * **/
      pub fn position_as_nth_sub_term(self, n : usize) -> Self {
        PositionInLanguageTerm::Child(n, Box::new(self))
     }

}


impl fmt::Display for PositionInLanguageTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionInLanguageTerm::Root => {
                write!(f,"ε")
            },
            PositionInLanguageTerm::Child(ref n, ref sub_pos) => {
                write!(f,"{:}_{:}",(n+1),sub_pos)
            }
        }
    }
}


