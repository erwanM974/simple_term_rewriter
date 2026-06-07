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

use crate::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

use super::probas::TermGenerationSymbolsProbabilities;
use super::types::{
    RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration,
};

/// Controls when random term generation stops recursing.
pub struct RandomTermGenerationStopCriterion<CONF: RandomTermGenerationConfig> {
    /// Maximum allowed depth; once reached, `symbol_at_end` is used instead of sampling.
    pub max_depth: u32,
    /// The symbol (operator or pattern) forced at the leaf when `max_depth` is reached.
    /// Must be a nullary operator or a pattern that generates a leaf-sized sub-term.
    pub symbol_at_end: TermGenerationSymbol<CONF::LOS, CONF::PATTERN>,
}

impl<CONF: RandomTermGenerationConfig> RandomTermGenerationStopCriterion<CONF> {
    /// Creates the stop criterion with the given depth limit and terminal symbol.
    pub fn new(
        max_depth: u32,
        symbol_at_end: TermGenerationSymbol<CONF::LOS, CONF::PATTERN>,
    ) -> Self {
        Self {
            max_depth,
            symbol_at_end,
        }
    }
}

/// Generates a random term using the given probability distribution and stop criterion.
pub fn generate_random_term<CONF: RandomTermGenerationConfig>(
    probas: &TermGenerationSymbolsProbabilities<CONF>,
    stop_crit: &RandomTermGenerationStopCriterion<CONF>,
    context: &CONF::CONTEXT,
    rng: &mut StdRng,
    factory: &mut TermFactory<CONF::LOS>,
) -> LanguageTerm<CONF::LOS> {
    generate_random_term_rec(probas, 0, stop_crit, context, rng, factory)
}

fn generate_random_term_rec<CONF: RandomTermGenerationConfig>(
    probas: &TermGenerationSymbolsProbabilities<CONF>,
    depth: u32,
    stop_crit: &RandomTermGenerationStopCriterion<CONF>,
    context: &CONF::CONTEXT,
    rng: &mut StdRng,
    factory: &mut TermFactory<CONF::LOS>,
) -> LanguageTerm<CONF::LOS> {
    if depth >= stop_crit.max_depth {
        return match &stop_crit.symbol_at_end {
            TermGenerationSymbol::LanguageSymbol(s) => {
                assert!(s.arity() == LanguageOperatorArity::Fixed(0));
                LanguageTermNode::build(s.clone(), vec![], factory)
            }
            TermGenerationSymbol::Pattern(p) => p.generate_term_from_pattern(rng, context, factory),
        };
    }
    let symbol = probas.get_random_symbol(rng);
    match symbol {
        TermGenerationSymbol::LanguageSymbol(s) => {
            let n = match s.arity() {
                LanguageOperatorArity::Fixed(n) => n,
                LanguageOperatorArity::Variadic => {
                    panic!("variadic operators are not supported in random term generation")
                }
            };
            let mut sub_terms = vec![];
            for _ in 0..n {
                sub_terms.push(generate_random_term_rec(
                    probas,
                    depth + 1,
                    stop_crit,
                    context,
                    rng,
                    factory,
                ));
            }
            LanguageTermNode::build(s.clone(), sub_terms, factory)
        }
        TermGenerationSymbol::Pattern(p) => p.generate_term_from_pattern(rng, context, factory),
    }
}
