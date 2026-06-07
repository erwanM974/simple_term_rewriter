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

//! Test suite for `is_greater_as_per_lexicographic_path_ordering`.
//!
//! Operator alphabet (greatest first): Concat(2) > Star(1) > Epsilon(0) > Empty(0)
//!
//! LPO definition (Baader & Nipkow §5.4): s = f(s₁,…,sₙ) >_lpo t iff:
//!
//! (Case f > g)  ∀j: s >_lpo tⱼ
//! (Case f < g)  ∃i: sᵢ ≥_lpo t   (i.e. sᵢ = t or sᵢ >_lpo t)
//! (Case f = g)  ∃k such that s₁=t₁,…,s_{k-1}=t_{k-1}, sₖ >_lpo tₖ,
//!               AND ∀j > k: s >_lpo tⱼ
//!
//! Tests are grouped by which LPO case they exercise.

mod common;

use std::cmp::Ordering;

use hashconsing::HConsign;

use simple_term_rewriter::rules::util::lpo::{
    is_greater_as_per_lexicographic_path_ordering, lexicographic_path_ordering,
};
use simple_term_rewriter::term;
use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

use common::regex::lang::RegexOp;
use common::regex::lang::RegexOp::{Concat, Empty, Epsilon, Star};

// == LPO helpers ===============================================================

fn lpo_cmp(x: &RegexOp, y: &RegexOp) -> Ordering {
    fn rank(op: &RegexOp) -> u16 {
        match op {
            RegexOp::Atom(x) => 5 + (*x as u16),
            RegexOp::Alt => 4,
            RegexOp::Concat => 3,
            RegexOp::Star => 2,
            RegexOp::Epsilon => 1,
            RegexOp::Empty => 0,
        }
    }
    rank(x).cmp(&rank(y))
}

fn lpo_gt(s: &LanguageTerm<RegexOp>, t: &LanguageTerm<RegexOp>) -> bool {
    is_greater_as_per_lexicographic_path_ordering(s, t, &lpo_cmp)
}

/// `s >_lpo t` should hold.
fn assert_gt(label: &str, s: &LanguageTerm<RegexOp>, t: &LanguageTerm<RegexOp>) {
    assert!(lpo_gt(s, t), "{label}: expected s >_lpo t");
}

/// `s >_lpo t` should NOT hold.
fn assert_not_gt(label: &str, s: &LanguageTerm<RegexOp>, t: &LanguageTerm<RegexOp>) {
    assert!(!lpo_gt(s, t), "{label}: expected ¬(s >_lpo t)");
}

// == 1. Irreflexivity =========================================================

#[test]
fn lpo_irreflexive_constant() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Empty ≯ Empty",
        &term!(&mut f, Empty),
        &term!(&mut f, Empty),
    );
    assert_not_gt(
        "Epsilon ≯ Epsilon",
        &term!(&mut f, Epsilon),
        &term!(&mut f, Epsilon),
    );
}

#[test]
fn lpo_irreflexive_unary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Star(Empty) ≯ Star(Empty)",
        &term!(&mut f, Star; term!(&mut f, Empty)),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_irreflexive_binary() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Empty,Epsilon) ≯ Concat(Empty,Epsilon)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
    );
}

// == 2. Constant ordering (leaf operators, Case f > g / f < g with arity 0) ==

#[test]
fn lpo_constant_ordering_epsilon_gt_empty() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Epsilon > Empty",
        &term!(&mut f, Epsilon),
        &term!(&mut f, Empty),
    );
}

#[test]
fn lpo_constant_ordering_empty_not_gt_epsilon() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Empty ≯ Epsilon",
        &term!(&mut f, Empty),
        &term!(&mut f, Epsilon),
    );
}

#[test]
fn lpo_constant_ordering_epsilon_not_gt_star_empty() {
    // Epsilon is a nullary constant; Star(Empty) has a strictly greater root operator.
    // A bare constant cannot dominate a compound term with a higher root.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Epsilon ≯ Star(Empty)",
        &term!(&mut f, Epsilon),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
}

// == 3. Case f > g: root of s is strictly greater than root of t ==============
//
// Rule: s >_lpo t iff ∀j: s >_lpo tⱼ   (s dominates every child of t)

#[test]
fn lpo_case_f_gt_g_unary_over_constant() {
    // Star(Empty) > Empty: Star > Empty, arity(Empty)=0 → vacuously true.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Star(Empty) > Empty",
        &term!(&mut f, Star; term!(&mut f, Empty)),
        &term!(&mut f, Empty),
    );
    // Star(Empty) > Epsilon: same reasoning.
    assert_gt(
        "Star(Empty) > Epsilon",
        &term!(&mut f, Star; term!(&mut f, Empty)),
        &term!(&mut f, Epsilon),
    );
    // Star(Epsilon) > Empty: same.
    assert_gt(
        "Star(Epsilon) > Empty",
        &term!(&mut f, Star; term!(&mut f, Epsilon)),
        &term!(&mut f, Empty),
    );
}

