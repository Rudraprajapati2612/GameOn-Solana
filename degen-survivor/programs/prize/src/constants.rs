/// PDA seed for PrizePool
pub const PRIZE_POOL_SEED: &[u8] = b"prize-pool";

/// PDA seed for ClaimRecord
pub const CLAIM_RECORD_SEED: &[u8] = b"claim-record";

/// PDA seed for FeeCollector
pub const FEE_COLLECTOR_SEED: &[u8] = b"fee-collector";

/// Platform fee in basis points (600 = 6%)
pub const PLATFORM_FEE_BPS: u16 = 600;

/// Basis points divisor (100% = 10000 bps)
pub const BPS_DIVISOR: u64 = 10_000;

/// Number of winners (top 10)
pub const TOTAL_WINNERS: u16 = 10;

/// Prize distribution percentages (in basis points)
/// Total = 94% (6% reserved for platform fee)

/// Rank 1: 40% of distributable pool
pub const PRIZE_RANK_1_BPS: u16 = 4000;

/// Rank 2: 20% of distributable pool
pub const PRIZE_RANK_2_BPS: u16 = 2000;

/// Rank 3: 12% of distributable pool
pub const PRIZE_RANK_3_BPS: u16 = 1200;

/// Ranks 4-5: 6% each of distributable pool
pub const PRIZE_RANK_4_5_BPS: u16 = 600;

/// Ranks 6-10: 2% each of distributable pool
pub const PRIZE_RANK_6_10_BPS: u16 = 200;