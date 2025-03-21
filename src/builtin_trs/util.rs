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

use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};





pub fn get_associative_sub_terms_recursively<'a, LOS : RewritableLanguageOperatorSymbol>(
    term : &'a LanguageTerm<LOS>,
    considered_associative_operator : &LOS
) -> Vec<&'a LanguageTerm<LOS>> {
    // ***
    let mut sub_terms : Vec<&'a LanguageTerm<LOS>> = Vec::new();
    if &term.operator == considered_associative_operator {
        for sub_term in &term.sub_terms {
            sub_terms.extend( get_associative_sub_terms_recursively(sub_term, considered_associative_operator) );
        }
    } else {
        sub_terms.push(term);
    }
    sub_terms
}


pub fn fold_associative_sub_terms_recursively<LOS : RewritableLanguageOperatorSymbol>(
    considered_associative_operator : &LOS,
    sub_terms : &mut Vec<LanguageTerm<LOS>>,
    default_empty_term : &Option<LOS>
) -> LanguageTerm<LOS> {
    let sub_terms_num = sub_terms.len();
    match sub_terms_num {
        2 => {
            let t2 = sub_terms.pop().unwrap();
            let t1 = sub_terms.pop().unwrap();
            LanguageTerm::new(
                considered_associative_operator.clone(), 
                vec![t1,t2]
            )
        },
        1 => {
            sub_terms.pop().unwrap()
        },
        0 => {
            if let Some(empty_op) = default_empty_term {
                LanguageTerm::new(empty_op.clone(), vec![])
            } else {
                panic!("when folding sub-terms recursively, encountered an empty list, and no empty term is specified");
            }
        },
        _ => {
            let t1 = sub_terms.remove(0);
            let t2 = fold_associative_sub_terms_recursively(
                considered_associative_operator,
                sub_terms,
                default_empty_term
            );
            LanguageTerm::new(
                considered_associative_operator.clone(), 
                vec![t1,t2]
            )
        }
    }
}




pub fn lexicographic_path_ordering<LOS : RewritableLanguageOperatorSymbol>(
    s : &LanguageTerm<LOS>,
    t : &LanguageTerm<LOS>,
    compare_operators : &dyn Fn(&LOS,&LOS) -> Ordering,
    get_arity : &dyn Fn(&LOS) -> usize
) -> std::cmp::Ordering {
    if s == t {
        std::cmp::Ordering::Equal
    } else if is_greater_as_per_lexicographic_path_ordering(
        s, 
        t, 
        compare_operators, 
        get_arity
    ) {
        std::cmp::Ordering::Greater
    } else {
        debug_assert!(
            is_greater_as_per_lexicographic_path_ordering(
                t, 
                s, 
                compare_operators, 
                get_arity
            )
        );
        std::cmp::Ordering::Less
    }
}



/**
 Given a total order on the operator symbols, we derive a total order on the terms built using these operator symbols.
 **/
pub fn is_greater_as_per_lexicographic_path_ordering<LOS : RewritableLanguageOperatorSymbol>(
    s : &LanguageTerm<LOS>,
    t : &LanguageTerm<LOS>,
    compare_operators : &dyn Fn(&LOS,&LOS) -> Ordering,
    get_arity : &dyn Fn(&LOS) -> usize
) -> bool {
    match compare_operators(&s.operator,&t.operator) {
        Ordering::Greater => {
            // s dominates t if s dominates each of t's subterms
            let mut is_greater = true;
            'iter_tjs : for j in 0..get_arity(&t.operator) {
                let tj = t.sub_terms.get(j).unwrap();
                if !is_greater_as_per_lexicographic_path_ordering(s, tj, compare_operators, get_arity) {
                    is_greater = false;
                    break 'iter_tjs;
                }
            }
            if is_greater {
                return true;
            }
        },
        Ordering::Less => {
            // s dominates t if one of s's subterms dominates t
            for i in 0..get_arity(&s.operator) {
                let si = s.sub_terms.get(i).unwrap();
                if si == t || is_greater_as_per_lexicographic_path_ordering(si, t, compare_operators, get_arity) {
                    return true;
                }
            }
        },
        Ordering::Equal => {
            // the fact that s.operator and t.operator are equal
            // makes so that one must consider their sub-terms
            // because s and t have the same root operator, they have the same number of sub-terms, 
            // which is the arity of that operator
            for i in 0..get_arity(&s.operator) {
                let si = s.sub_terms.get(i).unwrap();
                let ti = t.sub_terms.get(i).unwrap();
                if is_greater_as_per_lexicographic_path_ordering(si, ti,compare_operators,get_arity) {
                    return true;
                }
                if is_greater_as_per_lexicographic_path_ordering(ti, si,compare_operators,get_arity) {
                    return false;
                }
            }
        },
    }
    false 
}