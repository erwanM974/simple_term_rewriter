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
use rand::rngs::StdRng;

use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};




pub trait RandomTermGenerationConfig : Sized {

    type LOS : RewritableLanguageOperatorSymbol;

    type CONTEXT;

    type PATTERN : TermPatternForRandomGeneration<Self>;

    fn get_arity(op : &Self::LOS) -> usize;

}


pub trait TermPatternForRandomGeneration<CONF : RandomTermGenerationConfig> : Clone + PartialEq + Eq + Hash {

    fn generate_term_from_pattern(
        &self,
        rng : &mut StdRng,
        context : &CONF::CONTEXT
    ) -> LanguageTerm<CONF::LOS>;

}





#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TermGenerationSymbol<LOS,PATTERN> {
    LanguageSymbol(LOS),
    Pattern(PATTERN)
}




