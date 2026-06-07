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

use crate::term::syntax::RewritableLanguageOperatorSymbol;

/// Describes the algebraic properties of operator symbols needed by the
/// factorization and defactorization rules.
pub trait DistributivityChecker<LOS: RewritableLanguageOperatorSymbol> {
    /// Returns `true` if `op` is a binary operator that this checker handles.
    fn is_binary(&self, op: &LOS) -> bool;

    /// Returns `true` if `op` is associative.
    fn is_associative(&self, op: &LOS) -> bool;

    /// Returns `true` if `op` is commutative.
    fn is_commutative(&self, op: &LOS) -> bool;

    /// Returns `true` if `op1` is left-distributive over `op2`:
    /// `op1(x, op2(y, z)) = op2(op1(x, y), op1(x, z))` for all `x`, `y`, `z`.
    ///
    /// Example: multiplication is left-distributive over addition —
    /// `*(2, +(1, 3)) = +(*(2, 1), *(2, 3))`.
    fn is_left_distributive_over(&self, op1: &LOS, op2: &LOS) -> bool;

    /// Returns `true` if `op1` is right-distributive over `op2`:
    /// `op1(op2(y, z), x) = op2(op1(y, x), op1(z, x))` for all `x`, `y`, `z`.
    ///
    /// Example: multiplication is right-distributive over addition —
    /// `*(+(1, 3), 2) = +(*(1, 2), *(3, 2))`.
    fn is_right_distributive_over(&self, op1: &LOS, op2: &LOS) -> bool;

    /// Returns the "zero" / identity operator symbol used as a placeholder
    /// in partial factorizations such as `op1(op2(x, y), x) → op2(x, op1(y, 0))`.
    fn get_empty_operation_symbol(&self) -> LOS;
}
