// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract CallerRegistry {
    // Data struct to associate caller ID with a boolean value
    mapping(address => bool) private callerStatus;

    // Function to set the caller status
    function setCallerStatus(bool status) public {
        callerStatus[msg.sender] = status;
    }

    // Function to view the caller status
    function viewCallerStatus(address caller) public view returns (bool) {
        return callerStatus[caller];
    }
}
        

    