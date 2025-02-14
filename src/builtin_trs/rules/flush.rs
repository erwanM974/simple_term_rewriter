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

use std::hash::Hash;

use crate::core::term::LanguageTerm;



/**
 Something that can check an operator is associative.
 **/
pub trait AssociativityChecker<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    fn is_binary_associative(&self, op : &LanguageOperatorSymbol) -> bool;
}


/**
    If op is a binary associative operator
    then for any x,y,z, we have op(x,op(y,z)) equivalent to op(op(x,y),z
    This transformation performs:
    op(op(x,y),z) -> op(x,op(y,z))
    i.e., it flushes the content to the right
 **/
pub(crate) fn transformation_flush_to_the_right<
    LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash
>(
    checker : &Box<dyn AssociativityChecker<LanguageOperatorSymbol>>,
    term : &LanguageTerm<LanguageOperatorSymbol>
) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary associative operator
    let precondition = checker.is_binary_associative(operator_at_root);
    if precondition {
        // given OP the operator
        // we have a term of the form t=OP(t1,t2)
        let t1 = term.sub_terms.first().unwrap();
        // if the left sub-term has at its root the same operator,
        if &t1.operator == operator_at_root {
            // we have a term of the form t=OP(OP(t11,t12),t2)
            // we then return OP(t11,OP(t12,t2))
            let t2 = term.sub_terms.get(1).unwrap();
            let t11 = t1.sub_terms.first().unwrap();
            let t12 = t1.sub_terms.get(1).unwrap();
            let new_right_sub_term = LanguageTerm::new(
                operator_at_root.clone(),
                vec![
                    t12.clone(),
                    t2.clone()
                ]
            );
            let new_term = LanguageTerm::new(
                operator_at_root.clone(),
                vec![
                    t11.clone(),
                    new_right_sub_term
                ]
            );
            Some(new_term)
        } else {
            None
        }
    } else {
        None
    }
}





/**
If op is a binary associative operator
then for any x,y,z, we have op(x,op(y,z)) equivalent to op(op(x,y),z)
This transformation performs:
op(x,op(y,z)) -> op(op(x,y),z)
i.e., it flushes the content to the left
 **/
pub(crate) fn transformation_flush_to_the_left<
    LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash
>(
    checker : &Box<dyn AssociativityChecker<LanguageOperatorSymbol>>,
    term : &LanguageTerm<LanguageOperatorSymbol>
) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary associative operator
    let precondition = checker.is_binary_associative(operator_at_root);
    if precondition {
        // given OP the operator
        // we have a term of the form t=OP(t1,t2)
        let t2 = term.sub_terms.get(1).unwrap();
        // if the right sub-term has at its root the same operator,
        if &t2.operator == operator_at_root {
            // we have a term of the form t=OP(t1,OP(t21,t22))
            // we then return OP(OP(t1,t21),t22)
            let t1 = term.sub_terms.first().unwrap();
            let t21 = t2.sub_terms.first().unwrap();
            let t22 = t2.sub_terms.get(1).unwrap();
            let new_left_sub_term = LanguageTerm::new(
                operator_at_root.clone(),
                vec![
                    t1.clone(),
                    t21.clone()
                ]
            );
            let new_term = LanguageTerm::new(
                operator_at_root.clone(),
                vec![
                    new_left_sub_term,
                    t22.clone()
                ]
            );
            Some(new_term)
        } else {
            None
        }
    } else {
        None
    }
}


