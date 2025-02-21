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


use crate::builtin_trs::util::is_greater_as_per_lexicographic_path_ordering;
use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};


/**
 Something that can check two sub-terms may be commuted when under a given root.
And that provide a total order on the language's operator symbols.
 **/
pub trait CommutativeCheckerAndOrderer<LOS : RewritableLanguageOperatorSymbol> {
    fn is_a_binary_operator_we_may_consider(
        &self,
        op : &LOS
    ) -> bool;
    fn may_commute_under(
        &self,
        parent_op :&LOS,
        left_sub_term : &LanguageTerm<LOS>,
        right_sub_term : &LanguageTerm<LOS>,
    ) -> bool;
    fn compare_operators(
        &self,
        op1 : &LOS,
        op2 : &LOS
    ) -> std::cmp::Ordering;
    fn get_arity(
        &self,
        op : &LOS
    ) -> usize;
    fn is_associative(
        &self,
        op : &LOS
    ) -> bool;
}






/**
If op is a binary commutative operator given a total order < on the concrete terms,
if we have x < y for any two terms then this transformation performs:
op(y,x) -> op(x,y)

If op is also associative, then we may also perform:
op(y,op(x,_)) -> op(x,op(y,_))
and 
op(op(_,y),x) -> op(op(_,x),y)
 **/
pub(crate) fn transformation_reorder_subterms_under_commutative_operator<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn CommutativeCheckerAndOrderer<LOS>>,
    term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {
    if !checker.is_a_binary_operator_we_may_consider(&term.operator) {
        return None;
    }
    //
    let considered_op = &term.operator;
    // we have a term of the form "op(t1,t2)"
    let left_sub_term = term.sub_terms.first().unwrap();
    let right_sub_term = term.sub_terms.get(1).unwrap();
    // this must be applied to a binary commutative operator
    if checker.may_commute_under(
        considered_op, 
        left_sub_term, 
        right_sub_term
    ) && is_greater_as_per_lexicographic_path_ordering::<LOS>(
        left_sub_term,
        right_sub_term, 
        &|x,y| checker.compare_operators(x,y),
        &|x| checker.get_arity(x),
    ) {
        // we may commute t1 and t2 and we have that t1 > t2 so we do so:
        // and return "op(t2,t1)"
        let new_term = LanguageTerm::new(
            considered_op.clone(),
            vec![
                right_sub_term.clone(),
                left_sub_term.clone()
            ]
        );
        return Some(new_term);
    }
    // ***
    if checker.is_associative(considered_op) {
        // if the operator is also associative, we may consider 
        // op(y,op(x,_)) -> op(x,op(y,_))
        // and op(op(_,y),x) -> op(op(_,x),y)
        if &right_sub_term.operator == considered_op {
           // we have a term of the form op(t1,op(t21,t22))
            let t21 = right_sub_term.sub_terms.first().unwrap();
            let t22 = right_sub_term.sub_terms.get(1).unwrap();
            if checker.may_commute_under(
                considered_op, 
                left_sub_term, 
                t21
            ) && is_greater_as_per_lexicographic_path_ordering::<LOS>(
                left_sub_term,
                t21, 
                &|x,y| checker.compare_operators(x,y),
                &|x| checker.get_arity(x),
            ) {
                // we may commute t1 and t21 and we have that t1 > t21 so we do so:
                // and return "op(t21,op(t1,y22))"
                let new_term = LanguageTerm::new(
                    considered_op.clone(),
                    vec![
                        t21.clone(),
                        LanguageTerm::new(
                            considered_op.clone(),
                            vec![
                                left_sub_term.clone(),
                                t22.clone()
                            ]
                        )
                    ]
                );
                return Some(new_term);
            }
        }
        // ***
        if &left_sub_term.operator == considered_op {
            // we have a term of the form op(op(t11,t12),t2)
             let t11 = left_sub_term.sub_terms.first().unwrap();
             let t12 = left_sub_term.sub_terms.get(1).unwrap();
             if checker.may_commute_under(
                 considered_op, 
                 t12, 
                 right_sub_term
             ) && is_greater_as_per_lexicographic_path_ordering::<LOS>(
                t12,
                 right_sub_term, 
                 &|x,y| checker.compare_operators(x,y),
                 &|x| checker.get_arity(x),
             ) {
                 // we may commute t12 and t2 and we have that t12 > t2 so we do so:
                 // and return "op(op(t11,t2),t12)"
                 let new_term = LanguageTerm::new(
                     considered_op.clone(),
                     vec![
                         LanguageTerm::new(
                             considered_op.clone(),
                             vec![
                                t11.clone(),
                                right_sub_term.clone()
                             ]
                         ),
                         t12.clone()
                     ]
                 );
                 return Some(new_term);
             }
         }
        // ***
    }
    None 
}