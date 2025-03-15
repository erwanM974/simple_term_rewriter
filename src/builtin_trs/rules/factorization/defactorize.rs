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





use crate::core::terms::{position::PositionInLanguageTerm, term::{LanguageTerm, RewritableLanguageOperatorSymbol}};

use super::distributivity_checker::DistributivityChecker;





/**
Performs the following :
OP1(x,OP2(y,z)) -> OP2(OP1(x,y),OP1(x,z))
 **/
pub(crate) fn transformation_defactorize_left_distributive<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn DistributivityChecker<LOS>>,
    term : &LanguageTerm<LOS>,
    _context_term : &LanguageTerm<LOS>,
    _position_in_context_term : &PositionInLanguageTerm
) -> Option<LanguageTerm<LOS>> {

    let op1 = &term.operator;
    if checker.is_binary(op1) {
        let x = term.sub_terms.first().unwrap();
        let right_sub_term = term.sub_terms.get(1).unwrap();
        if checker.is_binary(&right_sub_term.operator) {
            // term is of the form OP1(x,OP2(y,z))
            let op2 = &right_sub_term.operator;
            if checker.is_left_distributive_over(op1,op2) {
                let y = right_sub_term.sub_terms.first().unwrap();
                let z = right_sub_term.sub_terms.get(1).unwrap();
                // OP1(x,OP2(y,z)) -> OP2(OP1(x,y),OP1(x,z))
                let new_left = LanguageTerm::new(
                    op1.clone(),
                    vec![
                        x.clone(),
                        y.clone()
                    ]
                );
                let new_right = LanguageTerm::new(
                    op1.clone(),
                    vec![
                        x.clone(),
                        z.clone(),
                    ]
                );
                return Some(
                    LanguageTerm::new(
                        op2.clone(),
                        vec![
                            new_left,
                            new_right
                        ]
                    )
                );
            }
        }
    }
    None
}




/**
Performs the following :
OP1(OP2(y,z),x) -> OP2(OP1(y,x),OP1(z,x))
 **/
pub(crate) fn transformation_defactorize_right_distributive<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn DistributivityChecker<LOS>>,
    term : &LanguageTerm<LOS>,
    _context_term : &LanguageTerm<LOS>,
    _position_in_context_term : &PositionInLanguageTerm
) -> Option<LanguageTerm<LOS>> {

    let op1 = &term.operator;
    if checker.is_binary(op1) {
        let left_sub_term = term.sub_terms.first().unwrap();
        let x = term.sub_terms.get(1).unwrap();
        if checker.is_binary(&left_sub_term.operator) {
            // term is of the form OP1(OP2(y,z),x)
            let op2 = &left_sub_term.operator;
            if checker.is_right_distributive_over(op1,op2) {
                let y = left_sub_term.sub_terms.first().unwrap();
                let z = left_sub_term.sub_terms.get(1).unwrap();
                // OP1(OP2(y,z),x) -> OP2(OP1(y,x),OP1(z,x))
                let new_left = LanguageTerm::new(
                    op1.clone(),
                    vec![
                        y.clone(),
                        x.clone()
                    ]
                );
                let new_right = LanguageTerm::new(
                    op1.clone(),
                    vec![
                        z.clone(),
                        x.clone()
                    ]
                );
                return Some(
                    LanguageTerm::new(
                        op2.clone(),
                        vec![
                            new_left,
                            new_right
                        ]
                    )
                );
            }
        }
    }
    None
}


