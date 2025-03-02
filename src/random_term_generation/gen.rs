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

use rand::prelude::StdRng;

use crate::core::term::LanguageTerm;

use super::probas::TermGenerationSymbolsProbabilities;
use super::types::{RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration};



pub struct RandomTermGenerationStopCriterion<CONF : RandomTermGenerationConfig> {
    pub max_depth : u32,
    pub symbol_at_end : TermGenerationSymbol<CONF::LOS,CONF::PATTERN>
}

impl<CONF : RandomTermGenerationConfig>  RandomTermGenerationStopCriterion<CONF> {
    pub fn new(max_depth : u32,symbol_at_end : TermGenerationSymbol<CONF::LOS,CONF::PATTERN>) -> Self {
        Self { max_depth, symbol_at_end }
    }
}


pub fn generate_random_term<CONF : RandomTermGenerationConfig>(
    probas : &TermGenerationSymbolsProbabilities<CONF>,
    depth : u32,
    stop_crit : &RandomTermGenerationStopCriterion<CONF>,
    context : &CONF::CONTEXT,
    rng : &mut StdRng
) -> LanguageTerm<CONF::LOS> {
    if depth >= stop_crit.max_depth {
        return match &stop_crit.symbol_at_end {
            TermGenerationSymbol::LanguageSymbol(s) => {
                assert!(CONF::get_arity(s) == 0);
                LanguageTerm::new(s.clone(), vec![])
            },
            TermGenerationSymbol::Pattern(p) => {
                p.generate_term_from_pattern(rng, context)
            },
        };
    }
    let symbol = probas.get_random_symbol(rng);
    match symbol {
        TermGenerationSymbol::LanguageSymbol(s) => {
            let mut sub_terms = vec![];
            for _ in 0..CONF::get_arity(&s) {
                sub_terms.push( 
                    generate_random_term(probas,depth+1,stop_crit,context,rng)
                );
            }
            LanguageTerm::new(s.clone(), sub_terms)
        },
        TermGenerationSymbol::Pattern(p) => {
            p.generate_term_from_pattern(rng, context)
        },
    }
}


