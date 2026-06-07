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

use std::fmt::{self, Debug};
use std::hash::Hash;

use hashconsing::{HConsed, HConsign, HashConsign};

use crate::position::PositionInLanguageTerm;

/// The arity of a language operator symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageOperatorArity {
    /// The operator always takes exactly this many arguments.
    Fixed(usize),
    /// The operator takes any number of arguments; the actual count is
    /// determined by the number of sub-terms in a given term node.
    Variadic,
}

/// The operator-symbol type for a language that can be rewritten by this crate.
///
/// Implement this trait on your operator enum to make it usable with
/// [`LanguageTerm`], [`RewriteRule`](crate::rule::RewriteRule), and the
/// rest of the rewriting engine.
pub trait RewritableLanguageOperatorSymbol:
    Clone + PartialEq + Eq + Hash + Debug + 'static
{
    /// Returns the arity of this operator.
    fn arity(&self) -> LanguageOperatorArity;
}

/// The internal node of a hash-consed term tree.
///
/// Previously named `LanguageTerm`; renamed so the type alias below can
/// take the shorter name.  Build nodes through a [`TermFactory`] via
/// [`LanguageTermNode::build`] rather than constructing directly.
///
/// `HConsed<LanguageTermNode<LOS>>` (= [`LanguageTerm<LOS>`]) implements
/// `Deref<Target = LanguageTermNode<LOS>>`, so `term.operator` and
/// `term.sub_terms` work transparently on a handle.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LanguageTermNode<LOS: RewritableLanguageOperatorSymbol> {
    pub operator: LOS,
    pub sub_terms: Vec<LanguageTerm<LOS>>,
}

/// A hash-consed term handle.
///
/// Two handles are equal (O(1)) iff they refer to structurally identical
/// trees that were built through the **same** [`TermFactory`].
pub type LanguageTerm<LOS> = HConsed<LanguageTermNode<LOS>>;

/// The factory used to intern [`LanguageTerm`]s.
///
/// Obtain one with `HConsign::empty()` or `HConsign::with_capacity(n)`.
/// Pass `&mut factory` to every function that constructs a new term.
pub type TermFactory<LOS> = HConsign<LanguageTermNode<LOS>>;

impl<LOS: RewritableLanguageOperatorSymbol> LanguageTermNode<LOS> {
    /// Interns a new node with the given operator and already-interned children.
    ///
    /// Returns the existing handle if the node is already present in `factory`.
    pub fn build(
        operator: LOS,
        sub_terms: Vec<LanguageTerm<LOS>>,
        factory: &mut TermFactory<LOS>,
    ) -> LanguageTerm<LOS> {
        factory.mk(LanguageTermNode {
            operator,
            sub_terms,
        })
    }

    /// Returns a reference to the sub-node at `pos`, or `None` if the path is
    /// out of bounds.
    ///
    /// Position `[]` returns the node itself.  Position `[n]` returns child `n`.
    /// Deeper positions recurse.
    ///
    /// Callers holding a `&LanguageTerm<LOS>` (= `&HConsed<LanguageTermNode<LOS>>`)
    /// can invoke this via `Deref`: `term.get_sub_term_at_position(pos)`.
    pub fn get_sub_term_at_position<'a>(
        &'a self,
        pos: &PositionInLanguageTerm,
    ) -> Option<&'a LanguageTermNode<LOS>> {
        self.get_sub_term_at_position_rec(pos.get_absolute_coordinates_from_root())
    }

    fn get_sub_term_at_position_rec<'a>(
        &'a self,
        abs_pos: &[usize],
    ) -> Option<&'a LanguageTermNode<LOS>> {
        if abs_pos.is_empty() {
            Some(self)
        } else {
            let n = abs_pos[0];
            match self.sub_terms.get(n) {
                None => None,
                Some(child) => child.get_sub_term_at_position_rec(&abs_pos[1..]),
            }
        }
    }
}

impl<LOS: RewritableLanguageOperatorSymbol + fmt::Display> fmt::Display for LanguageTermNode<LOS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sub_terms.is_empty() {
            write!(f, "{}", self.operator)
        } else {
            write!(f, "{}(", self.operator)?;
            for (i, sub) in self.sub_terms.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", sub)?;
            }
            write!(f, ")")
        }
    }
}

/// Construct a [`LanguageTerm`] concisely.
///
/// # Syntax
///
/// ```text
/// term!(factory, OP)              // leaf
/// term!(factory, OP; t1, t2, ...) // node
/// ```
///
/// # Example
///
/// ```rust
/// use simple_term_rewriter::term;
/// use simple_term_rewriter::term::syntax::{LanguageTermNode, TermFactory, RewritableLanguageOperatorSymbol};
/// use hashconsing::HConsign;
///
/// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// enum Op { And, Not, True, False }
/// impl RewritableLanguageOperatorSymbol for Op {
///     fn arity(&self) -> simple_term_rewriter::term::syntax::LanguageOperatorArity {
///         use simple_term_rewriter::term::syntax::LanguageOperatorArity::Fixed;
///         match self { Op::And => Fixed(2), Op::Not => Fixed(1), _ => Fixed(0) }
///     }
/// }
///
/// let mut f: TermFactory<Op> = HConsign::empty();
/// let t = term!(&mut f, Op::And; term!(&mut f, Op::Not; term!(&mut f, Op::True)), term!(&mut f, Op::False));
/// ```
#[macro_export]
macro_rules! term {
    ($factory:expr, $op:expr) => {
        $crate::term::syntax::LanguageTermNode::build($op, vec![], $factory)
    };
    ($factory:expr, $op:expr ; $($sub:expr),+ $(,)?) => {
        $crate::term::syntax::LanguageTermNode::build($op, vec![$($sub),+], $factory)
    };
}
