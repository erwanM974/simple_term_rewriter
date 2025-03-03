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


use crate::builtin_trs::util::{fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively, is_greater_as_per_lexicographic_path_ordering};
use crate::core::term::{LanguageTerm, RewritableLanguageOperatorSymbol};



 pub trait ModuloAssociativePartialReorderer<LOS : RewritableLanguageOperatorSymbol> {

    fn is_an_associative_partially_commutative_binary_operator_we_may_consider(
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

}



pub(crate) fn transformation_modulo_assoc_partial_reordering<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn ModuloAssociativePartialReorderer<LOS>>,
    term : &LanguageTerm<LOS>
) -> Option<LanguageTerm<LOS>> {

    if checker.is_an_associative_partially_commutative_binary_operator_we_may_consider(&term.operator) {
        let considered_ac_operator = &term.operator;
        let mut flattened_sub_terms : Vec<LanguageTerm<LOS>> = get_associative_sub_terms_recursively(
            term, 
            considered_ac_operator
        ).iter().copied().cloned().collect();
        // ***
        let mut has_changed_global = false;
        {
            // try to reorder the sub_terms that may commute as per the lexicographic path ordering
            let mut has_changed = true;
            while has_changed {
                has_changed = false;
                for i in 0..(flattened_sub_terms.len()-1) {
                    let s1 = flattened_sub_terms.get(i).unwrap();
                    let s2 = flattened_sub_terms.get(i+1).unwrap();
                    if checker.may_commute_under(
                        considered_ac_operator, 
                        s1,
                        s2
                    ) && is_greater_as_per_lexicographic_path_ordering(
                        s1, 
                        s2, 
                        &|x,y| checker.compare_operators(x,y),
                        &|x| checker.get_arity(x),
                    ) {
                        flattened_sub_terms.swap(i, i+1);
                        has_changed = true;
                        has_changed_global = true;
                    }
                }
            }
        }
        if has_changed_global {
            let folded_transformed = fold_associative_sub_terms_recursively(
                considered_ac_operator,
                &mut flattened_sub_terms, 
                &None
            );
            return Some(folded_transformed);
        }
    }
    // ***
    None
}



#[cfg(test)]
mod test {
    use crate::{builtin_trs::util::{fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively}, core::term::{LanguageTerm, RewritableLanguageOperatorSymbol}};

    use super::{transformation_modulo_assoc_partial_reordering, ModuloAssociativePartialReorderer};



    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    enum TestOperator {
        USIZE(usize),
        STOP,
        THEN
    }

    impl RewritableLanguageOperatorSymbol for TestOperator {}

    struct ReordererChecker {}

    impl ModuloAssociativePartialReorderer<TestOperator> for ReordererChecker {
        fn is_an_associative_partially_commutative_binary_operator_we_may_consider(
            &self, 
            op : &TestOperator
        ) -> bool {
            op == &TestOperator::THEN
        }
    
        fn may_commute_under(
            &self,
            parent_op :&TestOperator,
            left_sub_term : &crate::core::term::LanguageTerm<TestOperator>,
            right_sub_term : &crate::core::term::LanguageTerm<TestOperator>,
        ) -> bool {
            assert!(parent_op == &TestOperator::THEN);
            left_sub_term.operator != TestOperator::STOP && right_sub_term.operator != TestOperator::STOP
        }
    
        fn compare_operators(
            &self,
            op1 : &TestOperator,
            op2 : &TestOperator
        ) -> std::cmp::Ordering {
            match (op1,op2) {
                (TestOperator::USIZE(u1),TestOperator::USIZE(u2)) => {
                    usize::cmp(u1,u2)
                },
                (TestOperator::USIZE(_),_) => {
                    std::cmp::Ordering::Less
                },
                (_,TestOperator::USIZE(_)) => {
                    std::cmp::Ordering::Greater
                },
                (TestOperator::STOP,TestOperator::STOP) => {
                    std::cmp::Ordering::Equal
                },
                (TestOperator::STOP,_) => {
                    std::cmp::Ordering::Less
                },
                (_,TestOperator::STOP) => {
                    std::cmp::Ordering::Greater
                },
                (TestOperator::THEN,TestOperator::THEN) => {
                    std::cmp::Ordering::Equal
                }
            }
        }
    
        fn get_arity(
            &self,
            op : &TestOperator
        ) -> usize {
            match op {
                TestOperator::THEN => {
                    2
                },
                _ => {
                    0
                }
            }
        }
    }

    #[test]
    fn test_reordering() {
        let term = {
            let mut sub_terms : Vec<LanguageTerm<TestOperator>> = vec![
                TestOperator::USIZE(3),
                TestOperator::USIZE(9),
                TestOperator::USIZE(7),
                TestOperator::STOP,
                TestOperator::USIZE(4),
                TestOperator::USIZE(10),
                TestOperator::STOP,
                TestOperator::USIZE(2),
                TestOperator::USIZE(1),
                TestOperator::USIZE(6)
            ].into_iter().map(|x| LanguageTerm::new(x, vec![])).collect();
            fold_associative_sub_terms_recursively(
                &TestOperator::THEN, 
                &mut sub_terms, 
                &None
            )
        };
        let reordered : Box<dyn ModuloAssociativePartialReorderer<TestOperator>> = Box::new(ReordererChecker{});

        let got = transformation_modulo_assoc_partial_reordering(
            &reordered,
            &term
        );
        assert!(got.is_some());
        if let Some(reordered) = got {
            let flattened_reordered : Vec<TestOperator> = get_associative_sub_terms_recursively(
                &reordered, 
                &TestOperator::THEN
            ).into_iter().map(|x| x.operator.clone()).collect();
            assert_eq!(
                flattened_reordered,
                vec![
                    TestOperator::USIZE(3),
                    TestOperator::USIZE(7),
                    TestOperator::USIZE(9),
                    TestOperator::STOP,
                    TestOperator::USIZE(4),
                    TestOperator::USIZE(10),
                    TestOperator::STOP,
                    TestOperator::USIZE(1),
                    TestOperator::USIZE(2),
                    TestOperator::USIZE(6)
                ]
            );
        }
    }

}