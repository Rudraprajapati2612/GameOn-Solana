use anchor_lang::prelude::*;

#[account]

pub struct GameState{
    // unique game id timestamp based 
    pub game_id : u64,
    // Game Type (BTC only sol only)
    pub game_type : GameType,
    // backend (who create this game)
    pub creator : Pubkey,
    // current game status [Pending,Active,Completed,Canceled]
    pub status : GameStatus,

    // when game is created at  
    pub created_at : i64,

    // Schedule start time Example (10 AM and 8 Pm) 
    pub start_time : i64,

    pub actual_start_time : Option<i64>,

    pub end_time : Option<i64>,

    pub current_round : u8,

    pub total_round : u8 ,
    /// Deadlines for each round [Round1_end, Round2_end, ...]
    pub round_deadline : [i64;5],
    // per player 500 degen token 
    pub entry_fee : u64,

    pub prize_pool : u64,
    // platform fees in bias point (600=6%)
    pub platform_fee_bps : u16,
    // wheter prize pool is being distributed or not 
    pub prize_pool_distributed : bool,

    pub total_player : u16,
    // max player is 50 
    pub max_player : u16,

    // to check for maximum player is reached 
    pub player_finalized : bool,    
    // types of question for each round 
    pub round_type  : [RoundType;5],

    pub leaderboard_finalized : bool,

    pub top_scorer : Option<Pubkey>,

    pub higest_score : u16,

    pub bump : u8,

    pub _reserved : [u8;128],
}

impl GameState {
    // 8 (discriminator) + 8 + 1 + 32 + 1 + 8 + 8 + 9 + 9 + 1 + 1 + 40 + 8 + 8 + 2 + 1 + 2 + 2 + 1 + 5 + 1 + 33 + 2 + 1 + 128
    pub const SIZE: usize = 8 + 8 + 1 + 32 + 1 + 8 + 8 + 9 + 9 + 1 + 1 + 40 + 8 + 8 + 2 + 1 + 2 + 2 + 1 + 5 + 1 + 33 + 2 + 1 + 128;
}

#[account]

pub struct PlayerState{
    pub game_id : u64 ,

    pub player : Pubkey,

    pub username : String ,

    pub entry_slot : u64,

    pub predection : [Option<RoundPrediction>;5],

    pub scores : [u16;5],

    pub total_score : u16,

    pub round_evaluated : u8,

    pub all_round_completed : bool,

    pub final_rank :  Option<u16>,

    pub prize_amount : u64 ,

    pub prize_claimed : u64,

    pub total_reponse_time : u64,
    pub avg_response_time : u32,

 /// Timestamp of first prediction (tie-breaker)
    pub first_prediction_ts: i64,
    
    /// PDA bump
    pub bump: u8,
    
    /// Reserved
    pub _reserved: [u8; 64],
}

impl PlayerState {
    // Rough calculation - actual size will be larger due to Option and String
    pub const SIZE: usize = 8 + 8 + 32 + 32 + 8 + 600 + 10 + 2 + 1 + 1 + 3 + 8 + 1 + 8 + 4 + 8 + 1 + 64;
}


#[account]

pub struct RoundResult {
    // which game is belong to 
    pub game_id : u64 ,
    // round number (1-5)
    pub round_number : u8,
    //  Types of Question 
    pub round_type : RoundType,

    pub start_price_btc : Option<u64>,

    pub end_price_btc : Option<u64>,

    pub start_price_sol : Option<u64>,

    pub end_price_sol : Option<u64>,
    // price can be positive or negative 
    pub price_change_btc : i64,

    pub price_change_sol : i64,

    pub correct_answer : Option<PredectionChoice>,

    pub round_start_ts : i64,

    pub round_end_ts : i64,

    pub evaluation_ts : Option<i64>,

    pub total_predection : u16,
    pub correct_predection:u16,
    pub partial_correct : u16,
    pub wrong_predection : u16,

    pub bump : u8,   
}

impl RoundResult {
    pub const SIZE: usize = 8 + 8 + 1 + 1 + 9 + 9 + 9 + 9 + 8 + 8 + 2 + 8 + 8 + 9 + 2 + 2 + 2 + 2 + 1 + 32;
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]

pub struct  RoundPrediction {
    pub round : u8,
    // Player chois for evualation
    pub choice : PredectionChoice,
    // when sumbited 
    pub sumbited_at : i64 ,

    // user take time for evualation 
    pub response_time : u32,
    // after evualation 
    pub point_earned : u16,
    // whether predection is corret or not 
    pub is_correct : bool
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]

pub enum GameType {
    BtcOnly,
    SolOnly,
    BtcVsSol
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]

pub enum  GameStatus {
    Pending ,
    Active ,
    Completed,
    Cancelled 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]

pub enum  RoundType {
    // Simple Up and Down 
    PriceDirection,
    // how much price move 
    Magnitude,
    // which asset will move more 
    Comperative ,
    // Specific Price Range 
    Range,
    // Pattern Predection
    Trend
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]

pub enum PredectionChoice {
    // PriceDirection 
    Up,
    Down,

    // Magnitude 

    // for magnitude
    RangeA, //less than $50
    RangeB, // between 50 and 100
    RangeC, //
    RangeD,

    // for comperative 

    BtcMore,
    SolMore,
    Equal,

    // For Price Range 

    ZoneA,
    ZoneB,
    ZoneC,
    ZoneD,

    // for trend 

    HigherHigher, 
    LowerLower,
    HigherLower,
    LowerHigher,
}



impl  PlayerState {
    //  prevent Double Predection 
    // so it checks predection for Specific round and make it true if not asnwered and false if no such round exist 
    pub fn has_predicted(&self,round:u8)->bool{
        if round == 0 || round > 5 {
            return false
        }else {
            self.predection[(round-1) as usize].is_some()
        }
    }

    pub fn get_prediction(&self, round: u8) -> Option<&RoundPrediction> {
        if round == 0 || round > 5 {
            return None;
        }
        self.predection[(round - 1) as usize].as_ref()
    }
}