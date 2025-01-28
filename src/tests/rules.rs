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


use crate::core::{rule::RewriteRule, term::LanguageTerm};

use super::lang::{MinimalExampleInterface, MinimalExampleLangOperators, MinimalExampleTransformationKind};





impl RewriteRule<MinimalExampleInterface> for MinimalExampleTransformationKind {
    fn get_transformation_kind(&self) -> MinimalExampleTransformationKind {
        self.clone()
    }

    fn try_apply(&self, term : &LanguageTerm<MinimalExampleLangOperators>) 
    -> Option<LanguageTerm<MinimalExampleLangOperators>> {
        match self {
            MinimalExampleTransformationKind::DoubleNegation => {
                if term.operator == MinimalExampleLangOperators::NEG {
                    let sub_term = term.sub_terms.first().unwrap();
                    if sub_term.operator == MinimalExampleLangOperators::NEG {
                        Some(sub_term.sub_terms.first().unwrap().clone())
                    } else {
                        None 
                    }
                } else {
                    None 
                }
            },
            MinimalExampleTransformationKind::EvaluateNeg => {
                if term.operator == MinimalExampleLangOperators::NEG {
                    let sub_term = term.sub_terms.first().unwrap();
                    if sub_term.operator == MinimalExampleLangOperators::TRUE {
                        Some(LanguageTerm::new(MinimalExampleLangOperators::FALSE, vec![]))
                    } else if sub_term.operator == MinimalExampleLangOperators::FALSE {
                        Some(LanguageTerm::new(MinimalExampleLangOperators::TRUE, vec![]))
                    } else {
                        None
                    }
                } else {
                    None 
                }
            },
            MinimalExampleTransformationKind::EvaluateAnd => {
                if term.operator == MinimalExampleLangOperators::AND {
                    let sub_term1 = term.sub_terms.first().unwrap();
                    let sub_term2 = term.sub_terms.get(1).unwrap();
                    match (&sub_term1.operator, &sub_term2.operator) {
                        (MinimalExampleLangOperators::TRUE, MinimalExampleLangOperators::TRUE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::TRUE, vec![]))
                        },
                        (MinimalExampleLangOperators::TRUE, MinimalExampleLangOperators::FALSE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::FALSE, vec![]))
                        },
                        (MinimalExampleLangOperators::FALSE, MinimalExampleLangOperators::TRUE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::FALSE, vec![]))
                        },
                        (MinimalExampleLangOperators::FALSE, MinimalExampleLangOperators::FALSE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::FALSE, vec![]))
                        },
                        (_,_) => {
                            None 
                        }
                    }
                } else {
                    None 
                }
            },
            MinimalExampleTransformationKind::EvaluateOr => {
                if term.operator == MinimalExampleLangOperators::OR {
                    let sub_term1 = term.sub_terms.first().unwrap();
                    let sub_term2 = term.sub_terms.get(1).unwrap();
                    match (&sub_term1.operator, &sub_term2.operator) {
                        (MinimalExampleLangOperators::TRUE, MinimalExampleLangOperators::TRUE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::TRUE, vec![]))
                        },
                        (MinimalExampleLangOperators::TRUE, MinimalExampleLangOperators::FALSE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::TRUE, vec![]))
                        },
                        (MinimalExampleLangOperators::FALSE, MinimalExampleLangOperators::TRUE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::TRUE, vec![]))
                        },
                        (MinimalExampleLangOperators::FALSE, MinimalExampleLangOperators::FALSE) => {
                            Some(LanguageTerm::new(MinimalExampleLangOperators::FALSE, vec![]))
                        },
                        (_,_) => {
                            None 
                        }
                    }
                } else {
                    None 
                }
            },
        }
    }
}