#[test]
fn lpo_case_f_gt_g_binary_over_constant() {
    // Concat(Empty,Empty) > Empty: Concat > Empty, arity(Empty)=0, vacuously true.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Empty,Empty) > Empty",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Empty),
    );
    assert_gt(
        "Concat(Empty,Empty) > Epsilon",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Epsilon),
    );
    assert_gt(
        "Concat(Empty,Epsilon) > Empty",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Empty),
    );
    assert_gt(
        "Concat(Empty,Epsilon) > Epsilon",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Epsilon),
    );
}

#[test]
fn lpo_case_f_gt_g_binary_over_unary() {
    // Concat(Empty,Empty) > Star(Empty): Concat > Star; must check Concat(Empty,Empty) > Empty (only child of Star).
    // Concat > Empty, arity(Empty)=0: true.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Empty,Empty) > Star(Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
    assert_gt(
        "Concat(Empty,Epsilon) > Star(Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
    assert_gt(
        "Concat(Empty,Epsilon) > Star(Epsilon)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Star; term!(&mut f, Epsilon)),
    );
}

#[test]
fn lpo_case_f_gt_g_fails_when_child_not_dominated() {
    // Concat(Empty,Empty) > Star(Epsilon): Concat > Star; must check Concat(Empty,Empty) > Epsilon.
    // Concat > Epsilon, arity(Epsilon)=0: true.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Empty,Empty) > Star(Epsilon)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Star; term!(&mut f, Epsilon)),
    );
    // However if t has a subterm that s cannot dominate, the Case f>g fails.
    // Concat(Empty,Empty) ≯ Star(Concat(Epsilon,Epsilon)):
    //   Concat > Star, must check Concat(Empty,Empty) > Concat(Epsilon,Epsilon).
    //   Case f=g: k=0: Empty vs Epsilon. Empty ≯ Epsilon; Epsilon > Empty → return false.
    //   So Concat(Empty,Empty) ≯ Concat(Epsilon,Epsilon) → Concat(Empty,Empty) ≯ Star(Concat(Epsilon,Epsilon)).
    assert_not_gt(
        "Concat(Empty,Empty) ≯ Star(Concat(Epsilon,Epsilon))",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Star; term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Epsilon))),
    );
}

// == 4. Case f < g: root of s is strictly less than root of t =================
//
// Rule: s >_lpo t iff ∃i: sᵢ = t or sᵢ >_lpo t

#[test]
fn lpo_case_f_lt_g_subterm_equals_t() {
    // Star(Concat(Empty,Epsilon)) > Concat(Empty,Epsilon):
    // Star < Concat, but s₀ = Concat(Empty,Epsilon) = t. Subterm rule fires.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Star(Concat(Empty,Epsilon)) > Concat(Empty,Epsilon)",
        &term!(&mut f, Star; term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon))),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
    );
}

#[test]
fn lpo_case_f_lt_g_subterm_dominates_t() {
    // Star(Concat(Epsilon,Epsilon)) > Concat(Empty,Empty):
    // Star < Concat, but s₀ = Concat(Epsilon,Epsilon) >_lpo Concat(Empty,Empty).
    //   Concat=Concat, k=0: Epsilon > Empty. ∀j>0: Concat(Epsilon,Epsilon) > Empty: Concat>Empty, true. ✓
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Star(Concat(Epsilon,Epsilon)) > Concat(Empty,Empty)",
        &term!(&mut f, Star; term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Epsilon))),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_case_f_lt_g_no_subterm_dominates() {
    // Star(Empty) ≯ Concat(Empty,Empty):
    // Star < Concat, s₀ = Empty. Empty = Concat(Empty,Empty)? No. Empty >_lpo Concat(Empty,Empty)? Empty < Concat → no subterm. False.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Star(Empty) ≯ Concat(Empty,Empty)",
        &term!(&mut f, Star; term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
    );
    // Epsilon ≯ Star(Empty): Epsilon < Star, Epsilon has no subterms. False.
    assert_not_gt(
        "Epsilon ≯ Star(Empty)",
        &term!(&mut f, Epsilon),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
    // Empty ≯ Star(Empty): Empty < Star, Empty has no subterms. False.
    assert_not_gt(
        "Empty ≯ Star(Empty)",
        &term!(&mut f, Empty),
        &term!(&mut f, Star; term!(&mut f, Empty)),
    );
}

// == 5. Case f = g: equal root operators, correct behaviour ===================
//
// Rule: ∃k such that s₁=t₁,…,s_{k-1}=t_{k-1}, sₖ >_lpo tₖ, AND ∀j>k: s >_lpo tⱼ
//
// The tests below exercise cases where both the current implementation and the
// correct LPO agree (no bug triggered).

#[test]
fn lpo_equal_root_first_arg_decides() {
    // Concat(Epsilon,Empty) > Concat(Empty,Empty): k=0 (Epsilon>Empty). ∀j>0: Concat(Epsilon,Empty) > Empty → Concat>Empty, true. ✓
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Epsilon,Empty) > Concat(Empty,Empty)",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Empty)),
    );
    // Concat(Epsilon,Empty) > Concat(Empty,Epsilon): k=0 (Epsilon>Empty). ∀j>0: Concat(Epsilon,Empty) > Epsilon → Concat>Epsilon, true. ✓
    assert_gt(
        "Concat(Epsilon,Empty) > Concat(Empty,Epsilon)",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Epsilon)),
    );
}

