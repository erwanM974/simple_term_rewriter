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

use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol};

/// Bridge trait for converting a [`LanguageTerm`] back into a domain-specific term type.
///
/// Implement this on your own term representation alongside
/// [`FromDomainSpecificTermToRewritableTerm`](super::to_rewritable_term::FromDomainSpecificTermToRewritableTerm)
/// to obtain a round-trip between the two representations.
pub trait FromRewritableTermToDomainSpecificTerm<LOS: RewritableLanguageOperatorSymbol>:
    Sized + Clone
{
    /// Builds a domain-specific term from an operator symbol and its already-converted sub-terms.
    fn instantiate_term_under_operator(operator: &LOS, sub_terms: &mut Vec<Self>) -> Self;

    /// Converts a [`LanguageTerm`] back to this domain-specific term type.
    ///
    /// Recursively converts sub-terms, then calls
    /// [`instantiate_term_under_operator`](Self::instantiate_term_under_operator).
    fn from_rewritable_term(rewritable_term: &LanguageTerm<LOS>) -> Self {
        let mut sub_terms = vec![];
        for rewr_sub_term in &rewritable_term.sub_terms {
            sub_terms.push(Self::from_rewritable_term(rewr_sub_term));
        }
        Self::instantiate_term_under_operator(&rewritable_term.operator, &mut sub_terms)
    }
}
