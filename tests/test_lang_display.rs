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

//! Tests for `LanguageTerm<LOS>` display formatting.
//!
//! `LanguageTerm` implements `Display` when `LOS: Display`.  After adding
//! `Display` to `RegexOp`, leaves render as the operator symbol and compound
//! terms render as `OP(sub1, sub2, ...)`.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::{alt, atom, concat, empty, epsilon, star};
use common::regex::lang::RegexOp;

// == leaves ====================================================================

#[test]
fn display_leaf_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(format!("{}", empty(&mut f)), "∅");
}

#[test]
fn display_leaf_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(format!("{}", epsilon(&mut f)), "ε");
}

#[test]
fn display_leaf_atom() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(format!("{}", atom(b'a', &mut f)), "a");
    assert_eq!(format!("{}", atom(b'z', &mut f)), "z");
}

// == single-child compound =====================================================

#[test]
fn display_unary_star() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(format!("{}", star(atom(b'a', &mut f), &mut f)), "*(a)");
}

#[test]
fn display_unary_star_of_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(format!("{}", star(epsilon(&mut f), &mut f)), "*(ε)");
}

// == two-child compounds =======================================================

#[test]
fn display_binary_alt() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        format!("{}", alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f)),
        "+(a, b)"
    );
}

#[test]
fn display_binary_concat() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        format!("{}", concat(epsilon(&mut f), empty(&mut f), &mut f)),
        "·(ε, ∅)"
    );
}

// == nested ====================================================================

#[test]
fn display_nested_alt_of_star_and_concat() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        format!(
            "{}",
            alt(
                star(atom(b'a', &mut f), &mut f),
                concat(atom(b'b', &mut f), epsilon(&mut f), &mut f),
                &mut f
            )
        ),
        "+(*(a), ·(b, ε))"
    );
}

#[test]
fn display_deeply_nested_stars() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        format!(
            "{}",
            star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f)
        ),
        "*(*(*(a)))"
    );
}
