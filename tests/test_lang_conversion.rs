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

//! Tests for `FromDomainSpecificTermToRewritableTerm` and
//! `FromRewritableTermToDomainSpecificTerm`.
//!
//! A locally-defined `DomainRegex` type implements both conversion traits and
//! is structurally equivalent to `LanguageTerm<RegexOp>`, allowing round-trip
//! tests in both directions.
//!
//! Hand-crafted tests verify specific structural cases (leaves, unary, binary,
//! nested).  Randomized round-trip tests at the bottom verify both conversion
//! directions over 200 randomly generated terms.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::term::conversion::{
    from_rewritable_term::FromRewritableTermToDomainSpecificTerm,
    to_rewritable_term::FromDomainSpecificTermToRewritableTerm,
};
use simple_term_rewriter::term::syntax::TermFactory;

use common::regex::constructors::{alt, atom, concat, empty, epsilon, star};
use common::regex::generation::generate_regex_terms;
use common::regex::lang::RegexOp;

// == domain type ===============================================================

#[derive(Clone, PartialEq, Debug)]
enum DomainRegex {
    Empty,
    Epsilon,
    Atom(u8),
    Alt(Box<DomainRegex>, Box<DomainRegex>),
    Concat(Box<DomainRegex>, Box<DomainRegex>),
    Star(Box<DomainRegex>),
}

impl DomainRegex {
    fn d_empty() -> Self {
        DomainRegex::Empty
    }
    fn d_epsilon() -> Self {
        DomainRegex::Epsilon
    }
    fn d_atom(b: u8) -> Self {
        DomainRegex::Atom(b)
    }
    fn d_alt(l: Self, r: Self) -> Self {
        DomainRegex::Alt(Box::new(l), Box::new(r))
    }
    fn d_concat(l: Self, r: Self) -> Self {
        DomainRegex::Concat(Box::new(l), Box::new(r))
    }
    fn d_star(inner: Self) -> Self {
        DomainRegex::Star(Box::new(inner))
    }
}

// == trait impls ===============================================================

impl FromDomainSpecificTermToRewritableTerm<RegexOp> for DomainRegex {
    fn get_operator_at_root(&self) -> RegexOp {
        match self {
            DomainRegex::Empty => RegexOp::Empty,
            DomainRegex::Epsilon => RegexOp::Epsilon,
            DomainRegex::Atom(b) => RegexOp::Atom(*b),
            DomainRegex::Alt(..) => RegexOp::Alt,
            DomainRegex::Concat(..) => RegexOp::Concat,
            DomainRegex::Star(..) => RegexOp::Star,
        }
    }

    fn get_subterms(&self) -> Vec<&Self> {
        match self {
            DomainRegex::Empty | DomainRegex::Epsilon | DomainRegex::Atom(_) => vec![],
            DomainRegex::Alt(l, r) | DomainRegex::Concat(l, r) => vec![l.as_ref(), r.as_ref()],
            DomainRegex::Star(inner) => vec![inner.as_ref()],
        }
    }
}

impl FromRewritableTermToDomainSpecificTerm<RegexOp> for DomainRegex {
    fn instantiate_term_under_operator(operator: &RegexOp, sub_terms: &mut Vec<Self>) -> Self {
        match operator {
            RegexOp::Empty => DomainRegex::Empty,
            RegexOp::Epsilon => DomainRegex::Epsilon,
            RegexOp::Atom(b) => DomainRegex::Atom(*b),
            RegexOp::Alt => {
                let l = sub_terms.remove(0);
                let r = sub_terms.remove(0);
                DomainRegex::Alt(Box::new(l), Box::new(r))
            }
            RegexOp::Concat => {
                let l = sub_terms.remove(0);
                let r = sub_terms.remove(0);
                DomainRegex::Concat(Box::new(l), Box::new(r))
            }
            RegexOp::Star => {
                let inner = sub_terms.remove(0);
                DomainRegex::Star(Box::new(inner))
            }
        }
    }
}

// == to_rewritable_term ========================================================

#[test]
fn to_rewritable_term_leaf_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        DomainRegex::d_empty().to_rewritable_term(&mut f),
        empty(&mut f)
    );
}

#[test]
fn to_rewritable_term_leaf_atom() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        DomainRegex::d_atom(b'a').to_rewritable_term(&mut f),
        atom(b'a', &mut f)
    );
}

