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

use simple_term_rewriter::term::syntax::{LanguageOperatorArity, RewritableLanguageOperatorSymbol};

/// A minimal arithmetic language used to test the factorization rules.
///
/// Operators:
/// - `Zero`      : the additive identity (arity 0); returned by `get_empty_operation_symbol`
/// - `Var(char)` : a named variable (arity 0)
/// - `Add`       : addition: associative, commutative (arity 2)
/// - `Mul`       : multiplication: left- and right-distributive over `Add` (arity 2)
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ArithOp {
    Zero,
    Var(char),
    Add,
    Mul,
}

impl RewritableLanguageOperatorSymbol for ArithOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            ArithOp::Zero | ArithOp::Var(_) => LanguageOperatorArity::Fixed(0),
            ArithOp::Add | ArithOp::Mul => LanguageOperatorArity::Fixed(2),
        }
    }
}
