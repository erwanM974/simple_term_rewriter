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

use crate::core::interface::*;
use crate::core::rule::RewriteRule;
use crate::core::term::LanguageTerm;


/** 
 * The result of the application of a given rewrite rule at a given position
 * **/
 pub struct TermTransformationResult<STRI : BarebonesTermRewritingInterface> {
    pub kind : STRI::TransformationKind,
    pub position : PositionInLanguageTerm,
    pub result : LanguageTerm<STRI::LanguageOperatorSymbol>
 }

 
impl<STRI : BarebonesTermRewritingInterface>  TermTransformationResult<STRI> {
    pub fn new(
        kind : STRI::TransformationKind,
        position : PositionInLanguageTerm,
        result : LanguageTerm<STRI::LanguageOperatorSymbol>) -> Self {
            Self{kind,position,result}
    }
    pub fn new_at_root(
        kind : STRI::TransformationKind,
        result : LanguageTerm<STRI::LanguageOperatorSymbol>) -> Self {
            Self::new(kind, PositionInLanguageTerm::get_root_position(),result)
    }
}





pub fn get_transformations<STRI : BarebonesTermRewritingInterface>(
    rewrite_rules : &Vec<Box<dyn RewriteRule<STRI>>>,
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>,
    keep_only_one : bool
) 
        -> Vec<TermTransformationResult<STRI>> 
{   
    let mut results = get_root_transformations(rewrite_rules,term,keep_only_one);
    if keep_only_one {
        return results;
    }
    for (n,sub_term) in term.sub_terms.iter().enumerate() {
        for sub_transfo in get_transformations::<STRI>(rewrite_rules, sub_term, keep_only_one) {
            let upd_pos = sub_transfo.position.position_as_nth_sub_term(n);
            let mut upd_sub_terms : Vec<LanguageTerm<STRI::LanguageOperatorSymbol>> = term.sub_terms.clone();
            upd_sub_terms.remove(n);
            upd_sub_terms.insert(n,sub_transfo.result);
            // ***
            let res = TermTransformationResult::new(
                sub_transfo.kind,
                upd_pos,
                LanguageTerm::new(term.operator.clone(), upd_sub_terms) 
            );
            results.push(res);
            if keep_only_one {
                return results;
            }
        }
    }
    results
}



/**
  f
 **/
fn get_root_transformations<STRI : BarebonesTermRewritingInterface>(
    rewrite_rules : &Vec<Box<dyn RewriteRule<STRI>>>,
    term : &LanguageTerm<STRI::LanguageOperatorSymbol>,
    keep_only_one : bool
) 
        -> Vec<TermTransformationResult<STRI>> 
{   
    let mut results = vec![];
    for rule in rewrite_rules {
        if let Some(result) = rule.try_apply(term) {
            results.push(
                TermTransformationResult::new_at_root(rule.get_transformation_kind(), result)
            );
            if keep_only_one {
                return results;
            }
        }
    }
    results
}





