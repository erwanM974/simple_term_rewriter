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



use crate::core::rule::RewriteRule;
use crate::core::terms::position::PositionInLanguageTerm;
use crate::core::terms::term::{LanguageTerm, RewritableLanguageOperatorSymbol};

use crate::builtin_trs::rules::flush::{AssociativityChecker, transformation_flush_to_the_left, transformation_flush_to_the_right};
use crate::builtin_trs::rules::reorder_commute::{BasicCommutativeCheckerAndOrderer, transformation_basic_reorder_subterms_under_commutative_operator};
use crate::builtin_trs::rules::simpl_binary::{GenericBinaryOperatorSimplifier, transformation_generic_simpl_under_binary_operator};
use crate::builtin_trs::rules::simpl_unary::{GenericUnaryOperatorSimplifier, transformation_generic_simpl_under_unary_operator};
use crate::builtin_trs::rules::modulo_associative_flattened_transfo::{ModuloAssociativeGenericFlattenedChecker, transformation_modulo_associative_generic_flattened_transfo};

use super::rules::factorization::defactorize::{transformation_defactorize_left_distributive, transformation_defactorize_right_distributive};
use super::rules::factorization::distributivity_checker::DistributivityChecker;
use super::rules::factorization::factorize_modulo_ac::{transformation_factorize_left_distributive_modulo_ac, transformation_factorize_right_distributive_modulo_ac};
use super::rules::factorization::factorize_simple::{transformation_factorize_left_distributive, transformation_factorize_right_distributive};
use super::rules::modulo_ac_reorderer::{transformation_modulo_assoc_partial_reordering, ModuloAssociativePartialReorderer};


pub enum BuiltinRewriteTransformationKind<LOS : RewritableLanguageOperatorSymbol> {
    // ***
    /// refer to [transformation_flush_to_the_right](transformation_flush_to_the_right)
    AssociativeFlushRight(Box<dyn AssociativityChecker<LOS>>),
    // ***
    /// refer to [transformation_flush_to_the_left](transformation_flush_to_the_left)
    AssociativeFlushLeft(Box<dyn AssociativityChecker<LOS>>),
    // ***
    /// refer to [transformation_basic_reorder_subterms_under_commutative_operator](transformation_basic_reorder_subterms_under_commutative_operator)
    ReorderOperandsIfCommuteBasic(Box<dyn BasicCommutativeCheckerAndOrderer<LOS>>),
    // ***
    /// refer to [transformation_modulo_assoc_partial_reordering](transformation_modulo_assoc_partial_reordering)
    ReorderOperandsIfCommuteModuloAC(Box<dyn ModuloAssociativePartialReorderer<LOS>>),
    // ***
    /// refer to [transformation_generic_simpl_under_unary_operator](transformation_generic_simpl_under_unary_operator)
    GenericSimplifyUnderUnary(Box<dyn GenericUnaryOperatorSimplifier<LOS>>),
    // ***
    /// refer to [transformation_generic_simpl_under_binary_operator](transformation_generic_simpl_under_binary_operator)
    GenericSimplifyUnderBinary(Box<dyn GenericBinaryOperatorSimplifier<LOS>>),
    // ***
    /// refer to [transformation_factorize_left_distributive](transformation_factorize_left_distributive)
    FactorizeLeftDistributiveSimple(Box<dyn DistributivityChecker<LOS>>),
    /// refer to [transformation_factorize_left_distributive_modulo_ac](transformation_factorize_left_distributive_modulo_ac)
    FactorizeLeftDistributiveModuloAC(Box<dyn DistributivityChecker<LOS>>),
    /// refer to [transformation_factorize_right_distributive](transformation_factorize_right_distributive)
    FactorizeRightDistributiveSimple(Box<dyn DistributivityChecker<LOS>>),
    /// refer to [transformation_factorize_right_distributive_modulo_ac](transformation_factorize_right_distributive_modulo_ac)
    FactorizeRightDistributiveModuloAC(Box<dyn DistributivityChecker<LOS>>),
    // ***
    /// refer to [transformation_defactorize_left_distributive](transformation_defactorize_left_distributive)
    DeFactorizeLeftDistributive(Box<dyn DistributivityChecker<LOS>>),
    /// refer to [transformation_defactorize_right_distributive](transformation_defactorize_right_distributive)
    DeFactorizeRightDistributive(Box<dyn DistributivityChecker<LOS>>),
    // ***
    /// refer to [transformation_modulo_associative_generic_flattened_transfo](transformation_modulo_associative_generic_flattened_transfo)
    ModuloAssociativeGenericFlattenedTransfo(Box<dyn ModuloAssociativeGenericFlattenedChecker<LOS>>)
}


pub struct BuiltinRewriteTransformation<LOS : RewritableLanguageOperatorSymbol> {
    pub kind : BuiltinRewriteTransformationKind<LOS>,
    pub desc : String
}

impl<LOS : RewritableLanguageOperatorSymbol> RewriteRule<LOS> for BuiltinRewriteTransformation<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn try_apply(&self,
                 term : &LanguageTerm<LOS>,
                 context_term : &LanguageTerm<LOS>,
                 position_in_context_term : &PositionInLanguageTerm
    ) -> Option<LanguageTerm<LOS>> {
        match &self.kind {
            BuiltinRewriteTransformationKind::AssociativeFlushRight(
                rule_application_checker
            ) => {
                transformation_flush_to_the_right::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::AssociativeFlushLeft(
                rule_application_checker
            ) => {
                transformation_flush_to_the_left::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::ReorderOperandsIfCommuteBasic(
                rule_application_checker
            ) => {
                transformation_basic_reorder_subterms_under_commutative_operator::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::ReorderOperandsIfCommuteModuloAC(
                rule_application_checker
            ) => {
                transformation_modulo_assoc_partial_reordering::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(
                rule_application_checker
            ) => {
                transformation_generic_simpl_under_unary_operator::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(
                rule_application_checker
            ) => {
                transformation_generic_simpl_under_binary_operator::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeLeftDistributiveModuloAC(
                rule_application_checker
            ) => {
                transformation_factorize_left_distributive_modulo_ac::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeLeftDistributiveSimple(
                rule_application_checker
            ) => {
                transformation_factorize_left_distributive::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::DeFactorizeLeftDistributive(
                rule_application_checker
            ) => {
                transformation_defactorize_left_distributive::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeRightDistributiveModuloAC(
                rule_application_checker
            ) => {
                transformation_factorize_right_distributive_modulo_ac::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeRightDistributiveSimple(
                rule_application_checker
            ) => {
                transformation_factorize_right_distributive::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::DeFactorizeRightDistributive(
                rule_application_checker
            ) => {
                transformation_defactorize_right_distributive::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
            BuiltinRewriteTransformationKind::ModuloAssociativeGenericFlattenedTransfo(
                rule_application_checker
            ) => {
                transformation_modulo_associative_generic_flattened_transfo::<LOS>(
                    rule_application_checker,
                    term,
                    context_term,
                    position_in_context_term
                )
            },
        }
    }
}



