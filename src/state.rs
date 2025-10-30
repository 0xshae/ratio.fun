use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Yes,
    No,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PollStatus {
    Uninitialized,
    Active,
    Resolved,
    Cancelled,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct PollAccount {
    pub creator: Pubkey,
    pub resolver_authority: Pubkey,
    pub question_uri: [u8; 64],
    pub starts_at_unix_ts: i64,
    pub ends_at_unix_ts: i64,
    pub fee_lamports: u64,
    pub fee_bps: u16,
    pub creator_fee_bps: u16,
    pub total_yes: u64,
    pub total_no: u64,
    pub status: PollStatus,
    pub bump: u8,
}

impl Default for PollAccount {
    fn default() -> Self {
        Self {
            creator: Pubkey::default(),
            resolver_authority: Pubkey::default(),
            question_uri: [0u8; 64],
            starts_at_unix_ts: 0,
            ends_at_unix_ts: 0,
            fee_lamports: 0,
            fee_bps: 0,
            creator_fee_bps: 0,
            total_yes: 0,
            total_no: 0,
            status: PollStatus::Uninitialized,
            bump: 0,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct VoteReceipt {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub side: Side,
    pub staked_lamports: u64,
    pub claimed: bool,
    pub time_placed_unix_ts: i64,
}

