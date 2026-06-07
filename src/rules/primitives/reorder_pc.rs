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
use crate::rules::util::lpo::is_greater_as_per_lexicographic_path_ordering;
use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

/// Everything needed to canonically order the two operands of a commutative
/// binary operator.
pub trait CommutativeCheckerAndOrderer<LOS: RewritableLanguageOperatorSymbol> {
    fn is_a_binary_commutative_operator(&self, op: &LOS) -> bool;

    fn may_commute_under(
        &self,
        parent_op: &LOS,
        left_sub_term: &LanguageTerm<LOS>,
        right_sub_term: &LanguageTerm<LOS>,
    ) -> bool;

    fn compare_operators(&self, op1: &LOS, op2: &LOS) -> std::cmp::Ordering;
}

/// Swaps the two operands of a commutative binary operator when they are
/// out of LPO order: `op(y, x) → op(x, y)` when `x <_lpo y`.
pub struct CommuteReorderRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    checker: Box<dyn CommutativeCheckerAndOrderer<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> CommuteReorderRule<LOS> {
    pub fn new(
        desc: impl Into<String>,
        checker: impl CommutativeCheckerAndOrderer<LOS> + 'static,
    ) -> Self {
        Self {
            desc: desc.into(),
            checker: Box::new(checker),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for CommuteReorderRule<LOS> {
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
        let op = &term.operator;
        if !self.checker.is_a_binary_commutative_operator(op) {
            return None;
        }
        let left = &term.sub_terms[0];
        let right = &term.sub_terms[1];
        if self.checker.may_commute_under(op, left, right)
            && is_greater_as_per_lexicographic_path_ordering::<LOS>(left, right, &|x, y| {
                self.checker.compare_operators(x, y)
            })
        {
            Some(LanguageTermNode::build(
                op.clone(),
                vec![right.clone(), left.clone()],
                factory,
            ))
        } else {
            None
        }
    }
}
