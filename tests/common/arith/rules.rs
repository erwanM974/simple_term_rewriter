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

use simple_term_rewriter::rules::primitives::factorization::distributivity_checker::DistributivityChecker;

use super::lang::ArithOp;

/// Models a semiring:
/// - Add is associative and commutative;
/// - Mul distributes over Add from both sides.
/// - Mul is not treated as associative here so that the modulo-AC flattening only applies to the outer Add layer (keeping tests predictable).
pub struct ArithChecker;

impl DistributivityChecker<ArithOp> for ArithChecker {
    fn is_binary(&self, op: &ArithOp) -> bool {
        matches!(op, ArithOp::Add | ArithOp::Mul)
    }

    fn is_associative(&self, op: &ArithOp) -> bool {
        *op == ArithOp::Add
    }

    fn is_commutative(&self, op: &ArithOp) -> bool {
        *op == ArithOp::Add
    }

    fn is_left_distributive_over(&self, op1: &ArithOp, op2: &ArithOp) -> bool {
        *op1 == ArithOp::Mul && *op2 == ArithOp::Add
    }

    fn is_right_distributive_over(&self, op1: &ArithOp, op2: &ArithOp) -> bool {
        *op1 == ArithOp::Mul && *op2 == ArithOp::Add
    }

    fn get_empty_operation_symbol(&self) -> ArithOp {
        ArithOp::Zero
    }
}
