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

use std::hash::Hash;

use crate::core::term::LanguageTerm;


/**
  Let us consider the following example:
  (x \/ (y \/ z)) \/ ((z \/ y) \/ x)
  Is is equivalent to (x \/ (y \/ z))
  We can rewrite the former into the latter via a rule of the form "(x \/ x) -> x" modulo Associativity and Commutativity of "\/".
  We can implement such rewriting as follows:
  We flatten the term to obtain a sequence of subterms:
  In our example, from "(x \/ (y \/ z)) \/ ((z \/ y) \/ x)", we get "vec![x,y,z,z,y,x]".
  "Flattening" us formalized in the litterature via a set of "flattening rules".
  In any case, once we have this, we can implement the actual rewriting as removing duplicated elements from the vector.

  This trait corresponds to something that will:
  - given a specific associative operator, compute that vector (if possible)
  - and perform a specific transformation on that vector (if possible)
 **/
 pub trait ModuloAssociativeFlattenedChecker<LangOp : Clone + PartialEq + Eq + Hash> {

    fn is_an_associative_binary_operator_we_may_consider(
        &self, 
        op : &LangOp
    ) -> bool;
    
    fn transform_flattened_sub_terms(
        &self, 
        considered_ac_op : &LangOp, 
        flattened_subterms : Vec<&LanguageTerm<LangOp>>
    ) -> Option<Vec<LanguageTerm<LangOp>>>;

}



fn get_associative_sub_terms_recursively<'a, LangOp : Clone + PartialEq + Eq + Hash>(
    term : &'a LanguageTerm<LangOp>,
    considered_associative_operator : &LangOp
) -> Vec<&'a LanguageTerm<LangOp>> {
    // ***
    let mut sub_terms : Vec<&'a LanguageTerm<LangOp>> = Vec::new();
    if &term.operator == considered_associative_operator {
        for sub_term in &term.sub_terms {
            sub_terms.extend( get_associative_sub_terms_recursively(sub_term, considered_associative_operator) );
        }
    } else {
        sub_terms.push(term);
    }
    sub_terms
}


fn fold_associative_sub_terms_recursively<LangOp : Clone + PartialEq + Eq + Hash>(
    considered_associative_operator : &LangOp,
    sub_terms : &mut Vec<LanguageTerm<LangOp>>
) -> LanguageTerm<LangOp> {
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
            panic!("when using the modulo associativity flattened transformation rule, one should not return and empty list as the transformed flattened sub-terms");
        },
        _ => {
            let t1 = sub_terms.remove(0);
            let t2 = fold_associative_sub_terms_recursively(
                considered_associative_operator,
                sub_terms
            );
            LanguageTerm::new(
                considered_associative_operator.clone(), 
                vec![t1,t2]
            )
        }
    }
}



pub(crate) fn transformation_modulo_associative_flattened_transfo<
    LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash
>(
    checker : &Box<dyn ModuloAssociativeFlattenedChecker<LanguageOperatorSymbol>>,
    term : &LanguageTerm<LanguageOperatorSymbol>
) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
    if checker.is_an_associative_binary_operator_we_may_consider(&term.operator) {
        let considered_associative_operator = &term.operator;
        let flattened_sub_terms = get_associative_sub_terms_recursively(term, considered_associative_operator);
        checker.transform_flattened_sub_terms(
            considered_associative_operator, 
            flattened_sub_terms
        ).map(
            |mut transformed_flattened| 
            fold_associative_sub_terms_recursively(
                considered_associative_operator,
                &mut transformed_flattened
            )
        )
    } else {
        None
    }
}