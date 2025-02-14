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
 Something that can simplify a term which root is a unary operator.
 **/
pub trait GenericUnaryOperatorSimplifier<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    fn is_unary(&self, op : &LanguageOperatorSymbol) -> bool;

    /**
     Try to simplify a term which root is a unary operator.
    This can be used to:
        - simplify fixed points under unary operators
        - merge nested unary operators
    More complex cases can also be implemented via this trait.
    Examples :
    - in integer computation : try_compose_nested_unary_operators( .^3 , 1 ) will return Some(1) because 1
      is a fixed point for the power operator
    - in sequence diagrams : try_compose_nested_unary_operators( loop, assert(i) ) will return None because it is not possible to
      merge a "loop" and an "assert"
    - in integer computation : try_compose_nested_unary_operators( .^3 , x^5 ) will return Some(Some(x^8)) because,
      we can compose the power operators
    - in boolean logic : try_compose_nested_unary_operators( ¬ , ¬(x) ) will return Some(x) because, because,
      we if we compose ¬ and ¬, we get the identity operator
     **/
    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &LanguageOperatorSymbol,
        term_underneath : &LanguageTerm<LanguageOperatorSymbol>
    ) -> Option<LanguageTerm<LanguageOperatorSymbol>>;
}




/**
 *  This can be used to simplify a term which has a unary operator at its root.
 *  Two examples below.
 *  
 *  (1) If op and op' are two unary operators and if there exists
 *  an op'' such that for any x, op(op'(x)) is equivalent to op''(x)
 *  then we may rewrite :
 *  op(op'(x)) -> op''(x)
 *  
 *  A typical example of that is the power operation on real numbers:
 *  we have, for any reals y and z:
 *  - denoting the operator x->x^y as F
 *  - denoting the operator x->x^z as G
 *  - denoting the operator x->x^(y+z) as H
 * 
 *  We have, for any real x G(F(x)) = H(x) so we may perform the rewrite operation:
 *  G(F(x)) -> H(x)
 *  which is semantically correct
 *  
 *  (2) If c is a constant (operator with arity 0) and it is a fixpoint for a unary operator op
 *  then we may rewrite :
 *  - op(c) -> c
 *  
 * A typical example of that is fixpoints for unary operators, for example:
 * - for any real x, 1^x = 1
 **/
pub(crate) fn transformation_generic_simpl_under_unary_operator<
    LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash
>(
    checker : &Box<dyn GenericUnaryOperatorSimplifier<LanguageOperatorSymbol>>,
    term : &LanguageTerm<LanguageOperatorSymbol>
) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;

    if checker.is_unary(operator_at_root) {
        let sub_term = term.sub_terms.first().unwrap();
        checker.try_simplify_under_unary_operator(operator_at_root, sub_term)
    } else {
        None
    }
}


