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

use simple_term_rewriter::term::syntax::{LanguageOperatorArity, RewritableLanguageOperatorSymbol};

/// A small regular-expression language used as a shared test domain.
///
/// Operators:
/// - `Empty`    — the empty language ∅ (arity 0)
/// - `Epsilon`  — the empty string ε (arity 0)
/// - `Atom(u8)` — a single character (arity 0)
/// - `Alt`      — alternation `r | s` (arity 2)
/// - `Concat`   — concatenation `r · s` (arity 2)
/// - `Star`     — Kleene star `r*` (arity 1)
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RegexOp {
    Empty,
    Epsilon,
    Atom(u8),
    Alt,
    Concat,
    Star,
}

impl RewritableLanguageOperatorSymbol for RegexOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            RegexOp::Empty | RegexOp::Epsilon | RegexOp::Atom(_) => LanguageOperatorArity::Fixed(0),
            RegexOp::Alt | RegexOp::Concat => LanguageOperatorArity::Fixed(2),
            RegexOp::Star => LanguageOperatorArity::Fixed(1),
        }
    }
}

impl fmt::Display for RegexOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegexOp::Empty => write!(f, "∅"),
            RegexOp::Epsilon => write!(f, "ε"),
            RegexOp::Atom(b) => write!(f, "{}", *b as char),
            RegexOp::Alt => write!(f, "+"),
            RegexOp::Concat => write!(f, "·"),
            RegexOp::Star => write!(f, "*"),
        }
    }
}
