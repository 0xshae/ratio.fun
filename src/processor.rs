use crate::{error::RatioFunError, instruction::RatioFunInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

mod create_poll;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = RatioFunInstruction::try_from_slice(instruction_data)
        .map_err(|_| RatioFunError::InvalidInstruction)?;

    match instruction {
        RatioFunInstruction::CreatePoll {
            question_uri,
            starts_at_unix_ts,
            ends_at_unix_ts,
            fee_lamports,
            fee_bps,
            creator_fee_bps,
        } => create_poll::process_create_poll(
            program_id,
            accounts,
            question_uri,
            starts_at_unix_ts,
            ends_at_unix_ts,
            fee_lamports,
            fee_bps,
            creator_fee_bps,
        ),
        RatioFunInstruction::Vote { .. } => {
            msg!("ratio.fun Vote: not yet implemented");
            Ok(())
        }
        RatioFunInstruction::Resolve { .. } => {
            msg!("ratio.fun Resolve: not yet implemented");
            Ok(())
        }
        RatioFunInstruction::Claim => {
            msg!("ratio.fun Claim: not yet implemented");
            Ok(())
        }
    }
}

