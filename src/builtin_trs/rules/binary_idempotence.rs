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
    If op is a binary idempotent operator this means that for any x, op(x,x) is equivalent to x
    If this is the case, then this transformation performs
    op(x,x) -> x

    If op is also associative, for any y, this transformation may also perform
    either : op(x,op(x,y)) -> op(x,y)
    or     : op(op(y,x),x) -> op(y,x)

    As a side note, this works nicely when in combined use with:
    - (AssociativeFlushRight OR AssociativeFlushLeft) (other builtin transformations in this crate)
    Doing so, we benefit from a form of ""rewriting modulo associativity"".

    If we also combine it with
    - ReorderOperandsIfCommutative (another builtin transformations in this crate)
    we benefit from a form of ""rewriting modulo AC"".
 **/
pub fn transformation_eliminate_duplicates_under_binary_idempotent_operator<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary idempotent operator
    let precondition = (STRI::op_arity(operator_at_root) == 2)
        && STRI::op_is_binary_idempotent(operator_at_root);
    // ***
    if precondition {
        // we have a term of the form OP(t1,t2) with OP a binary idempotent operator
        let t1 = term.sub_terms.first().unwrap();
        let t2 = term.sub_terms.get(1).unwrap();
        if t1 == t2 {
            // if the left and right sub-terms are equal, because the parent operator is idempotent,
            // we rewrite the whole term keeping only one copy of the duplicated sub-term
            // i.e., performs op(x,x) -> x
            return Some(t1.clone());
        }
        if STRI::op_is_binary_associative(operator_at_root) {
            // if the operator is also associative
            if &t2.operator == operator_at_root {
                // we have a term of the form t=OP(t1,OP(t21,t22))
                let t21 = t2.sub_terms.first().unwrap();
                if t1 == t21 {
                    // performs op(x,op(x,y)) -> op(x,y)
                    return Some(t2.clone());
                }
            }
            if &t1.operator == operator_at_root {
                // we have a term of the form t=OP(OP(t11,t12),t2)
                let t12 = t1.sub_terms.get(1).unwrap();
                if t12 == t2 {
                    // performs op(op(y,x),x) -> op(y,x)
                    return Some(t1.clone());
                }
            }
        }

    }
    None
}








