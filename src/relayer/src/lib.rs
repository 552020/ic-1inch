mod helpers;
mod memory;
mod types;

use types::{CrossChainOrderDto, FusionError, Order, OrderStatus};

// ============================================================================
// 1INCH FUSION+ API ENDPOINTS (Our Bible)
// ============================================================================

/// Submit order - matches 1inch /fusion-plus/relayer/v1.0/submit
#[ic_cdk::update]
async fn fusion_plus_relayer_submit(
    order: CrossChainOrderDto,
    src_chain_id: u64,
    signature: String,
    extension: String,
    quote_id: String,
    secret_hashes: Vec<String>,
) -> Result<String, FusionError> {
    let caller = ic_cdk::caller();

    // Validate order parameters
    helpers::validate_order_parameters(&order)?;

    // Validate signature format
    if signature.is_empty() {
        return Err(FusionError::InvalidEIP712Signature);
    }

    // Validate secret hashes
    if secret_hashes.is_empty() {
        return Err(FusionError::InvalidSecretHash);
    }

    for hash in &secret_hashes {
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(FusionError::InvalidSecretHash);
        }
    }

    // Generate unique order ID (hash)
    let order_id = helpers::generate_order_hash(&order, src_chain_id, &signature);

    // Create internal order structure
    let internal_order = Order::new(
        order_id.clone(),
        order.maker.clone(),
        caller, // ICP principal of the maker
        order.salt.clone(),
        order.maker_asset.clone(),
        order.taker_asset.clone(),
        order.making_amount.clone(),
        order.taking_amount.clone(),
        secret_hashes[0].clone(), // Primary hashlock
        signature,
        quote_id,
        extension,
        src_chain_id,
        1, // dst_chain_id (ICP = 1 for now)
    );

    // Store the order
    memory::store_order(internal_order)?;

    // Log order creation
    ic_cdk::println!("📋 Order submitted: {}", order_id);

    Ok(order_id)
}

/// Get active orders - matches 1inch /fusion-plus/orders/v1.0/order/active
#[ic_cdk::query]
fn fusion_plus_orders_active() -> Vec<Order> {
    memory::get_active_orders()
}

/// Get order status - matches 1inch /fusion-plus/orders/v1.0/order/status/{orderHash}
#[ic_cdk::query]
fn fusion_plus_order_status(order_hash: String) -> Result<Order, FusionError> {
    memory::get_order(&order_hash)
}

/// Get order escrow - matches 1inch /fusion-plus/orders/v1.0/order/escrow
#[ic_cdk::query]
fn fusion_plus_order_escrow(order_hash: String, chain_id: u64) -> Result<Order, FusionError> {
    let order = memory::get_order(&order_hash)?;

    // Filter by chain ID if needed
    if order.src_chain_id != chain_id && order.dst_chain_id != chain_id {
        return Err(FusionError::OrderNotFound);
    }

    Ok(order)
}

/// Get order secrets - matches 1inch /fusion-plus/orders/v1.0/order/secrets/{orderHash}
#[ic_cdk::query]
fn fusion_plus_order_secrets(order_hash: String) -> Result<Vec<String>, FusionError> {
    let order = memory::get_order(&order_hash)?;
    Ok(order.secret_hashes)
}

/// Get ready-to-accept secret fills - matches 1inch /fusion-plus/orders/v1.0/order/ready-to-accept-secret-fills/{orderHash}
#[ic_cdk::query]
fn fusion_plus_order_ready_to_accept_secret_fills(order_hash: String) -> Result<bool, FusionError> {
    let order = memory::get_order(&order_hash)?;

    // Check if order is in a state where it can accept secret fills
    let ready = matches!(order.status, OrderStatus::Accepted) && !order.secret_hashes.is_empty();

    Ok(ready)
}

