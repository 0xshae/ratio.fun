use crate::{error::CursedTakesError, instruction::CursedTakesInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CursedTakesInstruction::try_from_slice(instruction_data)
        .map_err(|_| CursedTakesError::InvalidInstruction)?;

    match instruction {
        CursedTakesInstruction::CreatePoll { .. } => {
            msg!("CreatePoll: not yet implemented");
        }
        CursedTakesInstruction::Vote { .. } => {
            msg!("Vote: not yet implemented");
        }
        CursedTakesInstruction::Resolve { .. } => {
            msg!("Resolve: not yet implemented");
        }
        CursedTakesInstruction::Claim => {
            msg!("Claim: not yet implemented");
        }
    }

    Ok(())
}

