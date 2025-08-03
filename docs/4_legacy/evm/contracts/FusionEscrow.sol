// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/**
 * @title FusionEscrow
 * @dev Cross-chain atomic swap escrow contract for ICP-ETH swaps
 * @dev This is the Ethereum side of the mechanical turk fusion+ implementation
 */
contract FusionEscrow is ReentrancyGuard, Ownable {
  using ECDSA for bytes32;

  // Events
  event ETHLocked(
    string indexed orderId,
    address indexed maker,
    uint256 amount,
    bytes32 hashlock
  );
  event ETHClaimed(
    string indexed orderId,
    address indexed resolver,
    uint256 amount
  );
  event ETHRefunded(
    string indexed orderId,
    address indexed maker,
    uint256 amount
  );
  event ResolverAuthorized(address indexed resolver);
  event ResolverDeauthorized(address indexed resolver);

  // Structs
  struct EscrowData {
    address maker;
    address resolver;
    uint256 amount;
    bytes32 hashlock;
    uint256 timelock;
    bool claimed;
    bool refunded;
  }

  // State variables
  mapping(string => EscrowData) public escrows;
  mapping(address => bool) public authorizedResolvers;

  // Constants
  uint256 public constant MIN_LOCK_TIME = 1 hours;
  uint256 public constant MAX_LOCK_TIME = 24 hours;

  // Modifiers
  modifier onlyAuthorizedResolver() {
    require(
      authorizedResolvers[msg.sender],
      "FusionEscrow: not authorized resolver"
    );
    _;
  }

  modifier onlyEscrowMaker(string memory orderId) {
    require(
      escrows[orderId].maker == msg.sender,
      "FusionEscrow: not escrow maker"
    );
    _;
  }

  modifier escrowExists(string memory orderId) {
    require(
      escrows[orderId].maker != address(0),
      "FusionEscrow: escrow does not exist"
    );
    _;
  }

  modifier escrowNotClaimed(string memory orderId) {
    require(!escrows[orderId].claimed, "FusionEscrow: escrow already claimed");
    _;
  }

  modifier escrowNotRefunded(string memory orderId) {
    require(
      !escrows[orderId].refunded,
      "FusionEscrow: escrow already refunded"
    );
    _;
  }

  constructor() Ownable(msg.sender) {}

  /**
   * @dev Lock ETH for a cross-chain swap (mechanical turk version - simplified)
   * @param orderId Unique identifier for the swap order
   * @param timelock Unix timestamp when the escrow can be refunded
   */
  function lockETHForSwap(
    string memory orderId,
    uint256 timelock
  ) external payable nonReentrant {
    require(msg.value > 0, "FusionEscrow: must send ETH");
    require(
      escrows[orderId].maker == address(0),
      "FusionEscrow: order already exists"
    );
    require(
      timelock > block.timestamp + MIN_LOCK_TIME,
      "FusionEscrow: timelock too short"
    );
    require(
      timelock < block.timestamp + MAX_LOCK_TIME,
      "FusionEscrow: timelock too long"
    );

    escrows[orderId] = EscrowData({
      maker: msg.sender,
      resolver: address(0),
      amount: msg.value,
      hashlock: bytes32(0), // Not used in mechanical turk version
      timelock: timelock,
      claimed: false,
      refunded: false
    });

    emit ETHLocked(orderId, msg.sender, msg.value, bytes32(0));
  }

  /**
   * @dev Claim locked ETH (mechanical turk version - simplified)
   * @param orderId Unique identifier for the swap order
   * @param icpReceipt Receipt from ICP side transfer (for verification)
   */
  function claimLockedETH(
    string memory orderId,
    string memory icpReceipt
  )
    external
    onlyAuthorizedResolver
    escrowExists(orderId)
    escrowNotClaimed(orderId)
    escrowNotRefunded(orderId)
    nonReentrant
  {
    EscrowData storage escrow = escrows[orderId];

    require(
      escrow.timelock > block.timestamp,
      "FusionEscrow: timelock expired"
    );

    // For mechanical turk version, we'll accept the receipt as-is
    // In production, this would verify the ICP receipt cryptographically
    require(bytes(icpReceipt).length > 0, "FusionEscrow: invalid ICP receipt");

    escrow.claimed = true;
    escrow.resolver = msg.sender;

    (bool success, ) = msg.sender.call{value: escrow.amount}("");
    require(success, "FusionEscrow: ETH transfer failed");

    emit ETHClaimed(orderId, msg.sender, escrow.amount);
  }

  /**
   * @dev Refund locked ETH after timelock expires
   * @param orderId Unique identifier for the swap order
   */
  function refundLockedETH(
    string memory orderId
  )
    external
    escrowExists(orderId)
    escrowNotClaimed(orderId)
    escrowNotRefunded(orderId)
    nonReentrant
  {
    EscrowData storage escrow = escrows[orderId];

    require(
      escrow.timelock <= block.timestamp,
      "FusionEscrow: timelock not expired"
    );
    require(msg.sender == escrow.maker, "FusionEscrow: only maker can refund");

    escrow.refunded = true;

    (bool success, ) = escrow.maker.call{value: escrow.amount}("");
    require(success, "FusionEscrow: ETH transfer failed");

    emit ETHRefunded(orderId, escrow.maker, escrow.amount);
  }

  /**
   * @dev Authorize a resolver to claim escrows
   * @param resolver Address of the resolver to authorize
   */
  function authorizeResolver(address resolver) external onlyOwner {
    require(resolver != address(0), "FusionEscrow: invalid resolver address");
    authorizedResolvers[resolver] = true;
    emit ResolverAuthorized(resolver);
  }

  /**
   * @dev Deauthorize a resolver
   * @param resolver Address of the resolver to deauthorize
   */
  function deauthorizeResolver(address resolver) external onlyOwner {
    authorizedResolvers[resolver] = false;
    emit ResolverDeauthorized(resolver);
  }

  /**
   * @dev Get escrow data
   * @param orderId Unique identifier for the swap order
   */
  function getEscrowData(
    string memory orderId
  ) external view returns (EscrowData memory) {
    return escrows[orderId];
  }

  /**
   * @dev Check if an escrow exists
   * @param orderId Unique identifier for the swap order
   */
  function escrowExistsCheck(
    string memory orderId
  ) external view returns (bool) {
    return escrows[orderId].maker != address(0);
  }

  /**
   * @dev Emergency function to withdraw stuck ETH (only owner)
   */
  function emergencyWithdraw() external onlyOwner {
    (bool success, ) = owner().call{value: address(this).balance}("");
    require(success, "FusionEscrow: emergency withdrawal failed");
  }
}
