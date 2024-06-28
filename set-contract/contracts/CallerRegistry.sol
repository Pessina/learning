// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract CallerRegistry {
    // Data struct to associate a string key with a string value
    mapping(string => string) private callerData;

    // Function to set the caller data
    function setCallerData(string memory key, string memory value) public {
        callerData[key] = value;
    }

    // Function to view the caller data
    function viewCallerData(string memory key) public view returns (string memory) {
        return callerData[key];
    }
}
