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


use crate::builtin_trs::util::{fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively};
use crate::core::terms::position::PositionInLanguageTerm;
use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};


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

  With the above example, we have transformations such as:
  a + b + a + c + b + d -> a + b + c + d

  We also give the possibility of restricting that to subterms under a specific parent unary operator.
  For instance, to perform transformations such as:
  ( a* + b + c* + d )* -> ( a + b + c + d )*
 **/
 pub trait ModuloAssociativeGenericFlattenedChecker<LOS : RewritableLanguageOperatorSymbol> {

    fn is_an_associative_binary_operator_we_may_consider(
        &self, 
        op : &LOS
    ) -> bool;

    /** 
     * The transformation may require having a specific parent operator above the root (i.e. the immediate parent of the term's root in the term's context).
     * If this is the case, the "Fn(&LOS) -> bool" in the Option identifies it.
     * **/
    fn requires_a_specific_parent_operator(
        &self,
    ) -> Option<Box<dyn Fn(&LOS) -> bool>>;
    
    /** 
     * Transforms a term of the form:
     * - either Y(...,X(z_1,...,z_n),...) where X is binary associative
     * - or X(z_1,...,z_n) where X is binary associative
     * 
     * The return value consists of:
     * - Vec<LanguageTerm<LangOp> in both cases, which correspond to the z'_1,...z'_m which substitute the z_1,...,z_n
     * **/
    fn transform_flattened_sub_terms(
        &self, 
        considered_ac_op : &LOS, 
        considered_parent_op : Option<&LOS>,
        flattened_subterms : Vec<&LanguageTerm<LOS>>
    ) -> Option<Vec<LanguageTerm<LOS>>>;

}



fn ad_hoc_parent_checking_for_modulo_assoc_flattened_transfo<'a,LOS : RewritableLanguageOperatorSymbol>(
    checker : &Box<dyn ModuloAssociativeGenericFlattenedChecker<LOS>>,
    context_term : &'a LanguageTerm<LOS>,
    position_in_context_term : &PositionInLanguageTerm,
    considered_associative_operator : &LOS
) -> Option<Option<&'a LOS>> {
    match position_in_context_term.get_parent_position() {
        None => {
            // the position_in_context_term is the root so there is no parent at all
            // ***
            if checker.requires_a_specific_parent_operator().is_some() {
                // here the transformation requires a specific parent operator but there is no parent
                // so it cannot be applied
                None
            } else {
                // here the transformation can be applied
                // but there is no constraint on the parent operator
                Some(None)
            }
        },
        Some(parent_position) => {
            // here there is a parent in the context_term
            // and we get its operator
            let parent_operator = &context_term.get_sub_term_at_position(&parent_position).unwrap().operator;
            // ***
            if let Some(parent_requirement) = checker.requires_a_specific_parent_operator() {
                // if there is a requirement on the parent
                if parent_requirement(parent_operator) {
                    // if the parent satisfies this requirement then the transformation can be applied
                    // and we propagate the knowledge of the parent operator
                    Some(Some(parent_operator))
                } else {
                    // the parent does not satisfy the requirement so the transformation cannot be applied
                    None
                }
            } else {
                // there is no requirement on the parent
                if parent_operator == considered_associative_operator {
                    // if the parent is also the same operator, do not try applyng the transformation
                    // as it can be done from the parent
                    None
                } else {
                    Some(None)
                }
            }
            // ***
        }
    }
}


pub(crate) fn transformation_modulo_associative_generic_flattened_transfo<
    LOS : RewritableLanguageOperatorSymbol
>(
    checker : &Box<dyn ModuloAssociativeGenericFlattenedChecker<LOS>>,
    term : &LanguageTerm<LOS>,
    context_term : &LanguageTerm<LOS>,
    position_in_context_term : &PositionInLanguageTerm
) -> Option<LanguageTerm<LOS>> {

    if checker.is_an_associative_binary_operator_we_may_consider(&term.operator) {
        let considered_associative_operator = &term.operator;

        if let Some(if_req_parent_op) = ad_hoc_parent_checking_for_modulo_assoc_flattened_transfo(
            checker,context_term,position_in_context_term,considered_associative_operator
        ) {
            let flattened_sub_terms = get_associative_sub_terms_recursively(
                term, 
                considered_associative_operator
            );
            if let Some( mut transformed_flattened ) = checker.transform_flattened_sub_terms(
                considered_associative_operator, 
                if_req_parent_op,
                flattened_sub_terms
            ) {
                let folded_transformed = fold_associative_sub_terms_recursively(
                    considered_associative_operator,
                    &mut transformed_flattened, 
                    &None
                );
                let got = match if_req_parent_op {
                    None => {
                       folded_transformed
                    },
                    Some(parent_op) => {
                        LanguageTerm::new(
                            parent_op.clone(), 
                            vec![folded_transformed]
                        )
                    }
                };
                return Some(got);
            }
        }
    }
    // ***
    None
}