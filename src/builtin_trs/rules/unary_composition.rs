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
If op and op' are two unary operators and if there exists
an op'' such that for any x, op(op'(x)) is equivalent to op''(x)
then this transformation performs
op(op'(x)) -> op''(x)

A typical example of that is the power operation on real numbers:
we have, for any reals y and z:
- denoting the operator x->x^y as F
- denoting the operator x->x^z as G
- denoting the operator x->x^(y+z) as H
We have, for any real x G(F(x)) = H(x) so we may perform the rewrite operation:
G(F(x)) -> H(x)
which is semantically correct
 **/
pub fn transformation_simpl_compositions_of_unary_operators<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;

    if STRI::op_arity(operator_at_root) == 1 {
        let sub_term = term.sub_terms.first().unwrap();
        if STRI::op_arity(&sub_term.operator) == 1 {
            // we have a term of the form G(F(t))
            if let Some(new_composed_parent_operator) = STRI::compose_nested_unary_operators(
                operator_at_root,&sub_term.operator
            ) {
                let sub_sub_term = sub_term.sub_terms.first().unwrap();
                let new_term = LanguageTerm::new(
                    new_composed_parent_operator,
                    vec![sub_sub_term.clone()]
                );
                return Some(new_term);
            }
        }
    }
    None
}