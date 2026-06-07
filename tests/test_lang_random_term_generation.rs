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

//! Tests for `generate_random_term`, `TermGenerationSymbolsProbabilities`, and
//! related types from `simple_term_rewriter::random_term_generation`.

mod common;

use std::collections::HashMap;

use hashconsing::HConsign;
use rand::{rngs::StdRng, SeedableRng};

use simple_term_rewriter::random_term_generation::{
    gen::{generate_random_term, RandomTermGenerationStopCriterion},
    probas::{InteractionSymbolsProbabilitiesError, TermGenerationSymbolsProbabilities},
    types::{RandomTermGenerationConfig, TermGenerationSymbol, TermPatternForRandomGeneration},
};
use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

use common::regex::constructors::epsilon;
use common::regex::generation::{generate_regex_terms, RegexGenerationConfig};
use common::regex::lang::RegexOp;

// == determinism ===============================================================
//
// `generate_random_term` is deterministic for a fixed (probas, rng) pair.
// We test this by building the probas object once and running two identically-
// seeded RNGs against it.  Creating the probas inside `generate_regex_terms`
// twice is not sufficient because HashMap iteration order is non-deterministic
// across distinct HashMap instances, so the symbol→bucket mapping can differ.

fn simple_probas_and_stop() -> (
    TermGenerationSymbolsProbabilities<RegexGenerationConfig>,
    RandomTermGenerationStopCriterion<RegexGenerationConfig>,
) {
    let mut map = HashMap::new();
    map.insert(TermGenerationSymbol::LanguageSymbol(RegexOp::Alt), 0.40_f32);
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Star),
        0.30_f32,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        0.30_f32,
    );
    let probas = TermGenerationSymbolsProbabilities::from_map(map).unwrap();
    let stop_crit = RandomTermGenerationStopCriterion::<RegexGenerationConfig>::new(
        3,
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
    );
    (probas, stop_crit)
}

#[test]
fn same_seed_produces_same_terms() {
    let (probas, stop_crit) = simple_probas_and_stop();
    let mut rng1 = StdRng::seed_from_u64(42);
    let mut rng2 = StdRng::seed_from_u64(42);
    let mut f1: TermFactory<RegexOp> = HConsign::empty();
    let mut f2: TermFactory<RegexOp> = HConsign::empty();
    let a: Vec<_> = (0..20)
        .map(|_| generate_random_term(&probas, &stop_crit, &(), &mut rng1, &mut f1))
        .collect();
    let b: Vec<_> = (0..20)
        .map(|_| generate_random_term(&probas, &stop_crit, &(), &mut rng2, &mut f2))
        .collect();
    // Compare by debug string since they come from different factories
    let a_str: Vec<_> = a.iter().map(|t| format!("{t:?}")).collect();
    let b_str: Vec<_> = b.iter().map(|t| format!("{t:?}")).collect();
    assert_eq!(a_str, b_str);
}

#[test]
fn different_seeds_produce_different_terms() {
    let (probas, stop_crit) = simple_probas_and_stop();
    let mut rng1 = StdRng::seed_from_u64(1);
    let mut rng2 = StdRng::seed_from_u64(2);
    let mut f1: TermFactory<RegexOp> = HConsign::empty();
    let mut f2: TermFactory<RegexOp> = HConsign::empty();
    let a: Vec<_> = (0..20)
        .map(|_| generate_random_term(&probas, &stop_crit, &(), &mut rng1, &mut f1))
        .collect();
    let b: Vec<_> = (0..20)
        .map(|_| generate_random_term(&probas, &stop_crit, &(), &mut rng2, &mut f2))
        .collect();
    let a_str: Vec<_> = a.iter().map(|t| format!("{t:?}")).collect();
    let b_str: Vec<_> = b.iter().map(|t| format!("{t:?}")).collect();
    assert_ne!(a_str, b_str);
}

// == depth bound ===============================================================

fn tree_depth(term: &LanguageTerm<RegexOp>) -> usize {
    if term.sub_terms.is_empty() {
        1
    } else {
        1 + term.sub_terms.iter().map(tree_depth).max().unwrap()
    }
}

#[test]
fn depth_bound_is_respected() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // generate_regex_terms uses max_depth = 5; deepest possible tree is depth 6.
    let terms = generate_regex_terms(200, 7, &mut f);
    for t in &terms {
        assert!(
            tree_depth(t) <= 6,
            "term depth {} exceeds bound: {:?}",
            tree_depth(t),
            t
        );
    }
}

// == operator coverage =========================================================

fn collect_operators(term: &LanguageTerm<RegexOp>, out: &mut Vec<RegexOp>) {
    out.push(term.operator.clone());
    for sub in &term.sub_terms {
        collect_operators(sub, out);
    }
}

