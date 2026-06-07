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

//! Integration tests for [`RewriteProcessUntracedExecutor`] on regex terms.
//!
//! These tests verify full normalisation strategies on hand-crafted terms,
//! exercising combinations of `Repeat`, `TryOnePath`, `AnyChild`, and `Rule`.
//! Each test checks that a strategy applied to a concrete term yields the
//! expected normal form.
//!
//! The shared normalisation strategy is outermost: try a rule at the root
//! first, then descend left-to-right until a rule fires, and repeat until
//! fixpoint.

mod common;

use hashconsing::HConsign;

use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use common::regex::rules::*;

/// Apply `normalization_strategy()` to `term` and return the single result.
///
/// Uses the caller's factory so that the result is in the same hash-consing
/// universe and can be compared directly with other terms built from `f`.
fn normalize(term: LanguageTerm<RegexOp>, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    let mut results = rewrite(normalization_strategy(), term, f);
    assert_eq!(results.len(), 1, "expected exactly one normal form");
    results.remove(0)
}

// == identity / already normal =================================================

/// An `Atom` term is already in normal form; no rule fires.
#[test]
fn atom_is_already_normal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(normalize(atom(b'a', &mut f), &mut f), atom(b'a', &mut f));
}

/// `Epsilon` is already in normal form; no rule fires.
#[test]
fn epsilon_is_already_normal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(normalize(epsilon(&mut f), &mut f), epsilon(&mut f));
}

// == single-rule reductions ====================================================

/// `Star(∅) → ε`
#[test]
fn normalize_star_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(star(empty(&mut f), &mut f), &mut f),
        epsilon(&mut f)
    );
}

/// `Star(ε) → ε`
#[test]
fn normalize_star_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(star(epsilon(&mut f), &mut f), &mut f),
        epsilon(&mut f)
    );
}

/// `Star(Star(a)) → Star(a)`
#[test]
fn normalize_double_star() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f),
        star(atom(b'a', &mut f), &mut f)
    );
}

/// `Concat(∅, a) → ∅`  (absorbing element on the left)
#[test]
fn normalize_concat_left_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(concat(empty(&mut f), atom(b'a', &mut f), &mut f), &mut f),
        empty(&mut f)
    );
}

/// `Concat(a, ε) → a`  (right-identity of concatenation)
#[test]
fn normalize_concat_right_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(concat(atom(b'a', &mut f), epsilon(&mut f), &mut f), &mut f),
        atom(b'a', &mut f)
    );
}

// == multi-step reductions =====================================================

/// Three nested stars collapse to one: `Star(Star(Star(a))) →* Star(a)`
#[test]
fn normalize_nested_double_star() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(
            star(star(star(atom(b'a', &mut f), &mut f), &mut f), &mut f),
            &mut f
        ),
        star(atom(b'a', &mut f), &mut f)
    );
}

/// `Concat(Star(∅), a)` first reduces `Star(∅) → ε`, then `Concat(ε, a) → a`.
#[test]
fn normalize_concat_with_star_empty_child() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        normalize(
            concat(star(empty(&mut f), &mut f), atom(b'a', &mut f), &mut f),
            &mut f
        ),
        atom(b'a', &mut f)
    );
}

/// `Alt(Concat(ε, Star(Star(a))), Concat(b, ∅))`:
///
/// 1. `Concat(ε, Star(Star(a))) →* Star(a)`  (left identity + double-star)
/// 2. `Concat(b, ∅) → ∅`                     (right absorbing)
/// 3. `Alt(Star(a), ∅) → Star(a)`            (right identity of alt)
#[test]
fn normalize_deep_term() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(
        concat(
            epsilon(&mut f),
            star(star(atom(b'a', &mut f), &mut f), &mut f),
            &mut f,
        ),
        concat(atom(b'b', &mut f), empty(&mut f), &mut f),
        &mut f,
    );
    assert_eq!(normalize(t, &mut f), star(atom(b'a', &mut f), &mut f));
}

/// `Alt(Star(a), Star(a)) → Star(a)` via idempotence of alternation.
#[test]
fn normalize_alt_idempotent() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = alt(
        star(atom(b'a', &mut f), &mut f),
        star(atom(b'a', &mut f), &mut f),
        &mut f,
    );
    assert_eq!(normalize(t, &mut f), star(atom(b'a', &mut f), &mut f));
}
