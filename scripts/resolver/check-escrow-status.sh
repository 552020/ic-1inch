#!/bin/bash

# Simple script for resolvers to check escrow status
# This is part of the MVP mechanical turk system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Fusion+ Mechanical Turk - Escrow Status Checker${NC}"
echo "=================================================="

# Check if order ID is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}❌ Error: Please provide an order ID${NC}"
    echo "Usage: $0 <order_id>"
    echo "Example: $0 fusion_1753986569703800000_nn52d-qid5y-wdg5t-k5q4b-pb554-u4zvb-eyiv2-46bum-xv5ok-57va7-7qe"
    exit 1
fi

ORDER_ID="$1"
echo -e "${BLUE}📋 Checking status for order: ${YELLOW}$ORDER_ID${NC}"
echo ""

# Check ICP escrow status
echo -e "${BLUE}🔍 Checking ICP Escrow Status...${NC}"
ICP_STATUS=$(dfx canister call escrow is_tokens_locked "(\"$ORDER_ID\")" 2>/dev/null | grep -o 'true\|false' || echo "error")

if [ "$ICP_STATUS" = "true" ]; then
    echo -e "  ${GREEN}✅ ICP tokens are locked${NC}"
elif [ "$ICP_STATUS" = "false" ]; then
    echo -e "  ${YELLOW}⚠️  ICP tokens are NOT locked${NC}"
else
    echo -e "  ${RED}❌ Error checking ICP escrow status${NC}"
fi

# Check ETH escrow status (placeholder for future implementation)
echo -e "${BLUE}🔍 Checking ETH Escrow Status...${NC}"
echo -e "  ${YELLOW}⚠️  ETH escrow status checking not yet implemented${NC}"
echo -e "  ${YELLOW}   (Will be implemented when ETH escrow contract is ready)${NC}"

echo ""
echo -e "${BLUE}📊 Summary:${NC}"

if [ "$ICP_STATUS" = "true" ]; then
    echo -e "  ${GREEN}✅ ICP: Tokens locked${NC}"
    echo -e "  ${YELLOW}⚠️  ETH: Status unknown (not implemented)${NC}"
    echo ""
    echo -e "${YELLOW}💡 Resolver Action Required:${NC}"
    echo -e "  - Check ETH escrow manually or wait for implementation"
    echo -e "  - If both chains show locked tokens, proceed with secret sharing"
elif [ "$ICP_STATUS" = "false" ]; then
    echo -e "  ${RED}❌ ICP: Tokens not locked${NC}"
    echo -e "  ${YELLOW}⚠️  ETH: Status unknown (not implemented)${NC}"
    echo ""
    echo -e "${RED}🚫 Cannot proceed: ICP tokens are not locked${NC}"
else
    echo -e "  ${RED}❌ Error checking status${NC}"
    echo ""
    echo -e "${RED}🚫 Cannot proceed: Error checking escrow status${NC}"
fi

echo ""
echo -e "${BLUE}📝 Next Steps:${NC}"
echo "1. For ICP → ETH orders: Check if ICP tokens are locked"
echo "2. For ETH → ICP orders: Check if ETH tokens are locked (manual for now)"
echo "3. If both are locked, proceed with cross-chain coordination"
echo "4. If not locked, wait for maker to complete token locking" 