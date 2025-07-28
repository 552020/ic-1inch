use crate::memory::{clear_limit_order_data, generate_order_id};
use crate::mock_icrc1_token::{cleanup_test_tokens, setup_test_tokens, Account, TransferArgs};
use crate::types::{CreateOrderParams, Order, OrderError, OrderId, SystemStats};
use candid::Principal;
use std::str::FromStr;

/// Test utilities for Limit Order Protocol MVP
///
/// Uses ICRC-1 mock tokens for MVP testing.
/// Future: Will need ICRC-2 utilities for ChainFusion+ resolver testing.

/// Test data generators and fixtures
pub struct OrderTestFixtures;

impl OrderTestFixtures {
    /// Generate test principals (using valid principals)
    pub fn test_principals() -> (Principal, Principal) {
        let maker = Principal::management_canister(); // Valid management canister principal
        let taker = Principal::from_slice(&[0, 0, 0, 0, 1, 1, 1, 1]).unwrap(); // Valid generated principal
        (maker, taker)
    }

    /// Generate test token canisters (using valid principals that are different)
    pub fn test_token_canisters() -> (Principal, Principal) {
        let token_a = Principal::management_canister(); // Management canister as token A
        let token_b = Principal::from_slice(&[1, 1, 1, 1, 0, 0, 0, 0]).unwrap(); // Different valid principal for token B
        (token_a, token_b)
    }

    /// Create a basic test order
    pub fn create_basic_order() -> Order {
        let (maker, taker) = Self::test_principals();
        let (token_a, token_b) = Self::test_token_canisters();
        let current_time = ic_cdk::api::time();

        Order {
            id: 1,
            maker,
            receiver: maker,
            maker_asset: token_a,
            taker_asset: token_b,
            making_amount: 1_000_000,                    // 1 TTA
            taking_amount: 2_000_000,                    // 2 TTB
            expiration: current_time + 3600_000_000_000, // 1 hour from now
            created_at: current_time,
            allowed_taker: None,
            metadata: None,
        }
    }

    /// Create a private order (with allowed_taker)
    pub fn create_private_order() -> Order {
        let mut order = Self::create_basic_order();
        let (_, taker) = Self::test_principals();
        order.allowed_taker = Some(taker);
        order.id = 2;
        order
    }

    /// Create an expired order
    pub fn create_expired_order() -> Order {
        let mut order = Self::create_basic_order();
        let current_time = ic_cdk::api::time();
        order.expiration = current_time - 3600_000_000_000; // 1 hour ago
        order.id = 3;
        order
    }

    /// Create an order with large amounts
    pub fn create_large_amount_order() -> Order {
        let mut order = Self::create_basic_order();
        order.making_amount = 1_000_000_000_000; // 1M tokens
        order.taking_amount = 2_000_000_000_000; // 2M tokens
        order.id = 4;
        order
    }

    /// Create CreateOrderParams for testing
    pub fn create_order_params() -> CreateOrderParams {
        let (maker, _) = Self::test_principals();
        let (token_a, token_b) = Self::test_token_canisters();
        let current_time = ic_cdk::api::time();

        CreateOrderParams {
            receiver: maker,
            maker_asset: token_a,
            taker_asset: token_b,
            making_amount: 1_000_000,
            taking_amount: 2_000_000,
            expiration: current_time + 3600_000_000_000, // 1 hour from now
            allowed_taker: None,
        }
    }

    /// Create invalid CreateOrderParams for error testing
    pub fn create_invalid_order_params() -> Vec<CreateOrderParams> {
        let (maker, _) = Self::test_principals();
        let (token_a, token_b) = Self::test_token_canisters();
        let current_time = ic_cdk::api::time();

        vec![
            // Invalid amount (zero)
            CreateOrderParams {
                receiver: maker,
                maker_asset: token_a,
                taker_asset: token_b,
                making_amount: 0,
                taking_amount: 2_000_000,
                expiration: current_time + 3600_000_000_000,
                allowed_taker: None,
            },
            // Invalid expiration (past)
            CreateOrderParams {
                receiver: maker,
                maker_asset: token_a,
                taker_asset: token_b,
                making_amount: 1_000_000,
                taking_amount: 2_000_000,
                expiration: current_time - 3600_000_000_000, // 1 hour ago
                allowed_taker: None,
            },
            // Invalid asset pair (same token)
            CreateOrderParams {
                receiver: maker,
                maker_asset: token_a,
                taker_asset: token_a, // Same as maker_asset
                making_amount: 1_000_000,
                taking_amount: 2_000_000,
                expiration: current_time + 3600_000_000_000,
                allowed_taker: None,
            },
        ]
    }
}

/// Test environment setup and cleanup
pub struct TestEnvironment;

impl TestEnvironment {
    /// Setup complete test environment with ICRC-1 mock tokens
    pub fn setup() -> TestContext {
        // Clear existing data
        Self::cleanup();

        let (maker, taker) = OrderTestFixtures::test_principals();

        // Setup ICRC-1 mock tokens with initial balances for MVP testing
        setup_test_tokens(
            maker,
            taker,
            10_000_000_000, // 10k TTA (ICRC-1) for maker
            20_000_000_000, // 20k TTB (ICRC-1) for taker
        );

        TestContext {
            maker,
            taker,
            setup_complete: true,
        }
    }

