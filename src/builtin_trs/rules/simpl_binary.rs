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


use crate::core::terms::{position::PositionInLanguageTerm, term::{LanguageTerm, RewritableLanguageOperatorSymbol}};



pub trait GenericBinaryOperatorSimplifier<LOS : RewritableLanguageOperatorSymbol> {
    fn is_binary(&self, op : &LOS) -> bool;
    /**
     Try to simplify a term which root is a binary operator.
    This can be used to:
        - simplify neutral elements
        - or deal with idempotent operators
        - or try to perform concrete computations
    More complex cases can also be implemented via this trait.
    Examples :
    - in math formulae with uninterpreted variables :
        try_simplify_under_binary_operator( + , 0, x ) will return Some(x)
        try_simplify_under_binary_operator( + , x, 0 ) will return Some(x)
        because 0 is a neutral element for the addition
    - in boolean logic :
        try_simplify_under_binary_operator( OR , x, x ) will return Some(x)
        because OR is idempotent
    - in concrete math expressions :
        try_simplify_under_binary_operator( + , 3, 5 ) will return Some(8)
        because 3+5=8
     **/
    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &LOS,
        left : &LanguageTerm<LOS>,
        right : &LanguageTerm<LOS>,
    ) -> Option<LanguageTerm<LOS>>;
}


pub(crate) fn transformation_generic_simpl_under_binary_operator<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn GenericBinaryOperatorSimplifier<LOS>>,
    term : &LanguageTerm<LOS>,
    _context_term : &LanguageTerm<LOS>,
    _position_in_context_term : &PositionInLanguageTerm
) -> Option<LanguageTerm<LOS>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary operator
    let precondition = checker.is_binary(operator_at_root);
    // ***
    if precondition {
        let left_sub_term = term.sub_terms.first().unwrap();
        let right_sub_term = term.sub_terms.get(1).unwrap();
        checker.try_simplify_under_binary_operator(
            operator_at_root,
            left_sub_term,
            right_sub_term)
    } else {
        None
    }
}