#[test]
fn to_rewritable_term_unary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let d = DomainRegex::d_star(DomainRegex::d_atom(b'a'));
    assert_eq!(
        d.to_rewritable_term(&mut f),
        star(atom(b'a', &mut f), &mut f)
    );
}

#[test]
fn to_rewritable_term_binary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let d = DomainRegex::d_alt(DomainRegex::d_atom(b'a'), DomainRegex::d_epsilon());
    assert_eq!(
        d.to_rewritable_term(&mut f),
        alt(atom(b'a', &mut f), epsilon(&mut f), &mut f)
    );
}

#[test]
fn to_rewritable_term_nested() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let d = DomainRegex::d_concat(
        DomainRegex::d_star(DomainRegex::d_atom(b'a')),
        DomainRegex::d_alt(DomainRegex::d_epsilon(), DomainRegex::d_empty()),
    );
    assert_eq!(
        d.to_rewritable_term(&mut f),
        concat(
            star(atom(b'a', &mut f), &mut f),
            alt(epsilon(&mut f), empty(&mut f), &mut f),
            &mut f
        )
    );
}

// == from_rewritable_term ======================================================

#[test]
fn from_rewritable_term_leaf_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        DomainRegex::from_rewritable_term(&epsilon(&mut f)),
        DomainRegex::d_epsilon()
    );
}

#[test]
fn from_rewritable_term_leaf_atom() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        DomainRegex::from_rewritable_term(&atom(b'b', &mut f)),
        DomainRegex::d_atom(b'b')
    );
}

#[test]
fn from_rewritable_term_unary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let expected = DomainRegex::d_star(DomainRegex::d_atom(b'a'));
    assert_eq!(
        DomainRegex::from_rewritable_term(&star(atom(b'a', &mut f), &mut f)),
        expected
    );
}

#[test]
fn from_rewritable_term_binary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let expected = DomainRegex::d_concat(DomainRegex::d_epsilon(), DomainRegex::d_empty());
    assert_eq!(
        DomainRegex::from_rewritable_term(&concat(epsilon(&mut f), empty(&mut f), &mut f)),
        expected
    );
}

// == round-trips ===============================================================

#[test]
fn round_trip_domain_to_rewritable_to_domain() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let original = DomainRegex::d_alt(
        DomainRegex::d_star(DomainRegex::d_atom(b'a')),
        DomainRegex::d_concat(DomainRegex::d_epsilon(), DomainRegex::d_atom(b'b')),
    );
    let rewritable = original.to_rewritable_term(&mut f);
    let recovered = DomainRegex::from_rewritable_term(&rewritable);
    assert_eq!(recovered, original);
}

#[test]
fn round_trip_rewritable_to_domain_to_rewritable() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let original = concat(
        star(atom(b'a', &mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        &mut f,
    );
    let domain = DomainRegex::from_rewritable_term(&original);
    let recovered = domain.to_rewritable_term(&mut f);
    assert_eq!(recovered, original);
}

// == randomized round-trips ====================================================

/// For 200 random terms: `LanguageTerm → DomainRegex → LanguageTerm` must be
/// the identity.
///
/// Exercises the full variety of tree shapes produced by the generator (all six
/// operators, varying depths up to 5) and ensures `from_rewritable_term` /
/// `to_rewritable_term` are mutual inverses.
#[test]
fn random_round_trip_rewritable_to_domain_to_rewritable() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    for original in generate_regex_terms(200, 0, &mut f) {
        let domain = DomainRegex::from_rewritable_term(&original);
        let recovered = domain.to_rewritable_term(&mut f);
        assert_eq!(recovered, original);
    }
}

/// For 200 random terms: `LanguageTerm → DomainRegex → LanguageTerm → DomainRegex`
/// must equal the first `DomainRegex`.
///
/// Exercises the domain direction of the round-trip independently: whatever
/// `from_rewritable_term` produces, converting it back to a `LanguageTerm` and
/// then to `DomainRegex` again must yield the same domain value.
#[test]
fn random_round_trip_domain_to_rewritable_to_domain() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    for term in generate_regex_terms(200, 1, &mut f) {
        let domain_first = DomainRegex::from_rewritable_term(&term);
        let rewritable = domain_first.to_rewritable_term(&mut f);
        let domain_second = DomainRegex::from_rewritable_term(&rewritable);
        assert_eq!(domain_second, domain_first);
    }
}
