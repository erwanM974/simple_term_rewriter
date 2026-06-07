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
use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

use super::distributivity_checker::DistributivityChecker;

/// `op1(x, op2(y, z)) → op2(op1(x, y), op1(x, z))`
fn transformation_defactorize_left_distributive<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op1 = &term.operator;
    if !checker.is_binary(op1) {
        return None;
    }
    let x = &term.sub_terms[0];
    let rhs = &term.sub_terms[1];
    if !checker.is_binary(&rhs.operator) {
        return None;
    }
    let op2 = &rhs.operator;
    if !checker.is_left_distributive_over(op1, op2) {
        return None;
    }
    let y = &rhs.sub_terms[0];
    let z = &rhs.sub_terms[1];
    let new_l = LanguageTermNode::build(op1.clone(), vec![x.clone(), y.clone()], factory);
    let new_r = LanguageTermNode::build(op1.clone(), vec![x.clone(), z.clone()], factory);
    Some(LanguageTermNode::build(
        op2.clone(),
        vec![new_l, new_r],
        factory,
    ))
}

/// `op1(op2(y, z), x) → op2(op1(y, x), op1(z, x))`
fn transformation_defactorize_right_distributive<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op1 = &term.operator;
    if !checker.is_binary(op1) {
        return None;
    }
    let lhs = &term.sub_terms[0];
    let x = &term.sub_terms[1];
    if !checker.is_binary(&lhs.operator) {
        return None;
    }
    let op2 = &lhs.operator;
    if !checker.is_right_distributive_over(op1, op2) {
        return None;
    }
    let y = &lhs.sub_terms[0];
    let z = &lhs.sub_terms[1];
    let new_l = LanguageTermNode::build(op1.clone(), vec![y.clone(), x.clone()], factory);
    let new_r = LanguageTermNode::build(op1.clone(), vec![z.clone(), x.clone()], factory);
    Some(LanguageTermNode::build(
        op2.clone(),
        vec![new_l, new_r],
        factory,
    ))
}

/// Left-distributive defactorization: `op1(x, op2(y, z)) → op2(op1(x, y), op1(x, z))`.
pub struct DefactorizeLeftRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> DefactorizeLeftRule<LOS> {
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

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for DefactorizeLeftRule<LOS> {
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
        transformation_defactorize_left_distributive(self.checker.as_ref(), term, factory)
    }
}

/// Right-distributive defactorization: `op1(op2(y, z), x) → op2(op1(y, x), op1(z, x))`.
pub struct DefactorizeRightRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> DefactorizeRightRule<LOS> {
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

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for DefactorizeRightRule<LOS> {
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
        transformation_defactorize_right_distributive(self.checker.as_ref(), term, factory)
    }
}
