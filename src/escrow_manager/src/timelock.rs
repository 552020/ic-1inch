/// Timelock management module for conservative cross-chain coordination
///
/// This module handles all timelock-related calculations, validations, and configurations
/// for secure HTLC escrow coordination between ICP and EVM chains.
use crate::types::{EscrowError, TimelockConfig};

/// Buffer constants for conservative timelock coordination
pub mod constants {
    /// Total buffer time in minutes (2 min finality + 1 min coordination)
    pub const BUFFER_MINUTES: u64 = 3;

    /// Finality buffer in nanoseconds (2 minutes for EVM finality)
    pub const FINALITY_BUFFER_NS: u64 = 2 * 60 * 1_000_000_000;

    /// Coordination buffer in nanoseconds (1 minute for coordination)
    pub const COORDINATION_BUFFER_NS: u64 = 1 * 60 * 1_000_000_000;

    /// Total buffer in nanoseconds
    pub const TOTAL_BUFFER_NS: u64 = FINALITY_BUFFER_NS + COORDINATION_BUFFER_NS;

    /// Minimum timelock duration in nanoseconds (10 minutes)
    pub const MIN_TIMELOCK_DURATION_NS: u64 = 10 * 60 * 1_000_000_000;

    /// Additional safety buffer in nanoseconds (5 minutes)
    pub const SAFETY_BUFFER_NS: u64 = 5 * 60 * 1_000_000_000;
}

/// Structure to hold conservative timelock calculation results
#[derive(Clone, Debug)]
pub struct ConservativeTimelocks {
    /// ICP escrow timelock (full user-specified timelock)
    pub icp_timelock: u64,
    /// EVM escrow timelock (earlier to ensure ICP can claim first)
    pub evm_timelock: u64,
    /// Buffer duration in minutes
    pub buffer_minutes: u64,
    /// Complete timelock configuration
    pub config: TimelockConfig,
}

/// Timelock validation result
#[derive(Clone, Debug)]
pub struct TimelockValidation {
    /// Whether the timelock is valid
    pub is_valid: bool,
    /// Minimum required timelock
    pub min_required: u64,
    /// Validation message
    pub message: String,
}

/// Validate timelock duration against minimum requirements
pub fn validate_timelock_duration(timelock: u64, current_time: u64) -> TimelockValidation {
    let min_required = current_time + constants::MIN_TIMELOCK_DURATION_NS;

    if timelock <= min_required {
        TimelockValidation {
            is_valid: false,
            min_required,
            message: format!(
                "Timelock {} is too short. Minimum required: {} (10+ minutes from now)",
                timelock, min_required
            ),
        }
    } else {
        TimelockValidation {
            is_valid: true,
            min_required,
            message: "Timelock duration is valid".to_string(),
        }
    }
}

/// Calculate conservative timelocks with 3-minute buffer strategy
///
/// Buffer breakdown: 2 minutes for finality + 1 minute for coordination
///
/// # Arguments
/// * `base_timelock` - User-specified timelock
/// * `current_time` - Current timestamp in nanoseconds
///
/// # Returns
/// * `ConservativeTimelocks` - Calculated timelocks for ICP and EVM
pub fn calculate_conservative_timelocks(
    base_timelock: u64,
    current_time: u64,
) -> Result<ConservativeTimelocks, EscrowError> {
    // Ensure base timelock is far enough in the future
    let min_timelock = current_time + constants::TOTAL_BUFFER_NS + constants::SAFETY_BUFFER_NS;
    if base_timelock <= min_timelock {
        return Err(EscrowError::TimelockTooShort);
    }

    // Calculate conservative timelocks
    // ICP gets the full timelock (user-specified)
    // EVM gets an earlier timelock to ensure ICP can claim first
    let icp_timelock = base_timelock;
    let evm_timelock = base_timelock - constants::TOTAL_BUFFER_NS;

    let config = create_conservative_timelock_config(current_time);

    ic_cdk::println!(
        "📐 Calculated conservative timelocks: ICP={}, EVM={}, buffer={}min",
        icp_timelock,
        evm_timelock,
        constants::BUFFER_MINUTES
    );

    Ok(ConservativeTimelocks {
        icp_timelock,
        evm_timelock,
        buffer_minutes: constants::BUFFER_MINUTES,
        config,
    })
}

/// Create conservative timelock configuration
pub fn create_conservative_timelock_config(current_time: u64) -> TimelockConfig {
    TimelockConfig {
        deployed_at: current_time,
        src_withdrawal: 3600,           // 1 hour for ICP
        src_public_withdrawal: 7200,    // 2 hours for ICP
        src_cancellation: 10800,        // 3 hours for ICP
        src_public_cancellation: 14400, // 4 hours for ICP
        dst_withdrawal: 1800,           // 30 min for EVM (shorter window)
        dst_public_withdrawal: 3600,    // 1 hour for EVM
        dst_cancellation: 5400,         // 1.5 hours for EVM
        conservative_buffer: constants::BUFFER_MINUTES as u32 * 60, // 3 minutes in seconds
    }
}

/// Check if timelock has expired
pub fn is_timelock_expired(timelock: u64, current_time: u64) -> bool {
    current_time >= timelock
}

/// Calculate time remaining until timelock expiry
pub fn time_until_expiry(timelock: u64, current_time: u64) -> Option<u64> {
    if timelock > current_time {
        Some(timelock - current_time)
    } else {
        None
    }
}

/// Convert nanoseconds to human-readable duration
pub fn format_duration(nanoseconds: u64) -> String {
    let seconds = nanoseconds / 1_000_000_000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h {}m", days, hours % 24, minutes % 60)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

/// Validate timelock coordination between ICP and EVM
pub fn validate_cross_chain_coordination(
    icp_timelock: u64,
    evm_timelock: u64,
) -> Result<(), EscrowError> {
    // Ensure ICP timelock is later than EVM timelock
    if icp_timelock <= evm_timelock {
        return Err(EscrowError::InvalidTimelockCoordination);
    }

    // Ensure buffer is adequate
    let buffer = icp_timelock - evm_timelock;
    if buffer < constants::TOTAL_BUFFER_NS {
        return Err(EscrowError::InvalidTimelockCoordination);
    }

    Ok(())
}

/// Calculate timelock extension during network partitions
pub fn calculate_partition_extension(original_timelock: u64, partition_duration: u64) -> u64 {
    // Extend timelock by 50% of partition duration for safety
    original_timelock + (partition_duration / 2)
}

/// Timelock status for monitoring and debugging
#[derive(Clone, Debug)]
pub enum TimelockStatus {
    Active { remaining: u64 },
    Expired { overdue: u64 },
    Invalid { reason: String },
}

/// Get comprehensive timelock status
pub fn get_timelock_status(timelock: u64, current_time: u64) -> TimelockStatus {
    if timelock == 0 {
        return TimelockStatus::Invalid { reason: "Timelock not set".to_string() };
    }

    if current_time >= timelock {
        TimelockStatus::Expired { overdue: current_time - timelock }
    } else {
        TimelockStatus::Active { remaining: timelock - current_time }
    }
}


