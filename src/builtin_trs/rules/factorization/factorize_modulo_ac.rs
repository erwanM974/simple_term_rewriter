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




use std::{collections::{HashMap, HashSet}, vec};

use crate::{builtin_trs::util::{fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively}, core::term::{LanguageTerm, RewritableLanguageOperatorSymbol}};

use super::distributivity_checker::DistributivityChecker;





 pub(crate) fn transformation_factorize_left_distributive_modulo_ac<
 LOS : RewritableLanguageOperatorSymbol
>(
 checker : &Box<dyn DistributivityChecker<LOS>>,
 term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {

 let op2 = &term.operator;
 if checker.is_binary(op2) && checker.is_commutative(op2) {

    // ALT( x1, x2, ..., xn )
    let sub_terms = if checker.is_associative(op2) {
        get_associative_sub_terms_recursively(term, op2)
    } else {
        term.sub_terms.iter().collect()
    };

    // all possible binary head operators on the sub-terms that are left distributive over OP2
    let head_operators : HashSet<LOS> = sub_terms.iter()
        .filter(|x| checker.is_binary(&x.operator) && checker.is_left_distributive_over(&x.operator,op2))
        .map(|x| x.operator.clone()).collect();

    // looks for common factorizable sub_sub_terms on the left hand side of the sub_terms
    let mut new_factorized_sub_terms : Vec<LanguageTerm<LOS>> = vec![];

    let mut factorized_sub_terms_indices = HashSet::new();
    // ***
    // iter possible head operators
    for head_op in head_operators {
        // maps left-hand-sides to all matching right-hand-sides
        let mut found : HashMap<&LanguageTerm<LOS>, Vec<(usize,Vec<LanguageTerm<LOS>>)>> = HashMap::new();
        for (sub_term_id,sub_term) in sub_terms.iter().enumerate() {
            if sub_term.operator == head_op {
                // in ALT( x1, x2, ..., xn ) we have a xi = SEQ( xi1, ..., xim )
                let sub_sub_terms = if checker.is_associative(&head_op) {
                    get_associative_sub_terms_recursively(sub_term, &head_op)
                } else {
                    sub_term.sub_terms.iter().collect()
                };
                // put xi1 : [xi2,...,xim] in "found"
               let second_to_last = sub_sub_terms[1..].iter()
               .copied().cloned().collect();
                if found.contains_key(sub_sub_terms.first().unwrap()) {
                    let got = found.get_mut(sub_sub_terms.first().unwrap()).unwrap();
                    got.push( (sub_term_id,second_to_last) );
                } else {
                    found.insert(
                        sub_sub_terms.first().unwrap(), 
                        vec![ (sub_term_id,second_to_last) ]
                    );
                }
            } else {
                // in ALT( x1, x2, ..., xn ), we simply consider a whole xi as a left-hand-side factor
                if found.contains_key(*sub_term) {
                    let got = found.get_mut(*sub_term).unwrap();
                    got.push( (sub_term_id,vec![]) );
                } else {
                    found.insert(
                        *sub_term, 
                        vec![ (sub_term_id,vec![]) ]
                    );
                }
            }
        }
        for (lhs,rhss) in found {
            if rhss.len() > 1 {
                // here two rhs match the same lhs so we may factorize
                // in ALT( x1, x2, ..., xn ) we have at least two different xi and xj such that xi = SEQ(y, xi2, ...) and xj = SEQ(y, xj2, ...)
                // so we may factorize
                let mut rhss_as_terms : Vec<LanguageTerm<LOS>> = vec![];
                for (sub_term_id,mut rhs) in rhss {
                    factorized_sub_terms_indices.insert(sub_term_id);
                    // SEQ(xi2, ..., xim)
                    rhss_as_terms.push(
                        fold_associative_sub_terms_recursively(
                            &head_op, 
                            &mut rhs, 
                            &Some(checker.get_empty_operation_symbol())
                        )
                    );
                }
                // ALT( SEQ(xi2,...) , SEQ(xj2, ...) )
                let rhss_op2 = fold_associative_sub_terms_recursively(
                    op2, 
                    &mut rhss_as_terms, 
                    &Some(checker.get_empty_operation_symbol())
                );
                // SEQ( y, ALT( SEQ(xi2,...) , SEQ(xj2, ...) ) )
                let factorized = LanguageTerm::new(
                    head_op.clone(), 
                    vec![
                        lhs.clone(),
                        rhss_op2
                    ]
                );
                new_factorized_sub_terms.push(factorized);
            }
        }
    }
    // if we could do some factorization, then we have "factors" and "factorized_sub_terms_indices" not empty
    if !new_factorized_sub_terms.is_empty() {
        let mut new_sub_terms : Vec<LanguageTerm<LOS>> = vec![];
        for (sub_term_id,sub_term) in sub_terms.iter().enumerate() {
            if !factorized_sub_terms_indices.contains(&sub_term_id) {
                // not a sub_term we have factorized so we keep it as is
                new_sub_terms.push((*sub_term).clone());
            }
        }
        // we now add the new factorized sub_terms
        for new_factorized in new_factorized_sub_terms {
            new_sub_terms.push(new_factorized);
        }
        // we fold with the op2 operator
        let new_term = fold_associative_sub_terms_recursively(
            op2, 
            &mut new_sub_terms, 
            &Some(checker.get_empty_operation_symbol())
        );
        return Some(new_term);
    }
 }
 None 
}

     





pub(crate) fn transformation_factorize_right_distributive_modulo_ac<
LOS : RewritableLanguageOperatorSymbol
>(
checker : &Box<dyn DistributivityChecker<LOS>>,
term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {

let op2 = &term.operator;
if checker.is_binary(op2) && checker.is_commutative(op2) {

   // ALT( x1, x2, ..., xn )
   let sub_terms = if checker.is_associative(op2) {
       get_associative_sub_terms_recursively(term, op2)
   } else {
       term.sub_terms.iter().collect()
   };

   // all possible binary head operators on the sub-terms that are right distributive over OP2
   let head_operators : HashSet<LOS> = sub_terms.iter()
       .filter(|x| checker.is_binary(&x.operator) && checker.is_right_distributive_over(&x.operator,op2))
       .map(|x| x.operator.clone()).collect();

   // looks for common factorizable sub_sub_terms on the right hand side of the sub_terms
   let mut new_factorized_sub_terms : Vec<LanguageTerm<LOS>> = vec![];

   let mut factorized_sub_terms_indices = HashSet::new();
   // ***
   // iter possible head operators
   for head_op in head_operators {
       // maps right-hand-sides to all matching left-hand-sides
       let mut found : HashMap<&LanguageTerm<LOS>, Vec<(usize,Vec<LanguageTerm<LOS>>)>> = HashMap::new();
       for (sub_term_id,sub_term) in sub_terms.iter().enumerate() {
           if sub_term.operator == head_op {
               // in ALT( x1, x2, ..., xn ) we have a xi = SEQ( xi1, ..., xim )
               let sub_sub_terms = if checker.is_associative(&head_op) {
                   get_associative_sub_terms_recursively(sub_term, &head_op)
               } else {
                   sub_term.sub_terms.iter().collect()
               };
               // put xim : [xi1,...,xi(m-1)] in "found"
               let first_to_before_last = sub_sub_terms[0..(sub_sub_terms.len()-1)].iter()
                .copied().cloned().collect();
               if found.contains_key(sub_sub_terms.last().unwrap()) {
                   let got = found.get_mut(sub_sub_terms.last().unwrap()).unwrap();
                   got.push( (sub_term_id,first_to_before_last) );
               } else {
                   found.insert(
                       sub_sub_terms.last().unwrap(), 
                       vec![ (sub_term_id,first_to_before_last) ]
                   );
               }
           } else {
            // in ALT( x1, x2, ..., xn ), we simply consider a whole xi as a right-hand-side factor
            if found.contains_key(*sub_term) {
                let got = found.get_mut(*sub_term).unwrap();
                got.push( (sub_term_id,vec![]) );
            } else {
                found.insert(
                    *sub_term, 
                    vec![ (sub_term_id,vec![]) ]
                );
            }
        }
       }
       for (rhs,lhss) in found {
           if lhss.len() > 1 {
               // here two lhs match the same rhs so we may factorize
               // in ALT( x1, x2, ..., xn ) we have at least two different xi and xj such that xi = SEQ(xi1, ...,ximi, y) and xj = SEQ(xj1, ...,xjmj, y)
               // so we may factorize
               let mut lhss_as_terms : Vec<LanguageTerm<LOS>> = vec![];
               for (sub_term_id,mut lhs) in lhss {
                   factorized_sub_terms_indices.insert(sub_term_id);
                   // SEQ(xi1, ..., xi(m-1))
                   lhss_as_terms.push(
                       fold_associative_sub_terms_recursively(
                           &head_op, 
                           &mut lhs, 
                           &Some(checker.get_empty_operation_symbol())
                       )
                   );
               }
               // ALT( SEQ(xi1,...ximi) , SEQ(xj1, ...,xjmj) )
               let lhss_op2 = fold_associative_sub_terms_recursively(
                   op2, 
                   &mut lhss_as_terms, 
                   &Some(checker.get_empty_operation_symbol())
               );
               // SEQ( ALT( SEQ(xi1,...ximi) , SEQ(xj1, ...,xjmj) ), y )
               let factorized = LanguageTerm::new(
                   head_op.clone(), 
                   vec![
                        lhss_op2,
                        rhs.clone()
                   ]
               );
               new_factorized_sub_terms.push(factorized);
           }
       }
   }
   // if we could do some factorization, then we have "factors" and "factorized_sub_terms_indices" not empty
   if !new_factorized_sub_terms.is_empty() {
       let mut new_sub_terms : Vec<LanguageTerm<LOS>> = vec![];
       for (sub_term_id,sub_term) in sub_terms.iter().enumerate() {
           if !factorized_sub_terms_indices.contains(&sub_term_id) {
               // not a sub_term we have factorized so we keep it as is
               new_sub_terms.push((*sub_term).clone());
           }
       }
       // we now add the new factorized sub_terms
       for new_factorized in new_factorized_sub_terms {
           new_sub_terms.push(new_factorized);
       }
       // we fold with the op2 operator
       let new_term = fold_associative_sub_terms_recursively(
           op2, 
           &mut new_sub_terms, 
           &Some(checker.get_empty_operation_symbol())
       );
       return Some(new_term);
   }
}
None 
}

    



