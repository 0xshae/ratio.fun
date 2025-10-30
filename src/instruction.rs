use borsh::{BorshDeserialize, BorshSerialize};
use crate::state::Side;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum RatioFunInstruction {
    CreatePoll {
        question_uri: [u8; 64],
        starts_at_unix_ts: i64,
        ends_at_unix_ts: i64,
        fee_lamports: u64,
        fee_bps: u16,
        creator_fee_bps: u16,
    },
    Vote {
        side: Side,
        amount_lamports: u64,
    },
    Resolve {
        winning_side: Side,
    },
    Claim,
}

