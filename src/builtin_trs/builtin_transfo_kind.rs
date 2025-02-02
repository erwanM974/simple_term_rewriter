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



use std::fmt;
use crate::builtin_trs::interface::BuiltinTermRewritingInterface;
use crate::builtin_trs::rules::binary_idempotence::transformation_eliminate_duplicates_under_binary_idempotent_operator;
use crate::builtin_trs::rules::flush::{transformation_flush_to_the_left, transformation_flush_to_the_right};
use crate::builtin_trs::rules::reorder_commute::transformation_reorder_subterms_under_commutative_operator;
use crate::builtin_trs::rules::simpl_neutral_elements::{transformation_simpl_fixpoint_under_unary_operator, transformation_simpl_neutral_under_binary_operator};
use crate::builtin_trs::rules::unary_composition::transformation_simpl_compositions_of_unary_operators;
use crate::core::rule::RewriteRule;
use crate::core::term::LanguageTerm;

use crate::builtin_trs::rules::defactorize::{transformation_defactorize_left_distributive, transformation_defactorize_right_distributive};
use crate::builtin_trs::rules::factorize::{transformation_factorize_left_distributive, transformation_factorize_right_distributive};


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuiltinRewriteTransformations {
    // ***
    /// refer to [transformation_flush_to_the_right](transformation_flush_to_the_right)
    AssociativeFlushRight,
    // ***
    /// refer to [transformation_flush_to_the_left](transformation_flush_to_the_left)
    AssociativeFlushLeft,
    // ***
    /// refer to [transformation_reorder_subterms_under_commutative_operator](transformation_reorder_subterms_under_commutative_operator)
    ReorderOperandsIfCommutative,
    // ***
    /// refer to [transformation_simpl_neutral_under_binary_operator](transformation_simpl_neutral_under_binary_operator)
    SimplifyBinaryNeutral,
    // ***
    /// refer to [transformation_simpl_fixpoint_under_unary_operator](transformation_simpl_fixpoint_under_unary_operator)
    SimplifyUnaryFixpoint,
    // ***
    /// refer to [transformation_simpl_compositions_of_unary_operators](transformation_simpl_compositions_of_unary_operators)
    SimplifyCompositionsOfUnaryOperators,
    // ***
    /// refer to [transformation_eliminate_duplicates_under_binary_idempotent_operator](transformation_eliminate_duplicates_under_binary_idempotent_operator)
    DeduplicateUnderBinaryIdempotent,
    // ***
    // refer to [transformation_factorize_left_distributive](transformation_factorize_left_distributive) 
    FactorizeLeftDistributive,
    // refer to [transformation_defactorize_left_distributive](transformation_defactorize_left_distributive) 
    DeFactorizeLeftDistributive,
    // refer to [transformation_factorize_right_distributive](transformation_factorize_right_distributive)
    FactorizeRightDistributive,
    // refer to [transformation_defactorize_right_distributive](transformation_defactorize_right_distributive) 
    DeFactorizeRightDistributive,
}



impl fmt::Display for BuiltinRewriteTransformations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}





impl<STRI : BuiltinTermRewritingInterface> RewriteRule<STRI> for BuiltinRewriteTransformations {
    fn get_transformation_kind(&self) -> STRI::TransformationKind {
        STRI::from_builtin_to_generic_transformation_kind(self)
    }

    fn try_apply(&self, term : &LanguageTerm<STRI::LanguageOperatorSymbol>)
                 -> Option<LanguageTerm<STRI::LanguageOperatorSymbol>> {
        match self {
            BuiltinRewriteTransformations::AssociativeFlushRight => {
                transformation_flush_to_the_right::<STRI>(term)
            },
            BuiltinRewriteTransformations::AssociativeFlushLeft => {
                transformation_flush_to_the_left::<STRI>(term)
            },
            BuiltinRewriteTransformations::ReorderOperandsIfCommutative => {
                transformation_reorder_subterms_under_commutative_operator::<STRI>(term)
            }
            BuiltinRewriteTransformations::SimplifyBinaryNeutral => {
                transformation_simpl_neutral_under_binary_operator::<STRI>(term)
            },
            BuiltinRewriteTransformations::SimplifyUnaryFixpoint => {
                transformation_simpl_fixpoint_under_unary_operator::<STRI>(term)
            },
            BuiltinRewriteTransformations::SimplifyCompositionsOfUnaryOperators => {
                transformation_simpl_compositions_of_unary_operators::<STRI>(term)
            }
            BuiltinRewriteTransformations::DeduplicateUnderBinaryIdempotent => {
                transformation_eliminate_duplicates_under_binary_idempotent_operator::<STRI>(term)
            }
            BuiltinRewriteTransformations::FactorizeLeftDistributive => {
                transformation_factorize_left_distributive::<STRI>(term)
            },
            BuiltinRewriteTransformations::DeFactorizeLeftDistributive => {
                transformation_defactorize_left_distributive::<STRI>(term)
            },
            BuiltinRewriteTransformations::FactorizeRightDistributive => {
                transformation_factorize_right_distributive::<STRI>(term)
            },
            BuiltinRewriteTransformations::DeFactorizeRightDistributive => {
                transformation_defactorize_right_distributive::<STRI>(term)
            }
        }
    }
}