#[test]
fn all_operators_are_reachable() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let terms = generate_regex_terms(500, 42, &mut f);
    let mut ops: Vec<RegexOp> = Vec::new();
    for t in &terms {
        collect_operators(t, &mut ops);
    }
    assert!(ops.contains(&RegexOp::Empty),   "Empty not seen");
    assert!(ops.contains(&RegexOp::Epsilon), "Epsilon not seen");
    assert!(ops.iter().any(|op| matches!(op, RegexOp::Atom(_))), "Atom not seen");
    assert!(ops.contains(&RegexOp::Alt),     "Alt not seen");
    assert!(ops.contains(&RegexOp::Concat),  "Concat not seen");
    assert!(ops.contains(&RegexOp::Star),    "Star not seen");
}

// == Pattern arm: stop criterion ===============================================
//
// When the stop criterion holds (depth >= max_depth) and the stop symbol is a
// Pattern, generate_term_from_pattern must be called.

#[derive(Clone, PartialEq, Eq, Hash)]
struct ConstantEpsilonPattern;

impl TermPatternForRandomGeneration<PatternConfig> for ConstantEpsilonPattern {
    fn generate_term_from_pattern(
        &self,
        _rng: &mut StdRng,
        _ctx: &(),
        factory: &mut TermFactory<RegexOp>,
    ) -> LanguageTerm<RegexOp> {
        epsilon(factory)
    }
}

struct PatternConfig;

impl RandomTermGenerationConfig for PatternConfig {
    type LOS = RegexOp;
    type CONTEXT = ();
    type PATTERN = ConstantEpsilonPattern;
}

#[test]
fn stop_criterion_pattern_arm_is_exercised() {
    // max_depth = 0 means the stop criterion fires immediately on every call.
    let mut map = HashMap::new();
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        1.0_f32,
    );
    let probas = TermGenerationSymbolsProbabilities::<PatternConfig>::from_map(map).unwrap();
    let stop_crit = RandomTermGenerationStopCriterion::<PatternConfig>::new(
        0,
        TermGenerationSymbol::Pattern(ConstantEpsilonPattern),
    );
    let mut rng = StdRng::seed_from_u64(0);
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = generate_random_term(&probas, &stop_crit, &(), &mut rng, &mut f);
    assert_eq!(term, epsilon(&mut f));
}

// == Pattern arm: random symbol selection =====================================
//
// When the selected random symbol is a Pattern (not a LanguageSymbol),
// generate_term_from_pattern must be called.

#[test]
fn random_symbol_pattern_arm_is_exercised() {
    // Pattern at 100 % probability; max_depth is large so we never hit the stop.
    let mut map = HashMap::new();
    map.insert(
        TermGenerationSymbol::Pattern(ConstantEpsilonPattern),
        1.0_f32,
    );
    let probas = TermGenerationSymbolsProbabilities::<PatternConfig>::from_map(map).unwrap();
    let stop_crit = RandomTermGenerationStopCriterion::<PatternConfig>::new(
        10,
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
    );
    let mut rng = StdRng::seed_from_u64(0);
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let term = generate_random_term(&probas, &stop_crit, &(), &mut rng, &mut f);
    assert_eq!(term, epsilon(&mut f));
}

// == from_map validation errors ================================================

#[test]
fn from_map_rejects_probability_above_one() {
    let mut map = HashMap::new();
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        1.5_f32,
    );
    assert!(matches!(
        TermGenerationSymbolsProbabilities::<RegexGenerationConfig>::from_map(map),
        Err(InteractionSymbolsProbabilitiesError::SymbolProbabilityMustBeBetweenOAnd1)
    ));
}

#[test]
fn from_map_rejects_negative_probability() {
    let mut map = HashMap::new();
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        -0.1_f32,
    );
    assert!(matches!(
        TermGenerationSymbolsProbabilities::<RegexGenerationConfig>::from_map(map),
        Err(InteractionSymbolsProbabilitiesError::SymbolProbabilityMustBeBetweenOAnd1)
    ));
}

#[test]
fn from_map_rejects_sum_not_equal_to_one() {
    let mut map = HashMap::new();
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'a')),
        0.4_f32,
    );
    map.insert(
        TermGenerationSymbol::LanguageSymbol(RegexOp::Atom(b'b')),
        0.4_f32,
    );
    // sum = 0.8, not 1.0
    assert!(matches!(
        TermGenerationSymbolsProbabilities::<RegexGenerationConfig>::from_map(map),
        Err(InteractionSymbolsProbabilitiesError::SumOfProbabilitiesMustBe1)
    ));
}