#[test]
fn lpo_equal_root_second_arg_decides() {
    // Concat(Empty,Epsilon) > Concat(Empty,Empty): k=1 (Epsilon>Empty). No j>1. ✓
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Empty,Epsilon) > Concat(Empty,Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_equal_root_first_arg_smaller_loses() {
    // Concat(Empty,Epsilon) ≯ Concat(Epsilon,Empty): k=0: Empty<Epsilon → false; Epsilon>Empty → return false.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Empty,Epsilon) ≯ Concat(Epsilon,Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Epsilon)),
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
    );
    // Concat(Empty,Empty) ≯ Concat(Epsilon,Empty): k=0: Empty<Epsilon → false.
    assert_not_gt(
        "Concat(Empty,Empty) ≯ Concat(Epsilon,Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_equal_root_all_equal_args() {
    // Concat(Empty,Empty) ≯ Concat(Empty,Empty): irreflexivity via the equal-root path.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Empty,Empty) ≯ Concat(Empty,Empty)",
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_equal_root_nested_correct() {
    // Concat(Concat(Empty,Epsilon), Empty) > Concat(Concat(Empty,Empty), Empty):
    //   k=0: Concat(Empty,Epsilon) vs Concat(Empty,Empty). Concat(Empty,Epsilon) > Concat(Empty,Empty): k=1 (Epsilon>Empty), no j>1. TRUE.
    //   ∀j>0: Concat(Concat(Empty,Epsilon),Empty) > Empty → Concat>Empty, true. ✓
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Concat(Empty,Epsilon),Empty) > Concat(Concat(Empty,Empty),Empty)",
        &term!(&mut f, Concat; term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Epsilon)), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty)),   term!(&mut f, Empty)),
    );
}

#[test]
fn lpo_equal_root_remaining_dominated_by_f_gt_case() {
    // Concat(Epsilon,Empty) > Concat(Empty, Star(Empty)):
    //   k=0: Epsilon > Empty.
    //   ∀j>0: Concat(Epsilon,Empty) > Star(Empty) → Concat>Star, Concat(Epsilon,Empty)>Empty (Concat>Empty, arity 0). TRUE. ✓
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_gt(
        "Concat(Epsilon,Empty) > Concat(Empty,Star(Empty))",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Star; term!(&mut f, Empty))),
    );

    // Concat(Epsilon,Empty) > Concat(Empty, Concat(Empty,Empty)):
    //   k=0: Epsilon>Empty.
    //   ∀j>0: Concat(Epsilon,Empty) > Concat(Empty,Empty):
    //     k=0: Epsilon>Empty. ∀j>0: Concat(Epsilon,Empty)>Empty: Concat>Empty, true. ✓
    //   TRUE. ✓
    assert_gt(
        "Concat(Epsilon,Empty) > Concat(Empty,Concat(Empty,Empty))",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Concat; term!(&mut f, Empty), term!(&mut f, Empty))),
    );
}

// == 6. Case f = g: remaining-args dominance check (∀j>k: s >_lpo tⱼ) =========
//
// When the critical index k is found (sₖ >_lpo tₖ), the definition requires
// that s also dominates every tⱼ with j > k.  These tests exercise cases where
// that condition fails, so the result must be false even though a critical k
// exists.  The implementation handles this at util.rs:218-224.

