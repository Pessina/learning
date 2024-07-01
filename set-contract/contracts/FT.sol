pragma solidity ^0.8.26;

import "./FTInterface.sol";

contract EIP20 is EIP20Interface {

    uint256 constant private MAX_UINT256 = type(uint256).max;
    mapping (address => uint256) public balances;
    mapping (address => mapping (address => uint256)) public allowed;
    string public name;
    uint8 public decimals;
    string public symbol;
    uint256 private _totalSupply;

    constructor(
        uint256 _initialAmount,
        string memory _tokenName,
        uint8 _decimalUnits,
        string memory _tokenSymbol
    ) {
        balances[msg.sender] = _initialAmount;
        _totalSupply = _initialAmount;
        name = _tokenName;
        decimals = _decimalUnits;
        symbol = _tokenSymbol;
    }

    function totalSupply() external view override returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address _owner) public view override returns (uint256 balance) {
        return balances[_owner];
    }

    function transfer(address _to, uint256 _value) public override returns (bool success) {
        require(balances[msg.sender] >= _value, "Insufficient balance");
        unchecked {
            balances[msg.sender] -= _value;
            balances[_to] += _value;
        }
        emit Transfer(msg.sender, _to, _value);
        return true;
    }

    function transferFrom(address _from, address _to, uint256 _value) public override returns (bool success) {
        uint256 currentAllowance = allowed[_from][msg.sender];
        require(balances[_from] >= _value && currentAllowance >= _value, "Insufficient balance or allowance");
        unchecked {
            balances[_to] += _value;
            balances[_from] -= _value;
            if (currentAllowance < MAX_UINT256) {
                allowed[_from][msg.sender] -= _value;
            }
        }
        emit Transfer(_from, _to, _value);
        return true;
    }

    function approve(address _spender, uint256 _value) public override returns (bool success) {
        allowed[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    function allowance(address _owner, address _spender) public view override returns (uint256 remaining) {
        return allowed[_owner][_spender];
    }

    function mint(address _to, uint256 _amount) external override returns (bool success) {
        require(_to != address(0), "ERC20: mint to the zero address");
        _totalSupply += _amount;
        unchecked {
            balances[_to] += _amount;
        }
        emit Transfer(address(0), _to, _amount);
        return true;
    }
}
