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
use rand::RngExt;
use std::collections::HashMap;

use super::types::{RandomTermGenerationConfig, TermGenerationSymbol};

/// A discrete probability distribution over [`TermGenerationSymbol`]s, used to
/// drive random term generation.
pub struct TermGenerationSymbolsProbabilities<CONF: RandomTermGenerationConfig> {
    /// The symbols in the order they were inserted.
    pub ordered_symbols: Vec<TermGenerationSymbol<CONF::LOS, CONF::PATTERN>>,
    /// Cumulative probability bounds; `ordered_bounds[i]` is the upper bound for
    /// `ordered_symbols[i - 1]`.  Length is `ordered_symbols.len() + 1`.
    pub ordered_bounds: Vec<f32>,
}

/// Errors returned by [`TermGenerationSymbolsProbabilities::from_map`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum InteractionSymbolsProbabilitiesError {
    /// A symbol was assigned a probability outside `[0.0, 1.0]`.
    SymbolProbabilityMustBeBetweenOAnd1,
    /// The probabilities do not sum to 1.0 (within tolerance).
    SumOfProbabilitiesMustBe1,
}

impl<CONF: RandomTermGenerationConfig> TermGenerationSymbolsProbabilities<CONF> {
    /// Builds the distribution from a `{symbol → probability}` map.
    ///
    /// Returns an error if any probability is outside `[0, 1]` or if the total
    /// does not sum to 1.0 within a tolerance of 1e-6.
    pub fn from_map(
        map: HashMap<TermGenerationSymbol<CONF::LOS, CONF::PATTERN>, f32>,
    ) -> Result<Self, InteractionSymbolsProbabilitiesError> {
        let mut ordered_symbols = vec![];
        let mut ordered_bounds = vec![0.0_f32];
        let mut sum = 0.0;
        for (s, p) in map {
            if !(0.0 - 1e-6..=1.0 + 1e-6).contains(&p) {
                return Err(
                    InteractionSymbolsProbabilitiesError::SymbolProbabilityMustBeBetweenOAnd1,
                );
            }
            ordered_symbols.push(s);
            sum += p;
            ordered_bounds.push(sum);
        }
        if !(1.0 - 1e-6..=1.0 + 1e-6).contains(&sum) {
            return Err(InteractionSymbolsProbabilitiesError::SumOfProbabilitiesMustBe1);
        }
        assert!(ordered_bounds.len() == ordered_symbols.len() + 1);
        // ***
        Ok(Self {
            ordered_symbols,
            ordered_bounds,
        })
    }

    /// Samples one symbol from the distribution using `rng`.
    pub fn get_random_symbol(
        &self,
        rng: &mut StdRng,
    ) -> TermGenerationSymbol<CONF::LOS, CONF::PATTERN> {
        let got = rng.random_range(0.0_f32..1.0_f32);
        for (idx, x) in self.ordered_bounds.iter().enumerate() {
            if got <= *x + 1e-6 {
                if idx == 0 {
                    return self.ordered_symbols.first().unwrap().clone();
                } else {
                    return self.ordered_symbols.get(idx - 1).unwrap().clone();
                }
            }
        }
        panic!()
    }
}
