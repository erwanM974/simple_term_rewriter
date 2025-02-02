/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

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

use crate::{builtin_trs::interface::BuiltinTermRewritingInterface, core::term::LanguageTerm};







/** 
 * If OP1 and OP2 are binary operators.
 * And if OP1 distributes over OP2, this means that for any sub-terms x, y and z, 
 * we have OP1(OP2(x,y),OP2(x,z)) equivalent to OP2(x,OP1(y,z))
 * 
 * This transformation performs:
 * OP1(OP2(x,y),OP2(x,z)) -> OP2(x,OP1(y,z))
 * **/
pub fn transformation_factorize_left_distributive<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    if STRI::op_arity(operator_at_root) == 2 {
        let t1 = term.sub_terms.first().unwrap();
        let t2 = term.sub_terms.get(1).unwrap();
        // ***
        let precondition = (t1.operator == t2.operator) && 
        (STRI::op_arity(&t1.operator) == 2) &&
        (STRI::op_distributes_over(operator_at_root, &t1.operator));
        if precondition {
            let t11 = t1.sub_terms.first().unwrap();
            let t21 = t2.sub_terms.first().unwrap();
            if t11 == t21 {
                // we have a term of the form OP1(OP2(x,y),OP2(x,z))
                let t12 = t1.sub_terms.get(1).unwrap();
                let t22 = t2.sub_terms.get(1).unwrap();
                let new_right = LanguageTerm::new(
                    operator_at_root.clone(),
                    vec![
                        t12.clone(),
                        t22.clone()
                    ]
                );
                let new_term = LanguageTerm::new(
                    t1.operator.clone(),
                    vec![
                        t11.clone(),
                        new_right
                    ]
                );
                return Some(new_term);
            }
        }
    }
    None 
}





/** 
 * If OP1 and OP2 are binary operators.
 * And if OP1 distributes over OP2, this means that for any sub-terms x, y and z, 
 * we have OP1(OP2(y,x),OP2(z,x)) equivalent to OP2(OP1(y,z),x)
 * 
 * This transformation performs:
 * OP1(OP2(y,x),OP2(z,x)) -> OP2(OP1(y,z),x)
 * **/
 pub fn transformation_factorize_right_distributive<STRI : BuiltinTermRewritingInterface>(
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>
) -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
    let operator_at_root = &term.operator;
    if STRI::op_arity(operator_at_root) == 2 {
        let t1 = term.sub_terms.first().unwrap();
        let t2 = term.sub_terms.get(1).unwrap();
        // ***
        let precondition = (t1.operator == t2.operator) && 
        (STRI::op_arity(&t1.operator) == 2) &&
        (STRI::op_distributes_over(operator_at_root, &t1.operator));
        if precondition {
            let t12 = t1.sub_terms.get(1).unwrap();
            let t22 = t2.sub_terms.get(1).unwrap();
            if t12 == t22 {
                // we have a term of the form OP1(OP2(y,x),OP2(z,x))
                let t11 = t1.sub_terms.first().unwrap();
                let t21 = t2.sub_terms.first().unwrap();
                let new_left = LanguageTerm::new(
                    operator_at_root.clone(),
                    vec![
                        t11.clone(),
                        t21.clone()
                    ]
                );
                let new_term = LanguageTerm::new(
                    t1.operator.clone(),
                    vec![
                        new_left,
                        t12.clone()
                    ]
                );
                return Some(new_term);
            }
        }
    }
    None 
}

