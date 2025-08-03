// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title TestICP
 * @dev Mock ICP token for testing cross-chain swaps
 */
contract TestICP is ERC20, Ownable {
  constructor() ERC20("Test ICP", "tICP") Ownable(msg.sender) {}

  /**
   * @dev Mint test tokens (only owner for testing)
   */
  function mint(address to, uint256 amount) external onlyOwner {
    _mint(to, amount);
  }

  /**
   * @dev Burn tokens
   */
  function burn(uint256 amount) external {
    _burn(msg.sender, amount);
  }
}

/**
 * @title TestETH
 * @dev Mock ETH token for testing cross-chain swaps
 */
contract TestETH is ERC20, Ownable {
  constructor() ERC20("Test ETH", "tETH") Ownable(msg.sender) {}

  /**
   * @dev Mint test tokens (only owner for testing)
   */
  function mint(address to, uint256 amount) external onlyOwner {
    _mint(to, amount);
  }

  /**
   * @dev Burn tokens
   */
  function burn(uint256 amount) external {
    _burn(msg.sender, amount);
  }
}