#[test]
fn lpo_equal_root_remaining_not_dominated_simple() {
    // Concat(Epsilon, Empty) ≯ Concat(Empty, Concat(Epsilon, Epsilon))
    //
    // Critical index k=0: Epsilon >_lpo Empty.
    // Remaining check: Concat(Epsilon,Empty) >_lpo Concat(Epsilon,Epsilon)?
    //   Case f=g: k=0 — Epsilon=Epsilon (equal). k=1 — Empty vs Epsilon: Empty < Epsilon → false.
    // Dominance condition fails → overall false.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Epsilon,Empty) ≯ Concat(Empty,Concat(Epsilon,Epsilon))",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Epsilon))),
    );
}

#[test]
fn lpo_equal_root_remaining_is_self() {
    // Concat(Epsilon, Empty) ≯ Concat(Empty, Concat(Epsilon, Empty))
    //
    // Critical index k=0: Epsilon >_lpo Empty.
    // Remaining t₁ = Concat(Epsilon,Empty) = s.  s >_lpo s is false (irreflexivity).
    // A simplification ordering cannot have s > t when s is a subterm of t.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Epsilon,Empty) ≯ Concat(Empty,Concat(Epsilon,Empty))",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty))),
    );
}

#[test]
fn lpo_equal_root_remaining_not_dominated_with_unary_first_arg() {
    // Concat(Star(Epsilon), Empty) ≯ Concat(Empty, Concat(Star(Epsilon), Epsilon))
    //
    // Critical index k=0: Star(Epsilon) >_lpo Empty.
    // Remaining check: Concat(Star(Epsilon),Empty) >_lpo Concat(Star(Epsilon),Epsilon)?
    //   Case f=g: k=0 — Star(Epsilon)=Star(Epsilon). k=1 — Empty vs Epsilon: Empty < Epsilon → false.
    // Dominance condition fails → overall false.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt(
        "Concat(Star(Epsilon),Empty) ≯ Concat(Empty,Concat(Star(Epsilon),Epsilon))",
        &term!(&mut f, Concat; term!(&mut f, Star; term!(&mut f, Epsilon)), term!(&mut f, Empty)),
        &term!(&mut f, Concat; term!(&mut f, Empty),                term!(&mut f, Concat; term!(&mut f, Star; term!(&mut f, Epsilon)), term!(&mut f, Epsilon))),
    );
}

#[test]
fn lpo_equal_root_ternary_remaining_domination_fails() {
    // Concat is binary; a right-associated chain simulates a ternary argument list.
    // Concat(Epsilon, Concat(Empty, Epsilon)) ≯ Concat(Empty, Concat(Epsilon, Concat(Epsilon, Epsilon)))
    //
    // Outermost (Concat=Concat): k=0: Epsilon >_lpo Empty. Critical.
    // Remaining: Concat(Epsilon,Concat(Empty,Epsilon)) >_lpo Concat(Epsilon,Concat(Epsilon,Epsilon))?
    //   Concat=Concat: k=0 — Epsilon=Epsilon. k=1 — Concat(Empty,Epsilon) vs Concat(Epsilon,Epsilon):
    //     Concat=Concat: k=0 — Empty vs Epsilon: Empty < Epsilon → false.
    //   Concat(Epsilon,Concat(Empty,Epsilon)) ≯ Concat(Epsilon,Concat(Epsilon,Epsilon)) → overall false.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_not_gt("Concat(Epsilon,Concat(Empty,Epsilon)) ≯ Concat(Empty,Concat(Epsilon,Concat(Epsilon,Epsilon)))",
        &term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Epsilon))),
        &term!(&mut f, Concat; term!(&mut f, Empty),   term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Epsilon)))));
}

// == 7. lexicographic_path_ordering wrapper (three-way Ordering) ==============

#[test]
fn lpo_ordering_wrapper_equal() {
    // Structurally identical terms must compare as Equal.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = term!(&mut f, Concat; term!(&mut f, Epsilon), term!(&mut f, Empty));
    assert_eq!(
        lexicographic_path_ordering(&t, &t.clone(), &lpo_cmp),
        Ordering::Equal
    );
    assert_eq!(
        lexicographic_path_ordering(&term!(&mut f, Epsilon), &term!(&mut f, Epsilon), &lpo_cmp),
        Ordering::Equal
    );
}

#[test]
fn lpo_ordering_wrapper_greater() {
    // Star(Empty) >_lpo Empty → should return Greater.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        lexicographic_path_ordering(
            &term!(&mut f, Star; term!(&mut f, Empty)),
            &term!(&mut f, Empty),
            &lpo_cmp
        ),
        Ordering::Greater
    );
}

#[test]
fn lpo_ordering_wrapper_less() {
    // Empty <_lpo Star(Empty) → should return Less.
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_eq!(
        lexicographic_path_ordering(
            &term!(&mut f, Empty),
            &term!(&mut f, Star; term!(&mut f, Empty)),
            &lpo_cmp
        ),
        Ordering::Less
    );
}
