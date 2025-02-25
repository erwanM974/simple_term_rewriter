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

use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};

use super::distributivity_checker::DistributivityChecker;








/**
Performs the following :
OP2(OP1(x,y),OP1(x,z)) -> OP1(x,OP2(y,z))
 **/
 pub(crate) fn transformation_factorize_left_distributive<
 LOS : RewritableLanguageOperatorSymbol
>(
 checker : &Box<dyn DistributivityChecker<LOS>>,
 term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {

 let op2 = &term.operator;
 if checker.is_binary(op2) {
     let left_sub_term = term.sub_terms.first().unwrap();
     let right_sub_term = term.sub_terms.get(1).unwrap();
     if left_sub_term.operator == right_sub_term.operator && checker.is_binary(&left_sub_term.operator) {
         // term is of the form OP2(OP1(a,y),OP1(b,z))
         let op1 = &left_sub_term.operator;
         if checker.is_left_distributive_over(op1,op2) {
             let a = left_sub_term.sub_terms.first().unwrap();
             let y = left_sub_term.sub_terms.get(1).unwrap();
             let b = right_sub_term.sub_terms.first().unwrap();
             let z = right_sub_term.sub_terms.get(1).unwrap();
             if a == b {
                 let new_right = LanguageTerm::new(
                     op2.clone(),
                     vec![
                         y.clone(),
                         z.clone()
                     ]
                 );
                 return Some(
                     LanguageTerm::new(
                         op1.clone(),
                         vec![
                             a.clone(),
                             new_right
                         ]
                     )
                 );
             }
         }
     }
 }
 None
}






/**
Performs the following :
OP2(OP1(y,x),OP1(z,x)) -> OP1(OP2(y,z),x)
 **/
 pub(crate) fn transformation_factorize_right_distributive<
 LOS : RewritableLanguageOperatorSymbol
>(
 checker : &Box<dyn DistributivityChecker<LOS>>,
 term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {

 let op2 = &term.operator;
 if checker.is_binary(op2) {
     let left_sub_term = term.sub_terms.first().unwrap();
     let right_sub_term = term.sub_terms.get(1).unwrap();
     if left_sub_term.operator == right_sub_term.operator && checker.is_binary(&left_sub_term.operator) {
         // term is of the form OP2(OP1(y,a),OP1(z,b))
         let op1 = &left_sub_term.operator;
         if checker.is_left_distributive_over(op1,op2) {
             let y = left_sub_term.sub_terms.first().unwrap();
             let a = left_sub_term.sub_terms.get(1).unwrap();
             let z = right_sub_term.sub_terms.first().unwrap();
             let b = right_sub_term.sub_terms.get(1).unwrap();
             if a == b {
                 let new_left = LanguageTerm::new(
                     op2.clone(),
                     vec![
                         y.clone(),
                         z.clone()
                     ]
                 );
                 return Some(
                     LanguageTerm::new(
                         op1.clone(),
                         vec![
                             new_left,
                             a.clone()
                         ]
                     )
                 );
             }
         }
     }
 }
 None
}
