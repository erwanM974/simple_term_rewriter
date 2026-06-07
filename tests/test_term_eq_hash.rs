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

//! Explicit tests for [`LanguageTerm`] equality and hash consistency.

mod common;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use hashconsing::HConsign;

use common::regex::constructors::*;
use common::regex::lang::RegexOp;
use simple_term_rewriter::term::syntax::{LanguageTerm, TermFactory};

fn hash_of(t: &LanguageTerm<RegexOp>) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

// == equal terms have equal hashes =============================================

#[test]
fn independently_built_equal_terms_are_equal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    // With hash-consing from the same factory, structurally equal terms get the
    // same uid and are therefore identical handles.
    let t1 = concat(star(epsilon(&mut f), &mut f), empty(&mut f), &mut f);
    let t2 = concat(star(epsilon(&mut f), &mut f), empty(&mut f), &mut f);
    assert_eq!(t1, t2);
}

#[test]
fn independently_built_equal_terms_have_equal_hashes() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t1 = concat(star(epsilon(&mut f), &mut f), empty(&mut f), &mut f);
    let t2 = concat(star(epsilon(&mut f), &mut f), empty(&mut f), &mut f);
    assert_eq!(hash_of(&t1), hash_of(&t2));
}

#[test]
fn clone_equals_original() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(atom(b'x', &mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        &mut f,
    );
    assert_eq!(t, t.clone());
}

#[test]
fn clone_has_same_hash() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = concat(
        star(atom(b'x', &mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f),
        &mut f,
    );
    assert_eq!(hash_of(&t), hash_of(&t.clone()));
}

#[test]
fn deeply_nested_equal_terms_are_equal_and_same_hash() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t1 = concat(
        star(
            concat(
                epsilon(&mut f),
                alt(empty(&mut f), atom(b'x', &mut f), &mut f),
                &mut f,
            ),
            &mut f,
        ),
        atom(b'y', &mut f),
        &mut f,
    );
    let t2 = concat(
        star(
            concat(
                epsilon(&mut f),
                alt(empty(&mut f), atom(b'x', &mut f), &mut f),
                &mut f,
            ),
            &mut f,
        ),
        atom(b'y', &mut f),
        &mut f,
    );
    assert_eq!(t1, t2);
    assert_eq!(hash_of(&t1), hash_of(&t2));
}

// == structural differences produce inequality =================================

#[test]
fn different_operators_are_not_equal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_ne!(epsilon(&mut f), empty(&mut f));
    assert_ne!(
        star(epsilon(&mut f), &mut f),
        alt(epsilon(&mut f), empty(&mut f), &mut f)
    );
}

#[test]
fn same_operator_different_children_are_not_equal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_ne!(
        concat(epsilon(&mut f), empty(&mut f), &mut f),
        concat(empty(&mut f), epsilon(&mut f), &mut f)
    );
}

#[test]
fn different_depths_are_not_equal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_ne!(
        star(epsilon(&mut f), &mut f),
        star(star(epsilon(&mut f), &mut f), &mut f)
    );
}

#[test]
fn different_atoms_are_not_equal() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    assert_ne!(atom(b'a', &mut f), atom(b'b', &mut f));
}

// == equivalence-relation axioms ===============================================

#[test]
fn equality_is_reflexive() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t = star(concat(atom(b'a', &mut f), epsilon(&mut f), &mut f), &mut f);
    assert_eq!(t, t.clone());
}

#[test]
fn equality_is_symmetric() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t1 = alt(atom(b'a', &mut f), epsilon(&mut f), &mut f);
    let t2 = alt(atom(b'a', &mut f), epsilon(&mut f), &mut f);
    assert_eq!(t1, t2);
    assert_eq!(t2, t1);
}

#[test]
fn equality_is_transitive() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let t1 = alt(atom(b'a', &mut f), epsilon(&mut f), &mut f);
    let t2 = alt(atom(b'a', &mut f), epsilon(&mut f), &mut f);
    let t3 = alt(atom(b'a', &mut f), epsilon(&mut f), &mut f);
    assert_eq!(t1, t2);
    assert_eq!(t2, t3);
    assert_eq!(t1, t3);
}

// == usable in standard collections ============================================

#[test]
fn usable_as_hashmap_key() {
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let mut map: HashMap<LanguageTerm<RegexOp>, &str> = HashMap::new();
    let key = concat(epsilon(&mut f), empty(&mut f), &mut f);
    let lookup = concat(epsilon(&mut f), empty(&mut f), &mut f); // same uid from same factory
    map.insert(key, "found");
    assert_eq!(map.get(&lookup), Some(&"found"));
}

#[test]
fn hashset_deduplicates_equal_terms() {
    use std::collections::HashSet;
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let mut set: HashSet<LanguageTerm<RegexOp>> = HashSet::new();
    let t = alt(atom(b'a', &mut f), atom(b'b', &mut f), &mut f);
    set.insert(t.clone());
    set.insert(t.clone());
    assert_eq!(set.len(), 1);
}

#[test]
fn hashset_keeps_distinct_terms_separate() {
    use std::collections::HashSet;
    let mut f: TermFactory<RegexOp> = HConsign::empty();
    let mut set: HashSet<LanguageTerm<RegexOp>> = HashSet::new();
    set.insert(epsilon(&mut f));
    set.insert(empty(&mut f));
    set.insert(star(epsilon(&mut f), &mut f));
    assert_eq!(set.len(), 3);
}