// ============================================================================
// CANISTER LIFECYCLE
// ============================================================================

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let (orders, identities) = memory::serialize_relayer_state();
    ic_cdk::storage::stable_save((orders, identities)).expect("Failed to save state");
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (orders, identities): (Vec<(String, Order)>, Vec<(String, String)>) =
        ic_cdk::storage::stable_restore().expect("Failed to restore state");
    memory::deserialize_relayer_state(orders, identities);
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::helpers::{generate_order_hash, is_valid_eth_address, validate_order_parameters};
    use crate::types::{CrossChainOrderDto, FusionError};

    fn create_test_order() -> CrossChainOrderDto {
        CrossChainOrderDto {
            salt: "42".to_string(),
            maker: "0x1234567890123456789012345678901234567890".to_string(),
            receiver: "0x1234567890123456789012345678901234567890".to_string(),
            maker_asset: "0x0000000000000000000000000000000000000001".to_string(),
            taker_asset: "0x0000000000000000000000000000000000000002".to_string(),
            making_amount: "1000000000000000000".to_string(), // 1 ETH in wei (smaller number)
            taking_amount: "2000000000000000000".to_string(), // 2 ETH in wei (smaller number)
            maker_traits: "0x".to_string(),
        }
    }

    #[test]
    fn test_is_valid_eth_address() {
        // Valid addresses (42 chars: 0x + 40 hex chars)
        assert!(is_valid_eth_address("0x1234567890123456789012345678901234567890"));
        assert!(is_valid_eth_address("0x0000000000000000000000000000000000000000"));
        assert!(is_valid_eth_address("0xabcdefABCDEF1234567890123456789012345678"));

        // Invalid addresses
        assert!(!is_valid_eth_address("1234567890123456789012345678901234567890")); // No 0x prefix
        assert!(!is_valid_eth_address("0x123")); // Too short
        assert!(!is_valid_eth_address("0x12345678901234567890123456789012345678901")); // Too long
        assert!(!is_valid_eth_address("0x123456789012345678901234567890123456789G")); // Invalid hex char
        assert!(!is_valid_eth_address("")); // Empty string
    }

    #[test]
    fn test_validate_order_parameters_valid() {
        let order = create_test_order();
        match validate_order_parameters(&order) {
            Ok(()) => (),
            Err(e) => panic!("Expected validation to succeed but got error: {:?}", e),
        }
    }

    #[test]
    fn test_validate_order_parameters_empty_salt() {
        let mut order = create_test_order();
        order.salt = "".to_string();

        match validate_order_parameters(&order) {
            Err(FusionError::InvalidSalt) => (),
            _ => panic!("Expected InvalidSalt error"),
        }
    }

    #[test]
    fn test_validate_order_parameters_invalid_maker_address() {
        let mut order = create_test_order();
        order.maker = "invalid_address".to_string();

        match validate_order_parameters(&order) {
            Err(FusionError::TokenAddressInvalid) => (),
            _ => panic!("Expected TokenAddressInvalid error"),
        }
    }

    #[test]
    fn test_validate_order_parameters_zero_amount() {
        let mut order = create_test_order();
        order.making_amount = "0".to_string();

        match validate_order_parameters(&order) {
            Err(FusionError::InvalidAmount) => (),
            _ => panic!("Expected InvalidAmount error"),
        }
    }

    #[test]
    fn test_validate_order_parameters_invalid_amount_format() {
        let mut order = create_test_order();
        order.making_amount = "not_a_number".to_string();

        match validate_order_parameters(&order) {
            Err(FusionError::InvalidAmount) => (),
            _ => panic!("Expected InvalidAmount error"),
        }
    }

    #[test]
    fn test_generate_order_hash() {
        let order = create_test_order();
        let src_chain_id = 1;
        let signature = "0x1234567890abcdef";

        let hash1 = generate_order_hash(&order, src_chain_id, signature);
        let hash2 = generate_order_hash(&order, src_chain_id, signature);

        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should start with 0x and be 66 characters (0x + 64 hex chars)
        assert!(hash1.starts_with("0x"));
        assert_eq!(hash1.len(), 66);

        // Different signature should produce different hash
        let hash3 = generate_order_hash(&order, src_chain_id, "0xdifferent");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_generate_order_hash_different_inputs() {
        let order1 = create_test_order();
        let mut order2 = create_test_order();
        order2.salt = "43".to_string(); // Different salt

        let hash1 = generate_order_hash(&order1, 1, "0x123");
        let hash2 = generate_order_hash(&order2, 1, "0x123");

        // Different order data should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
