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

//! Unit tests for [`FlushRightRule`] and [`FlushLeftRule`].
//!
//! Each test calls `try_apply` directly at the root position.
//! No traversal machinery is involved.
//!
//! The test language uses a single associative binary operator `Cat`
//! (modelling string concatenation) with four constant leaves A, B, C, D.

use hashconsing::HConsign;

use simple_term_rewriter::position::PositionInLanguageTerm;
use simple_term_rewriter::rule::RewriteRule;
use simple_term_rewriter::rules::primitives::flush::{
    AssociativityChecker, FlushLeftRule, FlushRightRule,
};
use simple_term_rewriter::term;
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, RewritableLanguageOperatorSymbol, TermFactory,
};

// == test language =============================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum CatOp {
    Cat,
    A,
    B,
    C,
    D,
}

impl RewritableLanguageOperatorSymbol for CatOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            CatOp::Cat => LanguageOperatorArity::Fixed(2),
            _ => LanguageOperatorArity::Fixed(0),
        }
    }
}

struct CatChecker;

impl AssociativityChecker<CatOp> for CatChecker {
    fn is_binary_associative(&self, op: &CatOp) -> bool {
        *op == CatOp::Cat
    }
}

// == helpers ===================================================================

fn root_pos() -> PositionInLanguageTerm {
    PositionInLanguageTerm::get_root_position()
}

fn flush_right() -> FlushRightRule<CatOp> {
    FlushRightRule::new("flush right", CatChecker)
}
fn flush_left() -> FlushLeftRule<CatOp> {
    FlushLeftRule::new("flush left", CatChecker)
}

// == FlushRightRule ============================================================
//
// Pattern: Cat(Cat(x, y), z) → Cat(x, Cat(y, z))

#[test]
fn flush_right_fires_on_left_nested() {
    // Cat(Cat(A, B), C) → Cat(A, Cat(B, C))
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)), term!(&mut f, CatOp::C));
    assert_eq!(
        flush_right().try_apply(&t, &t, &root_pos(), &mut f),
        Some(
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::B), term!(&mut f, CatOp::C)))
        )
    );
}

#[test]
fn flush_right_no_fire_when_left_is_leaf() {
    // Cat(A, Cat(B, C)) — left child is a leaf, pattern does not match
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::B), term!(&mut f, CatOp::C)));
    assert!(flush_right()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn flush_right_no_fire_on_flat_pair() {
    // Cat(A, B) — neither child is Cat
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B));
    assert!(flush_right()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn flush_right_no_fire_on_leaf() {
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::A);
    assert!(flush_right()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn flush_right_three_elements_two_steps() {
    // Cat(Cat(Cat(A,B),C),D) →² Cat(A, Cat(B, Cat(C, D)))
    let pos = root_pos();
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t0 = term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::Cat;
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)),
            term!(&mut f, CatOp::C)),
        term!(&mut f, CatOp::D));

    let t1 = flush_right()
        .try_apply(&t0, &t0, &pos, &mut f)
        .expect("step 1 should fire");
    assert_eq!(
        t1,
        term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)),
        term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::C), term!(&mut f, CatOp::D)))
    );

    let t2 = flush_right()
        .try_apply(&t1, &t1, &pos, &mut f)
        .expect("step 2 should fire");
    assert_eq!(
        t2,
        term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::A),
        term!(&mut f, CatOp::Cat;
            term!(&mut f, CatOp::B),
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::C), term!(&mut f, CatOp::D))))
    );

    assert!(
        flush_right().try_apply(&t2, &t2, &pos, &mut f).is_none(),
        "should be irreducible"
    );
}

// == FlushLeftRule =============================================================
//
// Pattern: Cat(x, Cat(y, z)) → Cat(Cat(x, y), z)

#[test]
fn flush_left_fires_on_right_nested() {
    // Cat(A, Cat(B, C)) → Cat(Cat(A, B), C)
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::B), term!(&mut f, CatOp::C)));
    assert_eq!(
        flush_left().try_apply(&t, &t, &root_pos(), &mut f),
        Some(
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)), term!(&mut f, CatOp::C))
        )
    );
}

#[test]
fn flush_left_no_fire_when_right_is_leaf() {
    // Cat(Cat(A,B), C) — right child is C (a leaf), pattern does not match
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)), term!(&mut f, CatOp::C));
    assert!(flush_left()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn flush_left_no_fire_on_flat_pair() {
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t = term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B));
    assert!(flush_left()
        .try_apply(&t, &t, &root_pos(), &mut f)
        .is_none());
}

#[test]
fn flush_left_three_elements_two_steps() {
    // Cat(A, Cat(B, Cat(C, D))) →² Cat(Cat(Cat(A,B),C),D)
    let pos = root_pos();
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let t0 = term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::A),
        term!(&mut f, CatOp::Cat;
            term!(&mut f, CatOp::B),
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::C), term!(&mut f, CatOp::D))));

    let t1 = flush_left()
        .try_apply(&t0, &t0, &pos, &mut f)
        .expect("step 1 should fire");
    assert_eq!(
        t1,
        term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)),
        term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::C), term!(&mut f, CatOp::D)))
    );

    let t2 = flush_left()
        .try_apply(&t1, &t1, &pos, &mut f)
        .expect("step 2 should fire");
    assert_eq!(
        t2,
        term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::Cat;
            term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)),
            term!(&mut f, CatOp::C)),
        term!(&mut f, CatOp::D))
    );

    assert!(
        flush_left().try_apply(&t2, &t2, &pos, &mut f).is_none(),
        "should be irreducible"
    );
}

// == roundtrip: flush right then left returns to original =====================

#[test]
fn flush_right_then_left_roundtrip() {
    // Cat(Cat(A,B), C) →right Cat(A, Cat(B,C)) →left Cat(Cat(A,B), C)
    let pos = root_pos();
    let mut f: TermFactory<CatOp> = HConsign::empty();
    let original = term!(&mut f, CatOp::Cat;
        term!(&mut f, CatOp::Cat; term!(&mut f, CatOp::A), term!(&mut f, CatOp::B)),
        term!(&mut f, CatOp::C));

    let after_right = flush_right()
        .try_apply(&original, &original, &pos, &mut f)
        .expect("flush right should fire");
    let after_left = flush_left()
        .try_apply(&after_right, &after_right, &pos, &mut f)
        .expect("flush left should fire");

    assert_eq!(after_left, original);
}

// == get_desc =================================================================

#[test]
fn flush_right_rule_get_desc() {
    assert_eq!(flush_right().get_desc(), "flush right");
}

#[test]
fn flush_left_rule_get_desc() {
    assert_eq!(flush_left().get_desc(), "flush left");
}
