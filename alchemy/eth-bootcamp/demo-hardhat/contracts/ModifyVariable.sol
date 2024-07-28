// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.24;

contract ModifyVariable {
    uint public value;

    constructor(uint _value) {
        value = _value;
    }

    function modifyToLeet() public {
        value = 1337;
    }
}
