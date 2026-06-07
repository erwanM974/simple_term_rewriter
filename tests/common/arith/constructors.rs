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

use simple_term_rewriter::term::syntax::{LanguageTerm, LanguageTermNode, TermFactory};

use super::lang::ArithOp;

pub fn zero(f: &mut TermFactory<ArithOp>) -> LanguageTerm<ArithOp> {
    LanguageTermNode::build(ArithOp::Zero, vec![], f)
}

pub fn var(c: char, f: &mut TermFactory<ArithOp>) -> LanguageTerm<ArithOp> {
    LanguageTermNode::build(ArithOp::Var(c), vec![], f)
}

pub fn add(
    l: LanguageTerm<ArithOp>,
    r: LanguageTerm<ArithOp>,
    f: &mut TermFactory<ArithOp>,
) -> LanguageTerm<ArithOp> {
    LanguageTermNode::build(ArithOp::Add, vec![l, r], f)
}

pub fn mul(
    l: LanguageTerm<ArithOp>,
    r: LanguageTerm<ArithOp>,
    f: &mut TermFactory<ArithOp>,
) -> LanguageTerm<ArithOp> {
    LanguageTermNode::build(ArithOp::Mul, vec![l, r], f)
}
