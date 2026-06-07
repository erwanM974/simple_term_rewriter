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

use std::collections::{HashMap, HashSet};

use crate::position::PositionInLanguageTerm;
use crate::rule::RewriteRule;
use crate::rules::util::assoc::{
    fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively,
};
use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

use super::distributivity_checker::DistributivityChecker;

#[allow(clippy::type_complexity)]
fn transformation_factorize_left_distributive_modulo_ac<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    ctx: &LanguageTerm<LOS>,
    pos: &PositionInLanguageTerm,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op2 = &term.operator;
    if !checker.is_binary(op2) || !checker.is_commutative(op2) {
        return None;
    }

    // If the parent is the same commutative op, skip — let the parent handle it.
    if let Some(parent_pos) = pos.get_parent_position() {
        if let Some(parent) = ctx.get_sub_term_at_position(&parent_pos) {
            if &parent.operator == op2 {
                return None;
            }
        }
    }

    let sub_terms = if checker.is_associative(op2) {
        get_associative_sub_terms_recursively(term, op2)
    } else {
        term.sub_terms.iter().collect()
    };

    let head_operators: HashSet<LOS> = sub_terms
        .iter()
        .filter(|x| {
            checker.is_binary(&x.operator) && checker.is_left_distributive_over(&x.operator, op2)
        })
        .map(|x| x.operator.clone())
        .collect();

    let mut new_factorized: Vec<LanguageTerm<LOS>> = vec![];
    let mut factorized_indices = HashSet::new();

    for head_op in head_operators {
        let mut found: HashMap<&LanguageTerm<LOS>, Vec<(usize, Vec<LanguageTerm<LOS>>)>> = HashMap::new();
        for (idx, sub) in sub_terms.iter().enumerate() {
            if sub.operator == head_op {
                let sub_sub = if checker.is_associative(&head_op) {
                    get_associative_sub_terms_recursively(sub, &head_op)
                } else {
                    sub.sub_terms.iter().collect()
                };
                let rest: Vec<LanguageTerm<LOS>> = sub_sub[1..].iter().copied().cloned().collect();
                let key = sub_sub[0];
                found.entry(key).or_default().push((idx, rest));
            } else {
                found.entry(*sub).or_default().push((idx, vec![]));
            }
        }
        for (lhs, rhss) in found {
            if rhss.len() > 1 {
                let mut rhs_terms: Vec<LanguageTerm<LOS>> = vec![];
                for (idx, mut rhs) in rhss {
                    factorized_indices.insert(idx);
                    rhs_terms.push(
                        fold_associative_sub_terms_recursively(
                            &head_op,
                            &mut rhs,
                            &Some(checker.get_empty_operation_symbol()),
                            factory,
                        )
                        .unwrap(),
                    );
                }
                let rhs_op2 = fold_associative_sub_terms_recursively(
                    op2,
                    &mut rhs_terms,
                    &Some(checker.get_empty_operation_symbol()),
                    factory,
                )
                .unwrap();
                let factorized =
                    LanguageTermNode::build(head_op.clone(), vec![lhs.clone(), rhs_op2], factory);
                new_factorized.push(factorized);
            }
        }
    }

    if new_factorized.is_empty() {
        return None;
    }

    let mut new_subs: Vec<LanguageTerm<LOS>> = vec![];
    for (idx, sub) in sub_terms.iter().enumerate() {
        if !factorized_indices.contains(&idx) {
            new_subs.push((*sub).clone());
        }
    }
    new_subs.extend(new_factorized);
    Some(
        fold_associative_sub_terms_recursively(
            op2,
            &mut new_subs,
            &Some(checker.get_empty_operation_symbol()),
            factory,
        )
        .unwrap(),
    )
}

