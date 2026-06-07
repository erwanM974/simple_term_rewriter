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

//! Benchmark: random term generation throughput.
//!
//! Measures the time to generate a batch of terms from a fixed probability
//! distribution, parameterized by batch size and max depth.
//!
//! The RNG is re-seeded at the start of each criterion iteration so that each
//! measurement covers exactly the same sequence of terms, making results
//! directly comparable across runs.

use std::collections::HashMap;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hashconsing::HConsign;
use rand::{rngs::StdRng, SeedableRng};

use simple_term_rewriter::random_term_generation::{
    gen::{generate_random_term, RandomTermGenerationStopCriterion},
    probas::TermGenerationSymbolsProbabilities,
    types::{RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration},
};
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory,
};

// == language ==================================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RegexOp {
    Atom(u8),
    Alt,
    Concat,
    Star,
}

impl RewritableLanguageOperatorSymbol for RegexOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            RegexOp::Atom(_) => LanguageOperatorArity::Fixed(0),
            RegexOp::Alt | RegexOp::Concat => LanguageOperatorArity::Fixed(2),
            RegexOp::Star => LanguageOperatorArity::Fixed(1),
        }
    }
}

// == generation config =========================================================

#[derive(Clone, PartialEq, Eq, Hash)]
struct NeverPattern;

impl TermPatternForRandomGeneration<GenConfig> for NeverPattern {
    fn generate_term_from_pattern(
        &self,
        _: &mut StdRng,
        _: &(),
        _: &mut TermFactory<RegexOp>,
    ) -> LanguageTerm<RegexOp> {
        unreachable!()
    }
}

struct GenConfig;

impl RandomTermGenerationConfig for GenConfig {
    type LOS = RegexOp;
    type CONTEXT = ();
    type PATTERN = NeverPattern;
}

// == helper ====================================================================

fn make_setup(
    max_depth: u32,
) -> (
    TermGenerationSymbolsProbabilities<GenConfig>,
    RandomTermGenerationStopCriterion<GenConfig>,
) {
    let mut map = HashMap::new();
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Alt), 0.30_f32);
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Concat),
        0.30_f32,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Star),
        0.20_f32,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        0.10_f32,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'b')),
        0.10_f32,
    );
    let probas = TermGenerationSymbolsProbabilities::from_map(map).unwrap();
    let stop_crit = RandomTermGenerationStopCriterion::<GenConfig>::new(
        max_depth,
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
    );
    (probas, stop_crit)
}

// == benchmark =================================================================

fn bench_random_gen(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("random_gen");
    for max_depth in [3u32, 5, 8] {
        let (probas, stop_crit) = make_setup(max_depth);
        for count in [100usize, 1000] {
            group.bench_with_input(
                BenchmarkId::new(format!("depth{}", max_depth), count),
                &count,
                |b, &n| {
                    b.iter(|| {
                        let mut rng = StdRng::seed_from_u64(42);
                        (0..n)
                            .map(|_| {
                                generate_random_term(
                                    &probas,
                                    &stop_crit,
                                    black_box(&()),
                                    &mut rng,
                                    f,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_random_gen);
criterion_main!(benches);
