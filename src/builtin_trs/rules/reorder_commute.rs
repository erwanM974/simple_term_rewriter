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
use std::hash::Hash;
use crate::core::term::LanguageTerm;


/**
 Something that can check two sub-terms may be commuted when under a given root.
And that provide a total order on the language's operator symbols.
 **/
pub trait CommutativeCheckerAndOrderer<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    fn may_commute_under(
        &self,
        root_term : &LanguageTerm<LanguageOperatorSymbol>
    ) -> bool;
    fn compare_operators(
        &self,
        op1 : &LanguageOperatorSymbol,
        op2 : &LanguageOperatorSymbol
    ) -> std::cmp::Ordering;
    fn get_arity(
        &self,
        op : &LanguageOperatorSymbol
    ) -> usize;
}


/**
 Given a total order on the operator symbols, we derive a total order on the terms built using these operator symbols.
 **/
fn compare_terms<Lop : Clone + PartialEq + Eq + Hash>(
    t1 : &LanguageTerm<Lop>,
    t2 : &LanguageTerm<Lop>,
    checker : &Box<dyn CommutativeCheckerAndOrderer<Lop>>
) -> std::cmp::Ordering {
    match checker.compare_operators(&t1.operator,&t2.operator) {
        Ordering::Less => {
            Ordering::Less
        },
        Ordering::Equal => {
            let arity = checker.get_arity(&t1.operator);
            assert_eq!(arity, checker.get_arity(&t2.operator));
            for i in 0..arity {
                let sub_t1_at_i = t1.sub_terms.get(i).unwrap();
                let sub_t2_at_i = t2.sub_terms.get(i).unwrap();
                match compare_terms::<Lop>(sub_t1_at_i,sub_t2_at_i, checker) {
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
pub(crate) fn transformation_reorder_subterms_under_commutative_operator<
    LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash
>(
    checker : &Box<dyn CommutativeCheckerAndOrderer<LanguageOperatorSymbol>>,
    term : &LanguageTerm<LanguageOperatorSymbol>
) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
    // this must be applied to a binary commutative operator
    let precondition = checker.may_commute_under(term);
    // ***
    if precondition {
        let operator_at_root = &term.operator;
        let left_sub_term = term.sub_terms.first().unwrap();
        let right_sub_term = term.sub_terms.get(1).unwrap();
        match compare_terms::<LanguageOperatorSymbol>(left_sub_term,right_sub_term, checker) {
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