#[allow(clippy::type_complexity)]
fn transformation_factorize_right_distributive_modulo_ac<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    ctx: &LanguageTerm<LOS>,
    pos: &PositionInLanguageTerm,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op2 = &term.operator;
    if !checker.is_binary(op2) || !checker.is_commutative(op2) {
        return None;
    }

    if let Some(parent_pos) = pos.get_parent_position() {
        if let Some(parent) = ctx.get_sub_term_at_position(&parent_pos) {
            if &parent.operator == op2 {
                return None;
            }
        }
    }

    let sub_terms = if checker.is_associative(op2) {
        get_associative_sub_terms_recursively(term, op2)
    } else {
        term.sub_terms.iter().collect()
    };

    let head_operators: HashSet<LOS> = sub_terms
        .iter()
        .filter(|x| {
            checker.is_binary(&x.operator) && checker.is_right_distributive_over(&x.operator, op2)
        })
        .map(|x| x.operator.clone())
        .collect();

    let mut new_factorized: Vec<LanguageTerm<LOS>> = vec![];
    let mut factorized_indices = HashSet::new();

    for head_op in head_operators {
        let mut found: HashMap<&LanguageTerm<LOS>, Vec<(usize, Vec<LanguageTerm<LOS>>)>> = HashMap::new();
        for (idx, sub) in sub_terms.iter().enumerate() {
            if sub.operator == head_op {
                let sub_sub = if checker.is_associative(&head_op) {
                    get_associative_sub_terms_recursively(sub, &head_op)
                } else {
                    sub.sub_terms.iter().collect()
                };
                let last_idx = sub_sub.len() - 1;
                let rest: Vec<LanguageTerm<LOS>> =
                    sub_sub[..last_idx].iter().copied().cloned().collect();
                let key = sub_sub[last_idx];
                found.entry(key).or_default().push((idx, rest));
            } else {
                found.entry(*sub).or_default().push((idx, vec![]));
            }
        }
        for (rhs, lhss) in found {
            if lhss.len() > 1 {
                let mut lhs_terms: Vec<LanguageTerm<LOS>> = vec![];
                for (idx, mut lhs) in lhss {
                    factorized_indices.insert(idx);
                    lhs_terms.push(
                        fold_associative_sub_terms_recursively(
                            &head_op,
                            &mut lhs,
                            &Some(checker.get_empty_operation_symbol()),
                            factory,
                        )
                        .unwrap(),
                    );
                }
                let lhs_op2 = fold_associative_sub_terms_recursively(
                    op2,
                    &mut lhs_terms,
                    &Some(checker.get_empty_operation_symbol()),
                    factory,
                )
                .unwrap();
                let factorized =
                    LanguageTermNode::build(head_op.clone(), vec![lhs_op2, rhs.clone()], factory);
                new_factorized.push(factorized);
            }
        }
    }

    if new_factorized.is_empty() {
        return None;
    }

    let mut new_subs: Vec<LanguageTerm<LOS>> = vec![];
    for (idx, sub) in sub_terms.iter().enumerate() {
        if !factorized_indices.contains(&idx) {
            new_subs.push((*sub).clone());
        }
    }
    new_subs.extend(new_factorized);
    Some(
        fold_associative_sub_terms_recursively(
            op2,
            &mut new_subs,
            &Some(checker.get_empty_operation_symbol()),
            factory,
        )
        .unwrap(),
    )
}

/// Rewrite rule for left-distributive factorization modulo AC.
pub struct FactorizeLeftModACRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FactorizeLeftModACRule<LOS> {
    pub fn new(
        desc: impl Into<String>,
        checker: impl DistributivityChecker<LOS> + 'static,
    ) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FactorizeLeftModACRule<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        ctx: &LanguageTerm<LOS>,
        pos: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        transformation_factorize_left_distributive_modulo_ac(self.checker.as_ref(), term, ctx, pos, factory)
    }
}

/// Rewrite rule for right-distributive factorization modulo AC.
pub struct FactorizeRightModACRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FactorizeRightModACRule<LOS> {
    pub fn new(
        desc: impl Into<String>,
        checker: impl DistributivityChecker<LOS> + 'static,
    ) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FactorizeRightModACRule<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        ctx: &LanguageTerm<LOS>,
        pos: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        transformation_factorize_right_distributive_modulo_ac(
            self.checker.as_ref(),
            term,
            ctx,
            pos,
            factory,
        )
    }
}
