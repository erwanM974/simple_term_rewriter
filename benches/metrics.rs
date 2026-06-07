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

//! Benchmark: builtin term metrics on trees of increasing size.
//!
//! Two input shapes are compared:
//!
//! - **Uniform tree**: balanced `Alt` tree of depth `d` with `Star(Star(Atom))`
//!   at every leaf.  All leaves are structurally identical, so `dag_size` sees
//!   massive sharing (O(depth) distinct sub-terms vs O(2^depth) nodes).
//!
//! - **Varied tree**: same shape but each leaf carries a distinct `Atom(i % 26)`,
//!   so `dag_size ≈ tree_size` (no structural sharing to exploit).
//!
//! The contrast between the two reveals the sharing-detection overhead in
//! `dag_size` vs the savings from early termination when duplicates are found.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use hashconsing::HConsign;
use simple_term_rewriter::metrics::{dag_size, operator_count_by_symbol, tree_size};
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

// == language ==================================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RegexOp {
    Atom(u8),
    Alt,
    Star,
}

impl RewritableLanguageOperatorSymbol for RegexOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            RegexOp::Atom(_) => LanguageOperatorArity::Fixed(0),
            RegexOp::Alt => LanguageOperatorArity::Fixed(2),
            RegexOp::Star => LanguageOperatorArity::Fixed(1),
        }
    }
}

fn star(t: LanguageTerm<RegexOp>, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Star, vec![t], f)
}
fn alt(
    l: LanguageTerm<RegexOp>,
    r: LanguageTerm<RegexOp>,
    f: &mut TermFactory<RegexOp>,
) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Alt, vec![l, r], f)
}

// == input construction ========================================================

/// All leaves identical: Star(Star(Atom(b'a'))).
fn build_uniform(depth: usize, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    if depth == 0 {
        star(
            star(LanguageTermNode::build(RegexOp::Atom(b'a'), vec![], f), f),
            f,
        )
    } else {
        alt(build_uniform(depth - 1, f), build_uniform(depth - 1, f), f)
    }
}

/// Each leaf has a distinct atom index, so no structural sharing.
fn build_varied(
    depth: usize,
    counter: &mut u8,
    f: &mut TermFactory<RegexOp>,
) -> LanguageTerm<RegexOp> {
    if depth == 0 {
        let atom = LanguageTermNode::build(RegexOp::Atom(b'a' + (*counter % 26)), vec![], f);
        *counter = counter.wrapping_add(1);
        star(star(atom, f), f)
    } else {
        let l = build_varied(depth - 1, counter, f);
        let r = build_varied(depth - 1, counter, f);
        alt(l, r, f)
    }
}

// == benchmarks ================================================================

fn bench_tree_size(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("tree_size");
    for depth in [4usize, 6, 8] {
        let term = build_uniform(depth, f);
        group.bench_with_input(BenchmarkId::new("depth", depth), &term, |b, t| {
            b.iter(|| tree_size(black_box(t)))
        });
    }
    group.finish();
}

fn bench_dag_size_uniform(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("dag_size_uniform");
    for depth in [4usize, 6, 8] {
        let term = build_uniform(depth, f);
        group.bench_with_input(BenchmarkId::new("depth", depth), &term, |b, t| {
            b.iter(|| dag_size(black_box(t)))
        });
    }
    group.finish();
}

fn bench_dag_size_varied(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("dag_size_varied");
    for depth in [4usize, 6, 8] {
        let term = build_varied(depth, &mut 0, f);
        group.bench_with_input(BenchmarkId::new("depth", depth), &term, |b, t| {
            b.iter(|| dag_size(black_box(t)))
        });
    }
    group.finish();
}

fn bench_operator_count(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("operator_count");
    for depth in [4usize, 6, 8] {
        let term = build_varied(depth, &mut 0, f);
        group.bench_with_input(BenchmarkId::new("depth", depth), &term, |b, t| {
            b.iter(|| operator_count_by_symbol(black_box(t)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_tree_size,
    bench_dag_size_uniform,
    bench_dag_size_varied,
    bench_operator_count,
);
criterion_main!(benches);
