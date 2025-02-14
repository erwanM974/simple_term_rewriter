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

use crate::core::rule::RewriteRule;
use crate::core::term::LanguageTerm;

use crate::builtin_trs::rules::factorize::{DistributivityChecker, transformation_defactorize_left_distributive, transformation_defactorize_right_distributive, transformation_factorize_left_distributive, transformation_factorize_right_distributive};
use crate::builtin_trs::rules::flush::{AssociativityChecker, transformation_flush_to_the_left, transformation_flush_to_the_right};
use crate::builtin_trs::rules::reorder_commute::{CommutativeCheckerAndOrderer, transformation_reorder_subterms_under_commutative_operator};
use crate::builtin_trs::rules::simpl_binary::{GenericBinaryOperatorSimplifier, transformation_generic_simpl_under_binary_operator};
use crate::builtin_trs::rules::simpl_unary::{GenericUnaryOperatorSimplifier, transformation_generic_simpl_under_unary_operator};
use crate::builtin_trs::rules::modulo_associative_flattened_transfo::{ModuloAssociativeFlattenedChecker, transformation_modulo_associative_flattened_transfo};


pub enum BuiltinRewriteTransformationKind<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    // ***
    /// refer to [transformation_flush_to_the_right](transformation_flush_to_the_right)
    AssociativeFlushRight(Box<dyn AssociativityChecker<LanguageOperatorSymbol>>),
    // ***
    /// refer to [transformation_flush_to_the_left](transformation_flush_to_the_left)
    AssociativeFlushLeft(Box<dyn AssociativityChecker<LanguageOperatorSymbol>>),
    // ***
    /// refer to [transformation_reorder_subterms_under_commutative_operator](transformation_reorder_subterms_under_commutative_operator)
    ReorderOperandsIfCommutative(Box<dyn CommutativeCheckerAndOrderer<LanguageOperatorSymbol>>),
    // ***
    /// refer to [transformation_generic_simpl_under_unary_operator](transformation_generic_simpl_under_unary_operator)
    GenericSimplifyUnderUnary(Box<dyn GenericUnaryOperatorSimplifier<LanguageOperatorSymbol>>),
    // ***
    /// refer to [transformation_generic_simpl_under_binary_operator](transformation_generic_simpl_under_binary_operator)
    GenericSimplifyUnderBinary(Box<dyn GenericBinaryOperatorSimplifier<LanguageOperatorSymbol>>),
    // ***
    /// refer to [transformation_factorize_left_distributive](transformation_factorize_left_distributive)
    FactorizeLeftDistributive(Box<dyn DistributivityChecker<LanguageOperatorSymbol>>),
    /// refer to [transformation_factorize_right_distributive](transformation_factorize_right_distributive)
    FactorizeRightDistributive(Box<dyn DistributivityChecker<LanguageOperatorSymbol>>),
    /// refer to [transformation_defactorize_left_distributive](transformation_defactorize_left_distributive)
    DeFactorizeLeftDistributive(Box<dyn DistributivityChecker<LanguageOperatorSymbol>>),
    /// refer to [transformation_defactorize_right_distributive](transformation_defactorize_right_distributive)
    DeFactorizeRightDistributive(Box<dyn DistributivityChecker<LanguageOperatorSymbol>>),
    /// refer to [transformation_modulo_associative_flattened_transfo](transformation_modulo_associative_flattened_transfo)
    ModuloAssociativeFlattenedTransfo(Box<dyn ModuloAssociativeFlattenedChecker<LanguageOperatorSymbol>>)
}


pub struct BuiltinRewriteTransformation<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> {
    pub kind : BuiltinRewriteTransformationKind<LanguageOperatorSymbol>,
    pub desc : String
}

impl<LanguageOperatorSymbol : Clone + PartialEq + Eq + Hash> RewriteRule<LanguageOperatorSymbol> for BuiltinRewriteTransformation<LanguageOperatorSymbol> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn try_apply(&self,
                 term : &LanguageTerm<LanguageOperatorSymbol>
    ) -> Option<LanguageTerm<LanguageOperatorSymbol>> {
        match &self.kind {
            BuiltinRewriteTransformationKind::AssociativeFlushRight(
                rule_application_checker
            ) => {
                transformation_flush_to_the_right::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::AssociativeFlushLeft(
                rule_application_checker
            ) => {
                transformation_flush_to_the_left::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::ReorderOperandsIfCommutative(
                rule_application_checker
            ) => {
                transformation_reorder_subterms_under_commutative_operator::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            }
            BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(
                rule_application_checker
            ) => {
                transformation_generic_simpl_under_unary_operator::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(
                rule_application_checker
            ) => {
                transformation_generic_simpl_under_binary_operator::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeLeftDistributive(
                rule_application_checker
            ) => {
                transformation_factorize_left_distributive::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::DeFactorizeLeftDistributive(
                rule_application_checker
            ) => {
                transformation_defactorize_left_distributive::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::FactorizeRightDistributive(
                rule_application_checker
            ) => {
                transformation_factorize_right_distributive::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::DeFactorizeRightDistributive(
                rule_application_checker
            ) => {
                transformation_defactorize_right_distributive::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
            BuiltinRewriteTransformationKind::ModuloAssociativeFlattenedTransfo(
                rule_application_checker
            ) => {
                transformation_modulo_associative_flattened_transfo::<LanguageOperatorSymbol>(
                    rule_application_checker,
                    term
                )
            },
        }
    }
}



