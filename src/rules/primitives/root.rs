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
use crate::rule::RewriteRule;
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// A rewrite rule that fires only when the root operator of the current term
/// passes a guard predicate.
///
/// The rewrite closure receives the root operator, the children slice, and the
/// factory for constructing the result.  For context-sensitive rules, use
/// [`ClosureRewriteRule`](crate::rule::ClosureRewriteRule).
///
/// # Example
///
/// ```rust
/// use simple_term_rewriter::rules::primitives::root::RootRule;
/// use simple_term_rewriter::term::syntax::{LanguageTermNode, TermFactory, RewritableLanguageOperatorSymbol};
/// use hashconsing::HConsign;
///
/// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// enum Op { Not, True, False }
/// impl RewritableLanguageOperatorSymbol for Op {
///     fn arity(&self) -> simple_term_rewriter::term::syntax::LanguageOperatorArity {
///         use simple_term_rewriter::term::syntax::LanguageOperatorArity::Fixed;
///         match self { Op::Not => Fixed(1), _ => Fixed(0) }
///     }
/// }
///
/// // NOT(NOT(x)) → x
/// let double_neg = RootRule::unary(
///     "double negation",
///     |op: &Op| *op == Op::Not,
///     |_op, child, _f| {
///         if child.operator == Op::Not {
///             Some(child.sub_terms[0].clone())
///         } else {
///             None
///         }
///     },
/// );
/// ```
#[allow(clippy::type_complexity)]
pub struct RootRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    guard: Box<dyn Fn(&LOS) -> bool>,
    rewrite:
        Box<dyn Fn(&LOS, &[LanguageTerm<LOS>], &mut TermFactory<LOS>) -> Option<LanguageTerm<LOS>>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> RootRule<LOS> {
    /// Creates a rule that fires when `guard(root_op)` is true, then calls
    /// `rewrite(root_op, children, factory)`.
    pub fn new<G, R>(desc: impl Into<String>, guard: G, rewrite: R) -> Self
    where
        G: Fn(&LOS) -> bool + 'static,
        R: Fn(&LOS, &[LanguageTerm<LOS>], &mut TermFactory<LOS>) -> Option<LanguageTerm<LOS>>
            + 'static,
    {
        Self {
            desc: desc.into(),
            guard: Box::new(guard),
            rewrite: Box::new(rewrite),
        }
    }

    /// Creates a rule for **unary** operators (exactly one child).
    pub fn unary<G, R>(desc: impl Into<String>, guard: G, rewrite: R) -> Self
    where
        G: Fn(&LOS) -> bool + 'static,
        R: Fn(&LOS, &LanguageTerm<LOS>, &mut TermFactory<LOS>) -> Option<LanguageTerm<LOS>>
            + 'static,
    {
        Self::new(desc, guard, move |op, children, f| {
            rewrite(op, &children[0], f)
        })
    }

    /// Creates a rule for **binary** operators (exactly two children).
    pub fn binary<G, R>(desc: impl Into<String>, guard: G, rewrite: R) -> Self
    where
        G: Fn(&LOS) -> bool + 'static,
        R: Fn(
                &LOS,
                &LanguageTerm<LOS>,
                &LanguageTerm<LOS>,
                &mut TermFactory<LOS>,
            ) -> Option<LanguageTerm<LOS>>
            + 'static,
    {
        Self::new(desc, guard, move |op, children, f| {
            rewrite(op, &children[0], &children[1], f)
        })
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for RootRule<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        _ctx: &LanguageTerm<LOS>,
        _pos: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        if (self.guard)(&term.operator) {
            (self.rewrite)(&term.operator, &term.sub_terms, factory)
        } else {
            None
        }
    }
}
