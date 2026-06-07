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

use rand::rngs::StdRng;
use std::hash::Hash;

use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// Configuration bundle for random term generation.
///
/// Implement this trait to tie together the language operator symbol type,
/// an optional generation context, and a pattern type.
pub trait RandomTermGenerationConfig: Sized {
    /// The language operator symbol type for the language being generated.
    type LOS: RewritableLanguageOperatorSymbol;

    /// Arbitrary context data threaded through the generation process
    /// (e.g. a variable pool or a grammar).
    type CONTEXT;

    /// The pattern type used to generate structured sub-terms.
    type PATTERN: TermPatternForRandomGeneration<Self>;
}

/// A structured sub-term pattern that can generate a concrete [`LanguageTerm`] on demand.
///
/// Patterns allow the random generator to produce semantically interesting
/// sub-trees (e.g. well-typed expressions or atoms drawn from a fixed set)
/// rather than purely random operator applications.
pub trait TermPatternForRandomGeneration<CONF: RandomTermGenerationConfig>:
    Clone + PartialEq + Eq + Hash
{
    /// Generates a concrete term from this pattern using `rng` and `context`.
    fn generate_term_from_pattern(
        &self,
        rng: &mut StdRng,
        context: &CONF::CONTEXT,
        factory: &mut TermFactory<CONF::LOS>,
    ) -> LanguageTerm<CONF::LOS>;
}

/// A symbol the random generator may choose at each node: either a concrete
/// language operator or a structured pattern.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TermGenerationSymbol<LOS, PATTERN> {
    /// A concrete language operator; its arity determines how many sub-terms are generated.
    LanguageSymbol(LOS),
    /// A pattern that generates an entire sub-tree directly.
    Pattern(PATTERN),
}
