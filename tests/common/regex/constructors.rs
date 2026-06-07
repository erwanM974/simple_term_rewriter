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

use crate::common::regex::lang::RegexOp;

pub fn empty(f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Empty, vec![], f)
}

pub fn epsilon(f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Epsilon, vec![], f)
}

pub fn atom(c: u8, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Atom(c), vec![], f)
}

pub fn alt(
    l: LanguageTerm<RegexOp>,
    r: LanguageTerm<RegexOp>,
    f: &mut TermFactory<RegexOp>,
) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Alt, vec![l, r], f)
}

pub fn concat(
    l: LanguageTerm<RegexOp>,
    r: LanguageTerm<RegexOp>,
    f: &mut TermFactory<RegexOp>,
) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Concat, vec![l, r], f)
}

pub fn star(r: LanguageTerm<RegexOp>, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Star, vec![r], f)
}
