pragma solidity ^0.4.25;


contract Ownable {

    address public owner;

    modifier onlyOwner {
        require(msg.sender == owner);
        _;
    }
}
