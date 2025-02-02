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



use crate::builtin_trs::builtin_transfo_kind::BuiltinRewriteTransformations;
use crate::core::interface::BarebonesTermRewritingInterface;
use crate::core::term::LanguageTerm;


/**
The builtin interface, extending the barebones interface
 **/
pub trait BuiltinTermRewritingInterface : BarebonesTermRewritingInterface {

    /**
     Returns the arity of the operators.
    Conditions the application of some of the builtin rules.
     **/
    fn op_arity(op : &Self::LanguageOperatorSymbol)->usize;

    /**
     Whether or not binary operators are associative.
    Conditions the application of some of the builtin rules.
     **/
    fn op_is_binary_associative(op : &Self::LanguageOperatorSymbol)->bool;

    /**
     Whether or not binary operators are commutative.
    Conditions the application of some of the builtin rules.
     **/
    fn op_is_binary_commutative(op : &Self::LanguageOperatorSymbol)->bool;

    /**
     Whether or not binary operators are idempotent.
    Conditions the application of some of the builtin rules.
     **/
    fn op_is_binary_idempotent(op : &Self::LanguageOperatorSymbol)->bool;

    /**
     Whether op1(op2(x,y),op2(x,z)) is equivalent to op2(x,op1(y,z))
     **/
    fn op_distributes_over(op1 : &Self::LanguageOperatorSymbol, op2 : &Self::LanguageOperatorSymbol)->bool;

    /**
     Returns true if the provided sub-term is a neutral element / fixpoint for the given operator.
    Example:
    - for a binary operator such as the addition +, 0 is neutral so is_neutral_element_or_fixpoint_for(0,+) should return true
    - for a unary operator such as the square function (f:x->x^2), 1 is neutral so is_neutral_element_or_fixpoint_for(1,f) should return true
     **/
    fn is_neutral_element_or_fixpoint_for(
        sub_term : &LanguageTerm<Self::LanguageOperatorSymbol>,
        parent_operator : &Self::LanguageOperatorSymbol
    ) -> bool;

    /**
     To be able to include all the builtin rules' identifiers in the type of all possible rewrite rules
    that the library user defines.
     **/
    fn from_builtin_to_generic_transformation_kind(
        builtin_transfo : &BuiltinRewriteTransformations
    ) -> Self::TransformationKind;

    /**
     This is used to define a total order on terms so that we may use Ordered Rewriting to deal with commutative operators.
    See https://link.springer.com/content/pdf/10.1007/3-540-52885-7_100?pdf=chapter+toc
    The implementer should make sure that the order is total.
    I.e., that "compare_operators" only returns std::cmp::Ordering::Equal for equal values.
     **/
    fn compare_operators(
        op1 : &Self::LanguageOperatorSymbol,
        op2 : &Self::LanguageOperatorSymbol
    ) -> std::cmp::Ordering;

    /**
     For any two unary operators F and G,
    if there exists a unary operator H such that for any x, we have F(G(x)) equivalent to H(x)
    then this function returns Some(H) otherwise None.
     **/
    fn compose_nested_unary_operators(
        operator_on_top : &Self::LanguageOperatorSymbol,
        operator_underneath : &Self::LanguageOperatorSymbol
    ) -> Option<Self::LanguageOperatorSymbol>;

}



