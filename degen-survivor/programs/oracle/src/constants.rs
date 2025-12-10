/// PDA seed for OracleConfig
pub const ORACLE_CONFIG_SEED: &[u8] = b"oracle-config";

/// PDA seed for PriceSnapshot
pub const PRICE_SNAPSHOT_SEED: &[u8] = b"price-snapshot";

/// Default staleness threshold: 60 seconds
pub const DEFAULT_STALENESS_THRESHOLD: i64 = 60;

/// Default confidence threshold: 1000
/// Pyth confidence is in price units, lower is better
pub const DEFAULT_CONFIDENCE_THRESHOLD: u64 = 1000;

/// Minimum number of publishers required
pub const DEFAULT_MIN_PUBLISHERS: u8 = 3;

/// Pyth price status values
/// These match Pyth's internal status codes
pub const PYTH_STATUS_TRADING: u32 = 1;
pub const PYTH_STATUS_HALTED: u32 = 2;
pub const PYTH_STATUS_AUCTION: u32 = 3;
pub const PYTH_STATUS_UNKNOWN: u32 = 0;

/// Micro-dollars multiplier for price normalization
pub const MICRO_DOLLARS_MULTIPLIER: u64 = 1_000_000;