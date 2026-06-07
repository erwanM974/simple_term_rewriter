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

/// `op2(op1(x, y), op1(x, z)) → op1(x, op2(y, z))`
fn transformation_factorize_left_distributive<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op2 = &term.operator;
    if !checker.is_binary(op2) {
        return None;
    }
    let l = &term.sub_terms[0];
    let r = &term.sub_terms[1];
    if l.operator != r.operator || !checker.is_binary(&l.operator) {
        return None;
    }
    let op1 = &l.operator;
    if !checker.is_left_distributive_over(op1, op2) {
        return None;
    }
    let a = &l.sub_terms[0];
    let y = &l.sub_terms[1];
    let b = &r.sub_terms[0];
    let z = &r.sub_terms[1];
    if a != b {
        return None;
    }
    let new_right = LanguageTermNode::build(op2.clone(), vec![y.clone(), z.clone()], factory);
    Some(LanguageTermNode::build(
        op1.clone(),
        vec![a.clone(), new_right],
        factory,
    ))
}

/// `op2(op1(y, x), op1(z, x)) → op1(op2(y, z), x)`
fn transformation_factorize_right_distributive<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn DistributivityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op2 = &term.operator;
    if !checker.is_binary(op2) {
        return None;
    }
    let l = &term.sub_terms[0];
    let r = &term.sub_terms[1];
    if l.operator != r.operator || !checker.is_binary(&l.operator) {
        return None;
    }
    let op1 = &l.operator;
    if !checker.is_right_distributive_over(op1, op2) {
        return None;
    }
    let y = &l.sub_terms[0];
    let a = &l.sub_terms[1];
    let z = &r.sub_terms[0];
    let b = &r.sub_terms[1];
    if a != b {
        return None;
    }
    let new_left = LanguageTermNode::build(op2.clone(), vec![y.clone(), z.clone()], factory);
    Some(LanguageTermNode::build(
        op1.clone(),
        vec![new_left, a.clone()],
        factory,
    ))
}

/// Left-distributive factorization (syntactic, no AC):
/// `op2(op1(x, y), op1(x, z)) → op1(x, op2(y, z))`.
pub struct FactorizeLeftRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FactorizeLeftRule<LOS> {
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

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FactorizeLeftRule<LOS> {
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
        transformation_factorize_left_distributive(self.checker.as_ref(), term, factory)
    }
}

/// Right-distributive factorization (syntactic, no AC):
/// `op2(op1(y, x), op1(z, x)) → op1(op2(y, z), x)`.
pub struct FactorizeRightRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn DistributivityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FactorizeRightRule<LOS> {
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

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FactorizeRightRule<LOS> {
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
        transformation_factorize_right_distributive(self.checker.as_ref(), term, factory)
    }
}
