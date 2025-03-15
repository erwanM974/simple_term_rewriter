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


use graph_process_manager_core::queue::priorities::AbstractPriorities;

use crate::core::terms::term::RewritableLanguageOperatorSymbol;

use super::step::RewriteStepKind;



pub struct RewritePriorities {}



impl<LOS : RewritableLanguageOperatorSymbol> AbstractPriorities<RewriteStepKind<LOS>> for RewritePriorities {
    fn get_priority_of_step(
        &self,
        _step: &RewriteStepKind<LOS>
    ) -> i32 {
        0
    }

}

