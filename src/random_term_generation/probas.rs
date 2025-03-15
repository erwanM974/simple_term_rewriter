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


use std::collections::HashMap;
use rand::prelude::StdRng;
use rand::Rng;

use super::types::{RandomTermGenerationConfig, TermGenerationSymbol};




pub struct TermGenerationSymbolsProbabilities<CONF : RandomTermGenerationConfig> {
    pub ordered_symbols : Vec<TermGenerationSymbol<CONF::LOS,CONF::PATTERN>>,
    pub ordered_bounds : Vec<f32>
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum InteractionSymbolsProbabilitiesError {
    SymbolProbabilityMustBeBetweenOAnd1,
    SumOfProbabilitiesMustBe1
}


impl<CONF : RandomTermGenerationConfig> TermGenerationSymbolsProbabilities<CONF> {

    pub fn from_map(map : HashMap<TermGenerationSymbol<CONF::LOS,CONF::PATTERN>,f32>) -> Result<Self,InteractionSymbolsProbabilitiesError> {
        let mut ordered_symbols = vec![];
        let mut ordered_bounds = vec![0.0_f32];
        let mut sum = 0.0;
        for (s,p) in map {
            if p < 0.0 - 1e-6 || p > 1.0 + 1e-6 {
                return Err(InteractionSymbolsProbabilitiesError::SymbolProbabilityMustBeBetweenOAnd1);
            }
            ordered_symbols.push(s);
            sum += p;
            ordered_bounds.push(sum);
        }
        if sum < 1.0-1e-6 || sum > 1.0 +1e-6 {
            return Err(InteractionSymbolsProbabilitiesError::SumOfProbabilitiesMustBe1);
        }
        assert!(ordered_bounds.len() == ordered_symbols.len() +1);
        // ***
        Ok(Self{ordered_symbols,ordered_bounds})
    }

    pub fn get_random_symbol(&self, rng : &mut StdRng) -> TermGenerationSymbol<CONF::LOS,CONF::PATTERN> {
        let got = rng.random_range(0.0_f32..1.0_f32);
        for (idx,x) in self.ordered_bounds.iter().enumerate() {
            if got <= *x + 1e-6 {
                if idx == 0 {
                    return self.ordered_symbols.first().unwrap().clone();
                } else {
                    return self.ordered_symbols.get(idx-1).unwrap().clone();
                }
            }
        }
        panic!()
    }
}



