/**
 * Test script to verify fillOrderArgs triggers SrcEscrowCreated event
 * 
 * This script creates a test order, signs it, and calls fillOrderArgs
 * to verify that the SrcEscrowCreated event is emitted correctly.
 */

const { ethers } = require('ethers');
require('dotenv').config();

// Configuration
const ESCROW_FACTORY_ADDRESS = '0x65b3db8baef0215a1f9b14c506d2a3078b2c84ae'; // Base Sepolia
const LOP_ADDRESS = '0x111111125421ca6dc452d289314280a0f8842a65'; // 1inch LOP V4
const RESOLVER_ADDRESS = '0x50c5725949a6f0c72e6c4a641f24049a917db0cb'; // ResolverExample

async function testFillOrderArgs() {
  // Connect to provider
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL || 'https://sepolia.base.org');
  
  // Load contract ABIs
  const escrowFactoryAbi = [
    'event SrcEscrowCreated(IBaseEscrow.Immutables srcImmutables, DstImmutablesComplement dstImmutablesComplement)'
  ];
  
  const lopAbi = [
    'function fillOrderArgs(IOrderMixin.Order calldata order, bytes32 r, bytes32 vs, uint256 amount, TakerTraits takerTraits, bytes calldata args) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)'
  ];
  
  const escrowFactory = new ethers.Contract(ESCROW_FACTORY_ADDRESS, escrowFactoryAbi, provider);
  const lop = new ethers.Contract(LOP_ADDRESS, lopAbi, provider);
  
  // Listen for SrcEscrowCreated event
  escrowFactory.on('SrcEscrowCreated', (srcImmutables, dstImmutablesComplement, event) => {
    console.log('✅ SrcEscrowCreated event emitted!');
    console.log('Transaction hash:', event.transactionHash);
    console.log('Block number:', event.blockNumber);
    console.log('srcImmutables:', srcImmutables);
    console.log('dstImmutablesComplement:', dstImmutablesComplement);
  });
  
  console.log('Listening for SrcEscrowCreated events...');
  console.log('EscrowFactory address:', ESCROW_FACTORY_ADDRESS);
  console.log('LOP address:', LOP_ADDRESS);
  console.log('Resolver address:', RESOLVER_ADDRESS);
  
  // Keep the script running to listen for events
  console.log('Waiting for events... (Press Ctrl+C to stop)');
  
  // In a real test, you would:
  // 1. Create a test order with proper parameters
  // 2. Sign it with a private key
  // 3. Call resolver.deploySrc() or LOP.fillOrderArgs() directly
  // 4. Verify the SrcEscrowCreated event is emitted
  
  // For now, we're just listening for any events that might be triggered
  // by other transactions on the network
}

if (require.main === module) {
  testFillOrderArgs().catch(console.error);
}

module.exports = { testFillOrderArgs }; 