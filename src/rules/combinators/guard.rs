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

use crate::position::PositionInLanguageTerm;
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol};

/// A predicate on `(term, context_term, position)` that gates whether a rewrite
/// rule may fire at a given position.
///
/// Use [`GuardedRule`](crate::rules::combinators::guarded::GuardedRule)
/// to attach a guard to any existing rule without modifying it.
pub trait RewriteApplicationGuard<LOS: RewritableLanguageOperatorSymbol> {
    /// Returns `true` if the guard holds at `position` inside `context_term`,
    /// where `term` is the sub-term at that position.
    fn allows(
        &self,
        term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position: &PositionInLanguageTerm,
    ) -> bool;
}

/// Fires only at the root of the context term (depth 0).
pub struct RootOnlyRewriteApplicationGuard;

impl<LOS: RewritableLanguageOperatorSymbol> RewriteApplicationGuard<LOS>
    for RootOnlyRewriteApplicationGuard
{
    fn allows(
        &self,
        _term: &LanguageTerm<LOS>,
        _context_term: &LanguageTerm<LOS>,
        position: &PositionInLanguageTerm,
    ) -> bool {
        position.get_depth() == 0
    }
}

/// Fires only when the term's root operator differs from its parent's operator.
///
/// This is the canonical guard for rules that internally flatten an entire
/// associative chain (e.g. AC reordering, AC deduplication): the rule fires
/// once at the topmost node of the chain and handles all nested occurrences
/// internally, so there is no need to fire again at each nested node.
pub struct NotUnderSameOpRewriteApplicationGuard;

impl<LOS: RewritableLanguageOperatorSymbol> RewriteApplicationGuard<LOS>
    for NotUnderSameOpRewriteApplicationGuard
{
    fn allows(
        &self,
        term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position: &PositionInLanguageTerm,
    ) -> bool {
        match position.get_parent_position() {
            None => true,
            Some(parent_pos) => match context_term.get_sub_term_at_position(&parent_pos) {
                None => true,
                Some(parent) => parent.operator != term.operator,
            },
        }
    }
}

/// Fires only when the immediate parent operator satisfies the given predicate.
///
/// Returns `false` when the term is at the root (no parent exists).
pub struct OnlyUnderOpRewriteApplicationGuard<LOS: RewritableLanguageOperatorSymbol> {
    pred: Box<dyn Fn(&LOS) -> bool>,
}

impl<LOS: RewritableLanguageOperatorSymbol> OnlyUnderOpRewriteApplicationGuard<LOS> {
    /// Creates the guard with the given parent-operator predicate.
    pub fn new<F: Fn(&LOS) -> bool + 'static>(pred: F) -> Self {
        Self {
            pred: Box::new(pred),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteApplicationGuard<LOS>
    for OnlyUnderOpRewriteApplicationGuard<LOS>
{
    fn allows(
        &self,
        _term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position: &PositionInLanguageTerm,
    ) -> bool {
        match position.get_parent_position() {
            None => false,
            Some(parent_pos) => match context_term.get_sub_term_at_position(&parent_pos) {
                None => false,
                Some(parent) => (self.pred)(&parent.operator),
            },
        }
    }
}

/// Fires only when the term's content satisfies the given predicate.
/// Context and position are ignored.
///
/// This is the content-predicate counterpart to the positional guards.
/// Use it with [`GuardedRule`](crate::rules::combinators::guarded::GuardedRule)
/// to keep applicability conditions separate from rewrite logic.
#[allow(clippy::type_complexity)]
pub struct TermPredicateRewriteApplicationGuard<LOS: RewritableLanguageOperatorSymbol> {
    pred: Box<dyn Fn(&LanguageTerm<LOS>) -> bool>,
}

impl<LOS: RewritableLanguageOperatorSymbol> TermPredicateRewriteApplicationGuard<LOS> {
    /// Creates the guard with the given term predicate.
    pub fn new<F: Fn(&LanguageTerm<LOS>) -> bool + 'static>(pred: F) -> Self {
        Self {
            pred: Box::new(pred),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteApplicationGuard<LOS>
    for TermPredicateRewriteApplicationGuard<LOS>
{
    fn allows(
        &self,
        term: &LanguageTerm<LOS>,
        _context_term: &LanguageTerm<LOS>,
        _position: &PositionInLanguageTerm,
    ) -> bool {
        (self.pred)(term)
    }
}

/// A rewrite application guard backed by a plain closure over the full
/// `(term, context_term, position)` triple.
#[allow(clippy::type_complexity)]
pub struct ClosureRewriteApplicationGuard<LOS: RewritableLanguageOperatorSymbol> {
    pred: Box<dyn Fn(&LanguageTerm<LOS>, &LanguageTerm<LOS>, &PositionInLanguageTerm) -> bool>,
}

impl<LOS: RewritableLanguageOperatorSymbol> ClosureRewriteApplicationGuard<LOS> {
    /// Creates the guard with the given closure.
    pub fn new<F>(pred: F) -> Self
    where
        F: Fn(&LanguageTerm<LOS>, &LanguageTerm<LOS>, &PositionInLanguageTerm) -> bool + 'static,
    {
        Self {
            pred: Box::new(pred),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteApplicationGuard<LOS>
    for ClosureRewriteApplicationGuard<LOS>
{
    fn allows(
        &self,
        term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position: &PositionInLanguageTerm,
    ) -> bool {
        (self.pred)(term, context_term, position)
    }
}
