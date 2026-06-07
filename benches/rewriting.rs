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

//! Benchmark: outermost normalization of a term tree.
//!
//! Input: a balanced binary tree of `Alt` nodes whose leaves are all
//! `Star(Star(Atom))`.  The rule `Star(Star(r)) → Star(r)` fires once per
//! leaf; the strategy must traverse every node to find and reduce each one.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use hashconsing::HConsign;
use simple_term_rewriter::process::strategy::{DepthOrder, RewriteProcess, SiblingOrder};
use simple_term_rewriter::process::untraced::RewriteProcessUntracedExecutor;
use simple_term_rewriter::rule::ClosureRewriteRule;
use simple_term_rewriter::term::syntax::{
    LanguageOperatorArity, LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol,
    TermFactory,
};

// == language ==================================================================

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RegexOp {
    Atom,
    Alt,
    Star,
}

impl RewritableLanguageOperatorSymbol for RegexOp {
    fn arity(&self) -> LanguageOperatorArity {
        match self {
            RegexOp::Atom => LanguageOperatorArity::Fixed(0),
            RegexOp::Alt => LanguageOperatorArity::Fixed(2),
            RegexOp::Star => LanguageOperatorArity::Fixed(1),
        }
    }
}

fn atom(f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    LanguageTermNode::build(RegexOp::Atom, vec![], f)
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

/// Balanced Alt tree of depth `d`; leaves are `Star(Star(Atom))`.
fn build_term(depth: usize, f: &mut TermFactory<RegexOp>) -> LanguageTerm<RegexOp> {
    if depth == 0 {
        star(star(atom(f), f), f)
    } else {
        alt(build_term(depth - 1, f), build_term(depth - 1, f), f)
    }
}

// == rule and strategy =========================================================

/// Star(Star(r)) → Star(r)
fn double_star_rule() -> ClosureRewriteRule<RegexOp> {
    ClosureRewriteRule::new("double_star", |t, _, _, _| {
        if t.operator != RegexOp::Star {
            return None;
        }
        if t.sub_terms[0].operator != RegexOp::Star {
            return None;
        }
        Some(t.sub_terms[0].clone())
    })
}

/// Outermost normalization to fixpoint.
fn normalize_process() -> RewriteProcess<RegexOp> {
    RewriteProcess::Repeat(Box::new(RewriteProcess::TryOnePath(vec![
        RewriteProcess::Rule(Box::new(double_star_rule())),
        RewriteProcess::AnyChild(
            SiblingOrder::Leftmost,
            DepthOrder::Outermost,
            Box::new(RewriteProcess::Rule(Box::new(double_star_rule()))),
        ),
    ])))
}

// == benchmark =================================================================

fn bench_normalize(c: &mut Criterion) {
    let f = &mut HConsign::empty();
    let mut group = c.benchmark_group("normalize_double_star");
    for depth in [4usize, 6, 8] {
        let term = build_term(depth, f);
        let process = normalize_process();
        group.bench_with_input(BenchmarkId::new("tree_depth", depth), &term, |b, t| {
            b.iter(|| RewriteProcessUntracedExecutor::rewrite(&process, black_box(t), f))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_normalize);
criterion_main!(benches);
