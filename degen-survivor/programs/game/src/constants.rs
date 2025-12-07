pub const GAME_SEED : &[u8] = b"game";

/// PDA seed for PlayerState
pub const PLAYER_SEED: &[u8] = b"player";

/// PDA seed for RoundResult
pub const ROUND_RESULT_SEED: &[u8] = b"round-result";
// 500 Degen 
pub const DEFAULT_ENTRY_FEE : u64 = 500_000_000_000;

pub const MAX_PLAYER : u16 = 50;

pub const MIN_PLAYER :u8= 2;

pub const TOTAL_ROUND :u8 = 5;

pub const ROUND_DURATION_SECONDS :i64 = 60;
// 2min gaps between 2 round 
pub const ROUND_GAP_SECONDS :i64 = 120 ;

/// Prediction lockout window (last N seconds, no submissions allowed)
/// During the last 5 seconds, predictions are locked, so users cannot change or submit answers when prices or outcomes are almost known.
pub const PREDICTION_LOCKOUT_SECONDS: i64 = 5;
// platform fee in bias point (600 = 6%)
pub const PLATFORM_FEE_BPS:u16 = 600;

pub const MAX_USERNAME_LENGTH : usize = 20;
// point for correct answer
pub const POINT_EXCATE:u16 = 100;

pub const POINT_PARTIAL : u16 = 50 ;

pub const POINT_CLOSER : u16 = 60 ;
pub const POINT_FAR : u16 = 20;

pub const POINT_WRONG : u16 = 0;
// 40%
pub const PRIZE_RANK_1_BPS : u16 = 4000;
// 20%
pub const PRIZE_RANK_2_BPS : u16 = 2000;
// 12%
pub const PRIZE_RANK_3_BPS : u16 = 1200;
// 6%
pub const PRIZE_RANK_4_5_BPS : u16 = 600;
// 2%
pub const PRIZE_RANK_6_10_BPS : u16 = 200;

/// Basis points divisor
pub const BPS_DIVISOR: u64 = 10_000;

/// Time before game start when registration closes (2 minutes)
pub const REGISTRATION_CLOSE_BEFORE_START: i64 = 120;