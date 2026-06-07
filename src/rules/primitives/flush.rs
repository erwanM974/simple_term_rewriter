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

/// A checker that determines whether an operator is binary-associative.
pub trait AssociativityChecker<LOS: RewritableLanguageOperatorSymbol> {
    /// Returns `true` if `op` is a binary associative operator.
    fn is_binary_associative(&self, op: &LOS) -> bool;
}

/// `op(op(x, y), z) → op(x, op(y, z))`
fn transformation_flush_to_the_right<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn AssociativityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op = &term.operator;
    if !checker.is_binary_associative(op) {
        return None;
    }
    let t1 = term.sub_terms.first().unwrap();
    if &t1.operator != op {
        return None;
    }
    let t2 = &term.sub_terms[1];
    let t11 = &t1.sub_terms[0];
    let t12 = &t1.sub_terms[1];
    let inner = LanguageTermNode::build(op.clone(), vec![t12.clone(), t2.clone()], factory);
    Some(LanguageTermNode::build(
        op.clone(),
        vec![t11.clone(), inner],
        factory,
    ))
}

/// `op(x, op(y, z)) → op(op(x, y), z)`
fn transformation_flush_to_the_left<LOS: RewritableLanguageOperatorSymbol>(
    checker: &dyn AssociativityChecker<LOS>,
    term: &LanguageTerm<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let op = &term.operator;
    if !checker.is_binary_associative(op) {
        return None;
    }
    let t2 = &term.sub_terms[1];
    if &t2.operator != op {
        return None;
    }
    let t1 = &term.sub_terms[0];
    let t21 = &t2.sub_terms[0];
    let t22 = &t2.sub_terms[1];
    let inner = LanguageTermNode::build(op.clone(), vec![t1.clone(), t21.clone()], factory);
    Some(LanguageTermNode::build(
        op.clone(),
        vec![inner, t22.clone()],
        factory,
    ))
}

/// Rewrite rule that right-flushes an associative binary operator:
/// `op(op(x, y), z) → op(x, op(y, z))`.
pub struct FlushRightRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn AssociativityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FlushRightRule<LOS> {
    /// Creates the rule with the given description and associativity checker.
    pub fn new(desc: impl Into<String>, checker: impl AssociativityChecker<LOS> + 'static) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FlushRightRule<LOS> {
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
        transformation_flush_to_the_right(self.checker.as_ref(), term, factory)
    }
}

/// Rewrite rule that left-flushes an associative binary operator:
/// `op(x, op(y, z)) → op(op(x, y), z)`.
pub struct FlushLeftRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn AssociativityChecker<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> FlushLeftRule<LOS> {
    /// Creates the rule with the given description and associativity checker.
    pub fn new(desc: impl Into<String>, checker: impl AssociativityChecker<LOS> + 'static) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for FlushLeftRule<LOS> {
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
        transformation_flush_to_the_left(self.checker.as_ref(), term, factory)
    }
}