    /// Cleanup test environment
    pub fn cleanup() {
        cleanup_test_tokens();
        #[cfg(test)]
        clear_limit_order_data();
    }

    /// Create test scenario with multiple orders
    pub fn setup_multi_order_scenario() -> Vec<Order> {
        Self::setup();

        vec![
            OrderTestFixtures::create_basic_order(),
            OrderTestFixtures::create_private_order(),
            OrderTestFixtures::create_large_amount_order(),
        ]
    }
}

/// Test context with useful data
pub struct TestContext {
    pub maker: Principal,
    pub taker: Principal,
    pub setup_complete: bool,
}

impl TestContext {
    /// Get test token canisters
    pub fn token_canisters(&self) -> (Principal, Principal) {
        OrderTestFixtures::test_token_canisters()
    }

    /// Create order with this context
    pub fn create_order_params(&self) -> CreateOrderParams {
        let (token_a, token_b) = self.token_canisters();
        let current_time = ic_cdk::api::time();

        CreateOrderParams {
            receiver: self.maker,
            maker_asset: token_a,
            taker_asset: token_b,
            making_amount: 1_000_000,
            taking_amount: 2_000_000,
            expiration: current_time + 3600_000_000_000,
            allowed_taker: None,
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        if self.setup_complete {
            TestEnvironment::cleanup();
        }
    }
}

/// Assertion helpers for testing
pub struct OrderAssertions;

impl OrderAssertions {
    /// Assert order has correct initial state
    pub fn assert_valid_order(order: &Order) {
        assert!(order.making_amount > 0, "Making amount should be positive");
        assert!(order.taking_amount > 0, "Taking amount should be positive");
        assert_ne!(
            order.maker_asset, order.taker_asset,
            "Assets should be different"
        );
        assert!(
            order.expiration > ic_cdk::api::time(),
            "Order should not be expired"
        );
    }

    /// Assert order validation error
    pub fn assert_order_error(result: Result<OrderId, OrderError>, expected_error: OrderError) {
        match result {
            Err(actual_error) => {
                assert_eq!(
                    std::mem::discriminant(&actual_error),
                    std::mem::discriminant(&expected_error),
                    "Expected error type mismatch"
                );
            }
            Ok(_) => panic!("Expected error but got success"),
        }
    }

    /// Assert system stats are updated correctly
    pub fn assert_stats_updated(
        stats_before: &SystemStats,
        stats_after: &SystemStats,
        operation: &str,
    ) {
        match operation {
            "create" => {
                assert_eq!(
                    stats_after.orders_created,
                    stats_before.orders_created + 1,
                    "Orders created should increase by 1"
                );
            }
            "fill" => {
                assert_eq!(
                    stats_after.orders_filled,
                    stats_before.orders_filled + 1,
                    "Orders filled should increase by 1"
                );
            }
            "cancel" => {
                assert_eq!(
                    stats_after.orders_cancelled,
                    stats_before.orders_cancelled + 1,
                    "Orders cancelled should increase by 1"
                );
            }
            _ => panic!("Unknown operation for stats assertion"),
        }
    }
}

/// Performance testing utilities
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, u64)
    where
        F: FnOnce() -> R,
    {
        let start = ic_cdk::api::time();
        let result = f();
        let end = ic_cdk::api::time();
        (result, end - start)
    }

    /// Create multiple orders for load testing
    pub fn create_test_orders(count: usize) -> Vec<Order> {
        let mut orders = Vec::new();
        let base_order = OrderTestFixtures::create_basic_order();

        for i in 0..count {
            let mut order = base_order.clone();
            order.id = i as u64 + 1;
            order.making_amount = base_order.making_amount + (i as u64 * 1000);
            order.taking_amount = base_order.taking_amount + (i as u64 * 2000);
            orders.push(order);
        }

        orders
    }

    /// Benchmark order operations
    pub fn benchmark_operation<F>(operation_name: &str, iterations: usize, f: F)
    where
        F: Fn() -> Result<(), OrderError>,
    {
        let mut total_time = 0u64;
        let mut success_count = 0;
        let mut error_count = 0;

        for _ in 0..iterations {
            let (result, duration) = Self::measure_time(|| f());
            total_time += duration;

            match result {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }

        let avg_time_ms = (total_time / iterations as u64) / 1_000_000; // Convert to ms

        println!("Benchmark Results for {}:", operation_name);
        println!("  Iterations: {}", iterations);
        println!("  Success: {}", success_count);
        println!("  Errors: {}", error_count);
        println!("  Average time: {} ms", avg_time_ms);
        println!("  Total time: {} ms", total_time / 1_000_000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_creation() {
        let order = OrderTestFixtures::create_basic_order();
        OrderAssertions::assert_valid_order(&order);
    }

    #[test]
    fn test_environment_setup() {
        let ctx = TestEnvironment::setup();
        assert!(ctx.setup_complete);
        assert_ne!(ctx.maker, ctx.taker);
    }

    #[test]
    fn test_performance_utils() {
        let (result, duration) = PerformanceTestUtils::measure_time(|| {
            // Simulate some work
            42
        });

        assert_eq!(result, 42);
        assert!(duration > 0);
    }
}
