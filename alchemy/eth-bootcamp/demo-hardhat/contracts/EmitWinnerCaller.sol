// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {EmitWinner} from "./EmitWinner.sol";

contract EmitWinnerCaller {
    function callAttempt() external {
        EmitWinner(0x633a61fa4a917686d995D81b222de2FA3E20aCd6).attempt();
    }
}
