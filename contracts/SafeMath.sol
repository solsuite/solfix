pragma solidity ^0.4.25;


library SafeMath {

    function add(
        uint256 a,
        uint256 b
    )
        internal
        pure
        returns (uint256)
    {
        uint256 c = a + b;
        require(c >= a, "SafeMath: Integer Overflow on addition");
        return c;
    }

    function sub(
        uint256 a,
        uint256 b
    )
        internal
        pure
        returns (uint256)
    {
        require(a >= b, "SafeMath: Integer Underflow on subtraction");
        return a - b;
    }

    function mul(
        uint256 a,
        uint256 b
    )
        internal
        pure
        returns (uint256)
    {
        if (a == 0) {
            return 0;
        }
        uint256 c = a * b;
        require(c / a == b, "SafeMath: Integer Overflow on multiplication");
        return c;
    }

    function div(
        uint256 a,
        uint256 b
    )
        internal
        pure
        returns (uint256)
    {
        return a / b;
    }

    function mod(
        uint256 a,
        uint256 b
    )
        internal
        pure
        returns (uint256)
    {
        require(b != 0, "SafeMath: Modulus by zero");
        return a % b;
    }
}
