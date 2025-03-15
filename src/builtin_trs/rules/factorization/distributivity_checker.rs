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



use crate::core::terms::term::RewritableLanguageOperatorSymbol;



pub trait DistributivityChecker<LOS : RewritableLanguageOperatorSymbol> {

    fn is_binary(&self, op : &LOS) -> bool;

    fn is_associative(&self, op : &LOS) -> bool;

    fn is_commutative(&self, op : &LOS) -> bool;

    /**
    OP1 is left distributive over OP2 iff for any x, y and z:
    OP1(x,OP2(y,z)) = OP2(OP1(x,y),OP1(x,z))

    Example:
    Multiplication is left distributive over addition:
    "*(2,+(1,3)) = +(*(2,1),*(2,3))"
     **/
    fn is_left_distributive_over(&self, op1 : &LOS, op2 : &LOS) -> bool;

    /**
    OP1 is right distributive over OP2 iff for any x, y and z:
    OP1(OP2(y,z),x) = OP2(OP1(y,x),OP1(z,x))

    Example:
    Multiplication is right distributive over addition:
    "*(+(1,3),2) = +(*(1,2),*(3,2))"
     **/
    fn is_right_distributive_over(&self, op1 : &LOS, op2 : &LOS) -> bool;

    /** 
     * When factorizing OP1(OP2(x,y),x) -> OP2(x,OP1(y,0))
     * We need to know the "0" empty operator symbol.
     * **/
    fn get_empty_operation_symbol(&self) -> LOS;

}

