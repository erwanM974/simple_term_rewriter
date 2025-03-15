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

use maplit::hashmap;
use rand::{rngs::StdRng, SeedableRng};

use crate::{core::terms::term::LanguageTerm, random_term_generation::{gen::{generate_random_term, RandomTermGenerationStopCriterion}, probas::TermGenerationSymbolsProbabilities, types::{RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration}}};

use super::lang::SimplisticBooleanLogicOperators;



#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimplisticBooleanLogicFormulaePattern {}

impl TermPatternForRandomGeneration<SimplisticBooleanLogicFormulaeGenerationConfig> for SimplisticBooleanLogicFormulaePattern {
    fn generate_term_from_pattern(
        &self,
        _rng : &mut rand::prelude::StdRng,
        _context : &()
    ) -> LanguageTerm<SimplisticBooleanLogicOperators> {
        LanguageTerm::new(
            SimplisticBooleanLogicOperators::TRUE, 
            vec![]
        )
    }
}
pub struct SimplisticBooleanLogicFormulaeGenerationConfig {}

impl RandomTermGenerationConfig for SimplisticBooleanLogicFormulaeGenerationConfig {
    type LOS = SimplisticBooleanLogicOperators;

    type CONTEXT = ();

    type PATTERN = SimplisticBooleanLogicFormulaePattern;

    fn get_arity(op : &Self::LOS) -> usize {
        match op {
            SimplisticBooleanLogicOperators::TRUE => 0,
            SimplisticBooleanLogicOperators::FALSE => 0,
            SimplisticBooleanLogicOperators::OR => 2,
            SimplisticBooleanLogicOperators::AND => 2,
            SimplisticBooleanLogicOperators::NEG => 1,
        }
    }
}



fn generate_boolean_formulae(
    number_of_formulae : usize
) -> Vec<LanguageTerm<SimplisticBooleanLogicOperators>> {

    let probas : TermGenerationSymbolsProbabilities<SimplisticBooleanLogicFormulaeGenerationConfig> = TermGenerationSymbolsProbabilities::from_map(
        hashmap! {
            TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::TRUE) => 0.1,
            TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::FALSE) => 0.1,
            TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::AND) => 0.3,
            TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::FALSE) => 0.3,
            TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::NEG) => 0.2
        }
    ).unwrap();

    let stop_crit = RandomTermGenerationStopCriterion::new(
        30,
        TermGenerationSymbol::LanguageSymbol(SimplisticBooleanLogicOperators::TRUE)
    );

    let mut rng = StdRng::seed_from_u64(0);

    let mut formulae = vec![];
    for _ in 0..number_of_formulae {
        formulae.push(
            generate_random_term(
                &probas,
                &stop_crit,
                &(),
                &mut rng
            )
        )
    }
    formulae
}










