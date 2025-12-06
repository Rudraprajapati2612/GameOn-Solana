/// PDA seed for GlobalVault
pub const GLOBAL_VAULT_SEED: &[u8] = b"global-vault";

/// PDA seed prefix for UserVault
pub const USER_VAULT_SEED: &[u8] = b"user-vault";

/// PDA seed for fee collector account
pub const FEE_COLLECTOR_SEED: &[u8] = b"fee-collector";

/// Default conversion rate: 1 SOL = 10,000 DEGEN tokens
pub const DEFAULT_CONVERSION_RATE: u64 = 10_000;

/// Default withdrawal fee: 5% (500 basis points)
pub const DEFAULT_WITHDRAWAL_FEE_BPS: u16 = 500;

/// Maximum withdrawal fee: 10% (1000 basis points)
pub const MAX_WITHDRAWAL_FEE_BPS: u16 = 1000;

/// Withdrawal timelock duration: 24 hours in seconds
pub const WITHDRAWAL_TIMELOCK_SECONDS: i64 = 24 * 60 * 60; // 86400 seconds

/// Minimum deposit amount: 0.01 SOL (10_000_000 lamports)
pub const MIN_DEPOSIT_LAMPORTS: u64 = 10_000_000;

/// Maximum deposit amount: 100 SOL (to prevent whales from single txns)
pub const MAX_DEPOSIT_LAMPORTS: u64 = 100_000_000_000;

/// DEGEN token decimals (standard Solana token)
pub const TOKEN_DECIMALS: u8 = 9;

/// Basis points divisor (100% = 10000 basis points)
pub const BPS_DIVISOR: u64 = 10_000;

/// SOL lamports per SOL
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;