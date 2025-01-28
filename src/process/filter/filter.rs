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

use graph_process_manager_core::handler::filter::AbstractFilter;
use crate::process::filter::elim::RewriteFilterEliminationKind;

/** 
 * Additional criterion by which a filter could eliminate a node from the graph that is explored.
 * **/
pub struct RewriteFilterCriterion/*<STRI : SimpleTermRewritingInterface>*/{
    // we might want to limit the number of times transformations of a specific kind are performed
    // to that end, we need to keep track of that number
    //pub transformations_kinds_occurrences : HashMap<STRI::TransformationKind, u32>
}

impl/*<STRI : SimpleTermRewritingInterface>*/ fmt::Display for RewriteFilterCriterion/*<STRI>*/ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

pub enum RewriteFilter/*<STRI : SimpleTermRewritingInterface>*/ {
    //FilterOnMaxRuleApplication(STRI::TransformationKind,u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl/*<STRI : SimpleTermRewritingInterface>*/ fmt::Display for RewriteFilter/*<STRI> */{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            /*RewriteFilter::FilterOnMaxRuleApplication(rule,num) => {
                write!(f,"MaxApp-{}={}",rule,num)
            },*/
            RewriteFilter::MaxProcessDepth(num) => {
                write!(f,"MaxDepth={}",num)
            },
            RewriteFilter::MaxNodeNumber(num) => {
                write!(f,"MaxNum={}",num)
            }
        }
    }
}

impl/*<STRI : SimpleTermRewritingInterface>*/ AbstractFilter<RewriteFilterCriterion/*<STRI>*/,RewriteFilterEliminationKind>  for RewriteFilter/*<STRI>*/ {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    _criterion: &RewriteFilterCriterion) -> Option<RewriteFilterEliminationKind> {
        match self {
            /*RewriteFilter::FilterOnMaxRuleApplication( tranfo_kind,num ) => {
                if let Some(applications_num) = criterion.transformations_kinds_occurrences.get(tranfo_kind) {
                    if applications_num > num {
                        return Some( RewriteFilterEliminationKind::ExceededNumberOfApplicationsOfASpecificRule );
                    }
                }
            },*/
            RewriteFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( RewriteFilterEliminationKind::MaxProcessDepth );
                }
            },
            RewriteFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( RewriteFilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        None
    }

}

