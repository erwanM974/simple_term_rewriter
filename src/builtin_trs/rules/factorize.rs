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

/**
Something that can check an operator is associative.
 **/
pub trait DistributivityChecker<LOS : RewritableLanguageOperatorSymbol> {

    fn is_binary(&self, op : &LOS) -> bool;

    /**
    OP1 is left distributive over OP2 iff for any x, y and z:
    OP1(x,OP2(y,z)) = OP2(OP1(x,y),OP1(x,z))

    Example:
    Multiplication is left distributive over addition:
    "*(2,+(1,3)) = +(*(2,1),*(2,3))"
     **/
    fn is_left_distributive_over(&self, op1 : &LOS, op2 : &LOS) -> bool;

    /**
    OP1 is right distributive over OP2 iff for any x, y and z:
    OP1(OP2(y,z),x) = OP2(OP1(y,x),OP1(z,x))

    Example:
    Multiplication is right distributive over addition:
    "*(+(1,3),2) = +(*(1,2),*(3,2))"
     **/
    fn is_right_distributive_over(&self, op1 : &LOS, op2 : &LOS) -> bool;

}


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
OP1(x,OP2(y,z)) -> OP2(OP1(x,y),OP1(x,z))
 **/
pub(crate) fn transformation_defactorize_left_distributive<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn DistributivityChecker<LOS>>,
    term : &LanguageTerm<LOS>
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




/**
Performs the following :
OP1(OP2(y,z),x) -> OP2(OP1(y,x),OP1(z,x))
 **/
pub(crate) fn transformation_defactorize_right_distributive<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn DistributivityChecker<LOS>>,
    term : &LanguageTerm<LOS>
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





