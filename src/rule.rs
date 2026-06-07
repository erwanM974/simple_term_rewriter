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
use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

/// A rewrite rule that may be applied at any position in a term.
///
/// The library's traversal engine calls [`try_apply`](RewriteRule::try_apply) at every
/// sub-term position; the rule decides whether to fire.
///
/// # Implementing a rule
///
/// There are three ways to obtain a `Box<dyn RewriteRule<LOS>>`, in order of
/// increasing boilerplate and power:
///
/// 1. **[`RootRule`](crate::rules::primitives::root::RootRule)** — closure over the root operator
///    and its children; no context.
/// 2. **[`ClosureRewriteRule`]** — a plain closure over `(term, context, position, factory)`.
/// 3. **`impl RewriteRule<LOS>` on a custom struct** — the escape hatch for rules
///    that carry non-trivial state or require complex inspection.
pub trait RewriteRule<LOS: RewritableLanguageOperatorSymbol> {
    /// Returns a short human-readable description of this rule.
    fn get_desc(&self) -> String;

    /// Attempts to apply the rule to `term`.
    ///
    /// Returns `Some(rewritten)` if the rule fires, `None` if it does not apply.
    ///
    /// `context_term` is the root of the full term currently being rewritten, and
    /// `position_in_context_term` is the position of `term` within it.  Rules that
    /// do not need context may ignore both.
    ///
    /// `factory` must be used whenever the rule needs to construct a new term.
    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position_in_context_term: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>>;
}

/// A rewrite rule backed by a plain closure, requiring no separate struct or
/// `impl` block.
///
/// The closure receives `(term, context_term, position, factory)`.
///
/// # Example
///
/// ```rust
/// use simple_term_rewriter::rule::ClosureRewriteRule;
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
/// let rule: Box<dyn simple_term_rewriter::rule::RewriteRule<Op>> =
///     Box::new(ClosureRewriteRule::new("double negation", |term, _ctx, _pos, _f| {
///         if term.operator != Op::Not { return None; }
///         let child = term.sub_terms.first()?;
///         if child.operator != Op::Not { return None; }
///         Some(child.sub_terms[0].clone())
///     }));
/// ```
#[allow(clippy::type_complexity)]
pub struct ClosureRewriteRule<LOS: RewritableLanguageOperatorSymbol> {
    desc: String,
    apply: Box<
        dyn Fn(
            &LanguageTerm<LOS>,
            &LanguageTerm<LOS>,
            &PositionInLanguageTerm,
            &mut TermFactory<LOS>,
        ) -> Option<LanguageTerm<LOS>>,
    >,
}

impl<LOS: RewritableLanguageOperatorSymbol> ClosureRewriteRule<LOS> {
    /// Creates a new closure-based rule.
    ///
    /// `desc` is the human-readable name returned by [`RewriteRule::get_desc`].
    /// `apply` is the rewrite logic: return `Some(new_term)` to fire, `None` to skip.
    pub fn new<F>(desc: impl Into<String>, apply: F) -> Self
    where
        F: Fn(
                &LanguageTerm<LOS>,
                &LanguageTerm<LOS>,
                &PositionInLanguageTerm,
                &mut TermFactory<LOS>,
            ) -> Option<LanguageTerm<LOS>>
            + 'static,
    {
        Self {
            desc: desc.into(),
            apply: Box::new(apply),
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol> RewriteRule<LOS> for ClosureRewriteRule<LOS> {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn try_apply(
        &self,
        term: &LanguageTerm<LOS>,
        context_term: &LanguageTerm<LOS>,
        position_in_context_term: &PositionInLanguageTerm,
        factory: &mut TermFactory<LOS>,
    ) -> Option<LanguageTerm<LOS>> {
        (self.apply)(term, context_term, position_in_context_term, factory)
    }
}

/// Convenience: build a leaf term (no sub-terms).
///
/// Equivalent to `LanguageTermNode::build(op, vec![], factory)`.
pub fn mk_leaf<LOS: RewritableLanguageOperatorSymbol>(
    op: LOS,
    factory: &mut TermFactory<LOS>,
) -> LanguageTerm<LOS> {
    LanguageTermNode::build(op, vec![], factory)
}
