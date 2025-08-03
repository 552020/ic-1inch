use crate::types::{CrossChainOrderDto, FusionError};

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Helper function to validate 1inch order parameters
pub fn validate_order_parameters(order: &CrossChainOrderDto) -> Result<(), FusionError> {
    // Validate salt
    if order.salt.is_empty() {
        return Err(FusionError::InvalidSalt);
    }

    // Validate addresses (must be valid Ethereum addresses)
    if !is_valid_eth_address(&order.maker) {
        return Err(FusionError::TokenAddressInvalid);
    }

    if !is_valid_eth_address(&order.receiver) {
        return Err(FusionError::TokenAddressInvalid);
    }

    if !is_valid_eth_address(&order.maker_asset) {
        return Err(FusionError::TokenAddressInvalid);
    }

    if !is_valid_eth_address(&order.taker_asset) {
        return Err(FusionError::TokenAddressInvalid);
    }

    // Validate amounts (must be parseable as numbers)
    if order.making_amount.parse::<u64>().is_err() {
        return Err(FusionError::InvalidAmount);
    }

    if order.taking_amount.parse::<u64>().is_err() {
        return Err(FusionError::InvalidAmount);
    }

    // Check that amounts are not zero
    if order.making_amount == "0" || order.taking_amount == "0" {
        return Err(FusionError::InvalidAmount);
    }

    Ok(())
}

/// Validate Ethereum address format
pub fn is_valid_eth_address(address: &str) -> bool {
    address.starts_with("0x")
        && address.len() == 42
        && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

// ============================================================================
// HASH GENERATION HELPERS
// ============================================================================

/// Generate deterministic order hash from order data
pub fn generate_order_hash(
    order: &CrossChainOrderDto,
    src_chain_id: u64,
    signature: &str,
) -> String {
    use sha2::{Digest, Sha256};

    let hash_input = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        order.salt,
        order.maker,
        order.receiver,
        order.maker_asset,
        order.taker_asset,
        order.making_amount,
        order.taking_amount,
        order.maker_traits,
        src_chain_id,
        signature
    );

    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let result = hasher.finalize();

    format!("0x{}", hex::encode(result))
}
