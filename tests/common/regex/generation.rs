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

use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;

use simple_term_rewriter::random_term_generation::{
    gen::{generate_random_term, RandomTermGenerationStopCriterion},
    probas::TermGenerationSymbolsProbabilities,
    types::{RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration},
};
use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

use super::lang::RegexOp;

/// Dummy pattern — never instantiated; required by the trait bound on
/// [`RandomTermGenerationConfig`] but unused because the stop criterion
/// and the probability map use only [`TermGenerationSymbol::LanguageSymbol`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NeverPattern;

impl TermPatternForRandomGeneration<RegexGenerationConfig> for NeverPattern {
    fn generate_term_from_pattern(
        &self,
        _rng: &mut StdRng,
        _ctx: &(),
        _factory: &mut TermFactory<RegexOp>,
    ) -> LanguageTerm<RegexOp> {
        unreachable!("NeverPattern should never be called")
    }
}

pub struct RegexGenerationConfig;

impl RandomTermGenerationConfig for RegexGenerationConfig {
    type LOS = RegexOp;
    type CONTEXT = ();
    type PATTERN = NeverPattern;
}

/// Generate `count` random regex terms using a fixed `seed` for reproducibility.
///
/// Probabilities: Empty 5 %, Epsilon 10 %, Atom('a') 10 %, Atom('b') 5 %,
/// Alt 25 %, Concat 25 %, Star 20 %.  Max depth 5; leaves are `Atom('a')`.
pub fn generate_regex_terms(
    count: usize,
    seed: u64,
    f: &mut TermFactory<RegexOp>,
) -> Vec<LanguageTerm<RegexOp>> {
    let mut map: HashMap<TermGenerationSymbol<RegexOp, NeverPattern>, f32> = HashMap::new();
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Empty), 0.05);
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Epsilon), 0.10);
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        0.10,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'b')),
        0.05,
    );
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Alt), 0.25);
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Concat), 0.25);
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Star), 0.20);

    let probas = TermGenerationSymbolsProbabilities::from_map(map).unwrap();
    let stop_crit = RandomTermGenerationStopCriterion::<RegexGenerationConfig>::new(
        5,
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
    );
    let mut rng = StdRng::seed_from_u64(seed);

    (0..count)
        .map(|_| generate_random_term(&probas, &stop_crit, &(), &mut rng, f))
        .collect()
}
