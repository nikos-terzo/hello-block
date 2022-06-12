// SPDX-License-Identifier: MIT

pragma solidity ^0.8.14;

contract Chatter {
    address owner;
    mapping (address => string) hangingMessages;
    event HasMessage(address from);
    constructor() {
        owner = msg.sender;
    }

    function messageMe(string calldata text) public {
        hangingMessages[msg.sender] = string(abi.encodePacked(hangingMessages[msg.sender], "\n", text));
        emit HasMessage(msg.sender);
    }

    function getMessages(address from) public returns (string memory) {
        require(msg.sender == owner);
        string memory ret = hangingMessages[from];
        hangingMessages[from] = ""; // reset messages stored inside contract.
        return ret;
    }
}