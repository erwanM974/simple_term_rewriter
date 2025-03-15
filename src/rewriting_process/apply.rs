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


use crate::core::terms::position::*;
use crate::core::rule::RewriteRule;
use crate::core::terms::term::LanguageTerm;
use crate::core::terms::term::RewritableLanguageOperatorSymbol;


/** 
 * The result of the application:
 *   - of a given rewrite rule, unambiguously referred to via:
 *     + the phase index
 *     + the rule index in the phase
 *   - at a given position in the term
 * **/
 pub struct TermTransformationResult<LOS : RewritableLanguageOperatorSymbol> {
    pub phase_index : usize,
    pub rule_index_in_phase : usize,
    pub position : PositionInLanguageTerm,
    /// this is an Option so that we may take it later to propagate the result in the rewriting process without clone
    pub result : Option<LanguageTerm<LOS>>
 }

 
impl<LOS : RewritableLanguageOperatorSymbol>  TermTransformationResult<LOS> {
    pub fn new(
        phase_index : usize,
        rule_index_in_phase : usize,
        position : PositionInLanguageTerm,
        result : LanguageTerm<LOS>) -> Self {
        Self{
            phase_index,
            rule_index_in_phase,
            position,
            result : Some(result)
        }
    }
}





pub fn get_transformations<LOS : RewritableLanguageOperatorSymbol>(
    phase_index : usize,
    rewrite_rules : &Vec<Box<dyn RewriteRule<LOS>>>,
    term : &LanguageTerm<LOS>,
    keep_only_one : bool
) 
        -> Vec<TermTransformationResult<LOS>>
{   
    get_transformations_rec(
        phase_index,
        rewrite_rules,
        term,
        keep_only_one,
        term,
        &PositionInLanguageTerm::get_root_position()
    )
}




fn get_transformations_rec<LOS : RewritableLanguageOperatorSymbol>(
    phase_index : usize,
    rewrite_rules : &Vec<Box<dyn RewriteRule<LOS>>>,
    term : &LanguageTerm<LOS>,
    keep_only_one : bool,
    context_term : &LanguageTerm<LOS>,
    position_in_context_term : &PositionInLanguageTerm
) 
        -> Vec<TermTransformationResult<LOS>>
{   
    eprintln!("get transfos at pos {:}", position_in_context_term);
    let mut results = get_root_transformations(
        phase_index,
        rewrite_rules,
        term,
        keep_only_one,
        context_term,
        position_in_context_term
    );
    if keep_only_one && !results.is_empty() {
        return results;
    }
    for (n,sub_term) in term.sub_terms.iter().enumerate() {
        let sub_position = position_in_context_term.get_position_of_nth_child(n);
        for sub_transfo in get_transformations_rec::<LOS>(
            phase_index,
            rewrite_rules, 
            sub_term, 
            keep_only_one,
            context_term,
            &sub_position
        ) {
            let mut upd_sub_terms : Vec<LanguageTerm<LOS>> = term.sub_terms.clone();
            upd_sub_terms.remove(n);
            upd_sub_terms.insert(n,sub_transfo.result.unwrap());
            // ***
            let res = TermTransformationResult::new(
                sub_transfo.phase_index,
                sub_transfo.rule_index_in_phase,
                sub_transfo.position,
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



fn get_root_transformations<LOS : RewritableLanguageOperatorSymbol>(
    phase_index : usize,
    rewrite_rules : &Vec<Box<dyn RewriteRule<LOS>>>,
    term : &LanguageTerm<LOS>,
    keep_only_one : bool,
    context_term : &LanguageTerm<LOS>,
    position_in_context_term : &PositionInLanguageTerm
) -> Vec<TermTransformationResult<LOS>>
{   
    let mut results = vec![];
    for (rule_index,rule) in rewrite_rules.iter().enumerate() {
        eprintln!("try applying rule {:}", rule.get_desc());
        if let Some(result) = rule.try_apply(
            term,
            context_term,
            position_in_context_term
        ) {
            results.push(
                TermTransformationResult::new(
                    phase_index,
                    rule_index,
                    position_in_context_term.clone(),
                    result
                )
            );
            if keep_only_one {
                return results;
            }
        }
    }
    results
}





