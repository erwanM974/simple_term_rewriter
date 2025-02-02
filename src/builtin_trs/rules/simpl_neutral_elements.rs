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



use crate::builtin_trs::interface::BuiltinTermRewritingInterface;
use crate::core::term::LanguageTerm;


/**
If c is a constant (operator with arity 0) and it is a neutral element for a binary operator op
then this transformation does :
 - either : op(x,c) -> x
 - or     : op(c,x) -> x
 **/
pub fn transformation_simpl_neutral_under_binary_operator<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary operator, not necessarily associative
    // we suppose however, that it is a "loi de composition interne unifère" so that
    // the neutral element is well defined
    let precondition = STRI::op_arity(operator_at_root) == 2;
    // ***
    if precondition {
        let left_sub_term = term.sub_terms.first().unwrap();
        let right_sub_term = term.sub_terms.get(1).unwrap();
        if STRI::is_neutral_element_or_fixpoint_for(left_sub_term, operator_at_root) {
            // if the left sub-term is a neutral element for the parent operator
            // then we rewrite the whole term keeping only the right sub-term
            return Some(right_sub_term.clone());
        }
        if STRI::is_neutral_element_or_fixpoint_for(right_sub_term, operator_at_root) {
            // if the right sub-term is a neutral element for the parent operator
            // then we rewrite the whole term keeping only the left sub-term
            return Some(left_sub_term.clone());
        }
    }
    None
}




/**
If c is a constant (operator with arity 0) and it is a fixpoint for a unary operator op
then this transformation does :
- op(c) -> c
 **/
pub fn transformation_simpl_fixpoint_under_unary_operator<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a unary operator
    let precondition = STRI::op_arity(operator_at_root) == 1;
    // ***
    if precondition {
        let sub_term = term.sub_terms.first().unwrap();
        if STRI::is_neutral_element_or_fixpoint_for(sub_term, operator_at_root) {
            // if the sub-term is a fixpoint for the parent operator
            // then we rewrite the whole term keeping only the sub-term
            return Some(sub_term.clone());
        }
    }
    None
}