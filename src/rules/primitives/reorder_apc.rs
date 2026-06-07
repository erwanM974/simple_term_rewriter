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
use crate::rules::util::lpo::is_greater_as_per_lexicographic_path_ordering;
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// Everything needed to canonically reorder an associative, partially commutative
/// operator by sorting its flattened argument chain.
pub trait ModuloAssociativePartialReorderer<LOS: RewritableLanguageOperatorSymbol> {
    fn is_an_associative_partially_commutative_binary_operator_we_may_consider(
        &self,
        op: &LOS,
    ) -> bool;

    fn may_commute_under(
        &self,
        parent_op: &LOS,
        left_sub_term: &LanguageTerm<LOS>,
        right_sub_term: &LanguageTerm<LOS>,
    ) -> bool;

    fn compare_operators(&self, op1: &LOS, op2: &LOS) -> std::cmp::Ordering;
}

fn ad_hoc_partially_commutative_recursive_reorderer<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn ModuloAssociativePartialReorderer<LOS>,
    considered_ac_operator: &LOS,
    flattened_sub_terms: Vec<LanguageTerm<LOS>>,
    has_changed: &mut bool,
) -> Vec<LanguageTerm<LOS>> {
    if flattened_sub_terms.len() <= 1 {
        return flattened_sub_terms;
    }

    let mut items = flattened_sub_terms;
    let head = items.remove(0);
    let mut sorted_tail = ad_hoc_partially_commutative_recursive_reorderer(
        checker,
        considered_ac_operator,
        items,
        has_changed,
    );
    let head_of_tail = sorted_tail.remove(0);
    let mut remainder = sorted_tail;

    if checker.may_commute_under(considered_ac_operator, &head, &head_of_tail)
        && is_greater_as_per_lexicographic_path_ordering(&head, &head_of_tail, &|x, y| {
            checker.compare_operators(x, y)
        })
    {
        *has_changed = true;
        remainder.insert(0, head);
        let mut remainder = ad_hoc_partially_commutative_recursive_reorderer(
            checker,
            considered_ac_operator,
            remainder,
            has_changed,
        );
        remainder.insert(0, head_of_tail);
        remainder
    } else {
        remainder.insert(0, head_of_tail);
        remainder.insert(0, head);
        remainder
    }
}

fn transformation_modulo_assoc_partial_reordering<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn ModuloAssociativePartialReorderer<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    if !checker
        .is_an_associative_partially_commutative_binary_operator_we_may_consider(&term.operator)
    {
        return None;
    }
    let op = &term.operator;
    let flat: Vec<LanguageTerm<LOS>> = get_associative_sub_terms_recursively(term, op)
        .iter()
        .copied()
        .cloned()
        .collect();

    let mut has_changed = false;
    let mut sorted =
        ad_hoc_partially_commutative_recursive_reorderer(checker, op, flat, &mut has_changed);

    if has_changed {
        fold_associative_sub_terms_recursively(op, &mut sorted, &None, factory)
    } else {
        None
    }
}

/// Rewrite rule that canonically reorders operands of a partially commutative,
/// associative operator.
pub struct PartialACReorderRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn ModuloAssociativePartialReorderer<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> PartialACReorderRule<LOS> {
    pub fn new(
        desc: impl Into<String>,
        checker: impl ModuloAssociativePartialReorderer<LOS> + 'static,
    ) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for PartialACReorderRule<LOS> {
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
        transformation_modulo_assoc_partial_reordering(self.checker.as_ref(), term, factory)
    }
}
