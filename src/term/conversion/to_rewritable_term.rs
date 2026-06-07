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

use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

/// Bridge trait for converting a domain-specific term type into a [`LanguageTerm`].
///
/// Implement this on your own term representation so that the rest of the crate
/// can work with it via `to_rewritable_term`.
pub trait FromDomainSpecificTermToRewritableTerm<LOS: RewritableLanguageOperatorSymbol>:
    Sized + Clone
{
    /// Returns the operator symbol at the root of this domain-specific term.
    fn get_operator_at_root(&self) -> LOS;

    /// Returns the direct sub-terms of this domain-specific term.
    fn get_subterms(&self) -> Vec<&Self>;

    /// Converts this domain-specific term to a [`LanguageTerm`].
    ///
    /// Recursively converts sub-terms via [`get_subterms`](Self::get_subterms)
    /// and [`get_operator_at_root`](Self::get_operator_at_root).
    fn to_rewritable_term(&self, factory: &mut TermFactory<LOS>) -> LanguageTerm<LOS> {
        let mut sub_terms = vec![];
        for domain_specific_sub_term in self.get_subterms() {
            sub_terms.push(domain_specific_sub_term.to_rewritable_term(factory));
        }
        LanguageTermNode::build(self.get_operator_at_root(), sub_terms, factory)
    }
}
