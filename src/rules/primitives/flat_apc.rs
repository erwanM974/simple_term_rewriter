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

use crate::position::PositionInLanguageTerm;
use crate::rule::RewriteRule;
use crate::rules::util::assoc::{
    fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively,
};
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// A transformation strategy that works by flattening an associative operator,
/// transforming the resulting sequence, and reconstructing.
pub trait ModuloAssociativeGenericFlattenedChecker<LOS: RewritableLanguageOperatorSymbol> {
    /// Returns `true` if `op` is an associative binary operator whose argument
    /// chain should be flattened and passed to `transform_flattened_sub_terms`.
    fn is_an_associative_binary_operator_we_may_consider(&self, op: &LOS) -> bool;

    /// Transforms the flattened sub-term sequence of an associative node.
    ///
    /// Returns `Some(new_children)` to replace the sequence, `None` to leave
    /// the term unchanged.  Use `factory` to build any new terms that are
    /// needed in the result.
    fn transform_flattened_sub_terms(
        &self,
        considered_ac_op: &LOS,
        flattened_subterms: Vec<&LanguageTerm<LOS>>,
        factory: &mut TermFactory<LOS>,
    ) -> Option<Vec<LanguageTerm<LOS>>>;
}

fn transformation_modulo_associative_generic_flattened_transfo<
    LOS: RewritableLanguageOperatorSymbol,
>(
    checker: &dyn ModuloAssociativeGenericFlattenedChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    if checker.is_an_associative_binary_operator_we_may_consider(&term.operator) {
        let op = &term.operator;
        let flat = get_associative_sub_terms_recursively(term, op);
        if let Some(mut transformed) = checker.transform_flattened_sub_terms(op, flat, factory) {
            return fold_associative_sub_terms_recursively(op, &mut transformed, &None, factory);
        }
    }
    None
}

/// Rewrite rule that flattens an AC term, applies a user-defined transformation
/// to the flattened sequence, then reconstructs.
pub struct FlattenedACTransfoRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn ModuloAssociativeGenericFlattenedChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FlattenedACTransfoRule<LOS> {
    /// Creates the rule with the given description and transformation strategy.
    pub fn new(
        desc: impl Into<String>,
        checker: impl ModuloAssociativeGenericFlattenedChecker<LOS> + 'static,
    ) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FlattenedACTransfoRule<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        _ctx: &LanguageTerm<LOS>,
        _pos: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        transformation_modulo_associative_generic_flattened_transfo(self.checker.as_ref(), term, factory)
    }
}
