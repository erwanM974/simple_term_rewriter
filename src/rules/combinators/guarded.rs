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
use crate::rules::combinators::guard::RewriteApplicationGuard;
use crate::term::syntax::{LanguageTerm, RewritableLanguageOperatorSymbol, TermFactory};

/// Wraps any rewrite rule with a [`RewriteApplicationGuard`], restricting the
/// positions at which the rule may fire.
///
/// # Example
///
/// ```rust
/// use simple_term_rewriter::rules::combinators::guarded::GuardedRule;
/// use simple_term_rewriter::rules::combinators::guard::NotUnderSameOpRewriteApplicationGuard;
/// use simple_term_rewriter::rules::primitives::root::RootRule;
/// use simple_term_rewriter::term::syntax::RewritableLanguageOperatorSymbol;
///
/// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// enum Op { Add, Zero, X }
/// impl RewritableLanguageOperatorSymbol for Op {
///     fn arity(&self) -> simple_term_rewriter::term::syntax::LanguageOperatorArity {
///         use simple_term_rewriter::term::syntax::LanguageOperatorArity::Fixed;
///         match self { Op::Add => Fixed(2), _ => Fixed(0) }
///     }
/// }
///
/// let rule = GuardedRule::new(
///     RootRule::new("example", |op: &Op| *op == Op::Add, |_op, _children, _f| None),
///     NotUnderSameOpRewriteApplicationGuard,
/// );
/// ```
pub struct GuardedRule<LOS: RewritableLanguageOperatorSymbol> {
    inner: Box<dyn RewriteRule<LOS>>,
    guard: Box<dyn RewriteApplicationGuard<LOS>>,
}

impl<LOS: RewritableLanguageOperatorSymbol> GuardedRule<LOS> {
    pub fn new(
        inner: impl RewriteRule<LOS> + 'static,
        guard: impl RewriteApplicationGuard<LOS> + 'static,
    ) -> Self {
        Self {
            inner: Box::new(inner),
            guard: Box::new(guard),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for GuardedRule<LOS> {
    fn get_desc(&self) -> String {
        self.inner.get_desc()
    }

    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        ctx: &LanguageTerm<LOS>,
        pos: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        if self.guard.allows(term, ctx, pos) {
            self.inner.try_apply(term, ctx, pos, factory)
        } else {
            None
        }
    }
}
