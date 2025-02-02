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



use std::cmp::Ordering;
use crate::builtin_trs::interface::BuiltinTermRewritingInterface;
use crate::core::term::LanguageTerm;






/**
 Given a total order on the operator symbols, we derive a total order on the terms built using these operator symbols.
 **/
fn compare_terms<STRI : BuiltinTermRewritingInterface>(
        t1:&LanguageTerm<STRI::LanguageOperatorSymbol>,
        t2:&LanguageTerm<STRI::LanguageOperatorSymbol>
) -> std::cmp::Ordering {
    match STRI::compare_operators(&t1.operator,&t2.operator) {
        Ordering::Less => {
            Ordering::Less
        },
        Ordering::Equal => {
            let arity = STRI::op_arity(&t1.operator);
            assert_eq!(arity, STRI::op_arity(&t2.operator));
            for i in 0..arity {
                let sub_t1_at_i = t1.sub_terms.get(i).unwrap();
                let sub_t2_at_i = t2.sub_terms.get(i).unwrap();
                match compare_terms::<STRI>(sub_t1_at_i,sub_t2_at_i) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Equal => {
                        // do nothing
                    }
                }
            }
            Ordering::Equal
        },
        Ordering::Greater => {
            Ordering::Greater
        }
    }
}



/**
If op is a binary commutative operator given a total order < on the concrete terms,
if we have x < y for any two terms then this transformation performs:
op(y,x) -> op(x,y)
 **/
pub fn transformation_reorder_subterms_under_commutative_operator<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    // this must be applied to a binary commutative operator
    let precondition = (STRI::op_arity(operator_at_root) == 2)
        && STRI::op_is_binary_commutative(operator_at_root);
    // ***
    if precondition {
        let left_sub_term = term.sub_terms.first().unwrap();
        let right_sub_term = term.sub_terms.get(1).unwrap();
        match compare_terms::<STRI>(left_sub_term,right_sub_term) {
            Ordering::Greater => {
                // means that the left sub-term is greater than the right sub-term
                // so se should switch
                let new_term = LanguageTerm::new(
                    operator_at_root.clone(),
                    vec![
                        right_sub_term.clone(),
                        left_sub_term.clone()
                    ]
                );
                Some(new_term)
            },
            _ => {
                // means that the left sub-term is either equal or lower than the right
                // so the order in the term is already the good order
                None
            }
        }
    } else {
        None
    }
}