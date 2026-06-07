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

use crate::term::syntax::{
    LanguageTerm, LanguageTermNode, RewritableLanguageOperatorSymbol, TermFactory,
};

/// Flattens a right- or left-associated chain of a single associative operator
/// into an ordered list of its non-`op` sub-terms.
///
/// ```text
/// op(a, op(b, c))  →  [a, b, c]
/// op(op(a, b), c)  →  [a, b, c]
/// f(a, b)          →  [f(a, b)]   (root ≠ op: treated as a leaf)
/// ```
///
/// The inverse operation is [`fold_associative_sub_terms_recursively`].
pub fn get_associative_sub_terms_recursively<'a, LOS: RewritableLanguageOperatorSymbol>(
    term: &'a LanguageTerm<LOS>,
    considered_associative_operator: &LOS,
) -> Vec<&'a LanguageTerm<LOS>> {
    let mut sub_terms: Vec<&'a LanguageTerm<LOS>> = Vec::new();
    if &term.operator == considered_associative_operator {
        for sub_term in &term.sub_terms {
            sub_terms.extend(get_associative_sub_terms_recursively(
                sub_term,
                considered_associative_operator,
            ));
        }
    } else {
        sub_terms.push(term);
    }
    sub_terms
}

/// Folds a non-empty list of terms into a right-associated binary tree using
/// a given associative operator.
///
/// Given `[t1, t2, …, tn]` and operator `op`, builds:
/// ```text
/// op(t1, op(t2, … op(t_{n-1}, tn) … ))
/// ```
///
/// Special cases:
/// - **2 elements** — returns `op(t1, t2)` directly.
/// - **1 element** — returns that element unchanged (the operator is not inserted).
/// - **0 elements** — returns `Some` of a nullary leaf built from `default_empty_term`
///   if one is provided, otherwise returns `None`.
///
/// This is the inverse of [`get_associative_sub_terms_recursively`].
pub fn fold_associative_sub_terms_recursively<LOS: RewritableLanguageOperatorSymbol>(
    considered_associative_operator: &LOS,
    sub_terms: &mut Vec<LanguageTerm<LOS>>,
    default_empty_term: &Option<LOS>,
    factory: &mut TermFactory<LOS>,
) -> Option<LanguageTerm<LOS>> {
    let n = sub_terms.len();
    match n {
        2 => {
            let t2 = sub_terms.pop().unwrap();
            let t1 = sub_terms.pop().unwrap();
            Some(LanguageTermNode::build(
                considered_associative_operator.clone(),
                vec![t1, t2],
                factory,
            ))
        }
        1 => Some(sub_terms.pop().unwrap()),
        0 => default_empty_term
            .as_ref()
            .map(|empty_op| LanguageTermNode::build(empty_op.clone(), vec![], factory)),
        _ => {
            let t1 = sub_terms.remove(0);
            let t2 = fold_associative_sub_terms_recursively(
                considered_associative_operator,
                sub_terms,
                default_empty_term,
                factory,
            )?;
            Some(LanguageTermNode::build(
                considered_associative_operator.clone(),
                vec![t1, t2],
                factory,
            ))
        }
    }
}
