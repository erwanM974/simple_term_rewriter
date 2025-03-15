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

use crate::core::terms::{position::PositionInLanguageTerm, term::LanguageTerm};

use super::lang::SimplisticBooleanLogicOperators;





#[test]
fn test_p1() {

    let term = LanguageTerm::new(
        SimplisticBooleanLogicOperators::AND,
        vec![
            LanguageTerm::new(
                SimplisticBooleanLogicOperators::NEG, 
                vec![
                    LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE,vec![])
                ]
            ),
            LanguageTerm::new(
                SimplisticBooleanLogicOperators::NEG, 
                vec![
                    LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE,vec![])
                ]
            )
        ]
    );

    let root = PositionInLanguageTerm::get_root_position();
    assert_eq!(
        term.get_sub_term_at_position(&root).unwrap().operator,
        SimplisticBooleanLogicOperators::AND
    );
    
    let p1 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p1).unwrap().operator,
        SimplisticBooleanLogicOperators::NEG
    );

    let p11 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0).get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p11).unwrap().operator,
        SimplisticBooleanLogicOperators::TRUE
    );

    let p2 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(1);
    assert_eq!(
        term.get_sub_term_at_position(&p2).unwrap().operator,
        SimplisticBooleanLogicOperators::NEG
    );

    let p21 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(1).get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p21).unwrap().operator,
        SimplisticBooleanLogicOperators::FALSE
    );

    assert_eq!(
        p1.get_parent_position().unwrap(),
        root
    );
    assert_eq!(
        p11.get_parent_position().unwrap(),
        p1
    );
    assert_eq!(
        p2.get_parent_position().unwrap(),
        root
    );
    assert_eq!(
        p21.get_parent_position().unwrap(),
        p2
    );

}


#[test]
fn test_p2() {

    let term = LanguageTerm::new(
        SimplisticBooleanLogicOperators::NEG,
        vec![
            LanguageTerm::new(
                SimplisticBooleanLogicOperators::OR, 
                vec![
                    LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE,vec![]),
                    LanguageTerm::new(SimplisticBooleanLogicOperators::AND,
                        vec![
                            LanguageTerm::new(SimplisticBooleanLogicOperators::FALSE,vec![]),
                            LanguageTerm::new(SimplisticBooleanLogicOperators::TRUE,vec![]),
                        ]
                    )
                ]
            )
        ]
    );

    let root = PositionInLanguageTerm::get_root_position();
    assert_eq!(
        term.get_sub_term_at_position(&root).unwrap().operator,
        SimplisticBooleanLogicOperators::NEG
    );
    
    let p1 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p1).unwrap().operator,
        SimplisticBooleanLogicOperators::OR
    );

    let p11 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0).get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p11).unwrap().operator,
        SimplisticBooleanLogicOperators::TRUE
    );

    let p12 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0).get_position_of_nth_child(1);
    assert_eq!(
        term.get_sub_term_at_position(&p12).unwrap().operator,
        SimplisticBooleanLogicOperators::AND
    );

    let p121 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0).get_position_of_nth_child(1).get_position_of_nth_child(0);
    assert_eq!(
        term.get_sub_term_at_position(&p121).unwrap().operator,
        SimplisticBooleanLogicOperators::FALSE
    );

    let p122 = PositionInLanguageTerm::get_root_position().get_position_of_nth_child(0).get_position_of_nth_child(1).get_position_of_nth_child(1);
    assert_eq!(
        term.get_sub_term_at_position(&p122).unwrap().operator,
        SimplisticBooleanLogicOperators::TRUE
    );

    assert_eq!(
        p1.get_parent_position().unwrap(),
        root
    );
    assert_eq!(
        p11.get_parent_position().unwrap(),
        p1
    );
    assert_eq!(
        p12.get_parent_position().unwrap(),
        p1
    );
    assert_eq!(
        p121.get_parent_position().unwrap(),
        p12
    );
    assert_eq!(
        p122.get_parent_position().unwrap(),
        p12
    );

}

