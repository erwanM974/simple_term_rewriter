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

use crate::core::term::RewritableLanguageOperatorSymbol;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MinimalExampleLangOperators {
    TRUE,
    FALSE,
    OR,
    AND,
    NEG 
}

impl RewritableLanguageOperatorSymbol for MinimalExampleLangOperators {}


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

