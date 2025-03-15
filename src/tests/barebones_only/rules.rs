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


use std::fmt;

use crate::core::{rule::RewriteRule, terms::position::PositionInLanguageTerm};
use crate::core::terms::term::LanguageTerm;
use crate::tests::boolean_logic::lang::SimplisticBooleanLogicOperators;







#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MinimalExampleTransformationKind {
    DoubleNegation,
    EvaluateNeg,
    EvaluateAnd,
    EvaluateOr
}


impl fmt::Display for MinimalExampleTransformationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}





impl RewriteRule<SimplisticBooleanLogicOperators> for MinimalExampleTransformationKind {

    fn try_apply(
        &self, 
        term : &LanguageTerm<SimplisticBooleanLogicOperators>,
        _context_term : &LanguageTerm<SimplisticBooleanLogicOperators>,
        _position_in_context_term : &PositionInLanguageTerm
    ) 
    -> Option<LanguageTerm<SimplisticBooleanLogicOperators>> {
        match self {
            MinimalExampleTransformationKind::DoubleNegation => {
                if term.operator == SimplisticBooleanLogicOperators::NEG {
                    let sub_term = term.sub_terms.first().unwrap();
                    if sub_term.operator == SimplisticBooleanLogicOperators::NEG {
                        Some(sub_term.sub_terms.first().unwrap().clone())
                    } else {
                        None 
                    }
                } else {
                    None 
                }
            },
            MinimalExampleTransformationKind::EvaluateNeg => {
                if term.operator == SimplisticBooleanLogicOperators::NEG {
                    let sub_term = term.sub_terms.first().unwrap();
                    if sub_term.operator == SimplisticBooleanLogicOperators::TRUE {
                        Some(LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE, vec![]))
                    } else if sub_term.operator == SimplisticBooleanLogicOperators::FALSE {
                        Some(LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE, vec![]))
                    } else {
                        None
                    }
                } else {
                    None 
                }
            },
            MinimalExampleTransformationKind::EvaluateAnd => {
                if term.operator == SimplisticBooleanLogicOperators::AND {
                    let sub_term1 = term.sub_terms.first().unwrap();
                    let sub_term2 = term.sub_terms.get(1).unwrap();
                    match (&sub_term1.operator, &sub_term2.operator) {
                        (SimplisticBooleanLogicOperators::TRUE, SimplisticBooleanLogicOperators::TRUE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::TRUE, SimplisticBooleanLogicOperators::FALSE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::FALSE, SimplisticBooleanLogicOperators::TRUE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::FALSE, SimplisticBooleanLogicOperators::FALSE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE, vec![]))
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
                if term.operator == SimplisticBooleanLogicOperators::OR {
                    let sub_term1 = term.sub_terms.first().unwrap();
                    let sub_term2 = term.sub_terms.get(1).unwrap();
                    match (&sub_term1.operator, &sub_term2.operator) {
                        (SimplisticBooleanLogicOperators::TRUE, SimplisticBooleanLogicOperators::TRUE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::TRUE, SimplisticBooleanLogicOperators::FALSE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::FALSE, SimplisticBooleanLogicOperators::TRUE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE, vec![]))
                        },
                        (SimplisticBooleanLogicOperators::FALSE, SimplisticBooleanLogicOperators::FALSE) => {
                            Some(LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE, vec![]))
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
    
    fn get_desc(&self) -> String {
        self.to_string()
    }
}