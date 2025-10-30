use crate::{error::RatioFunError, instruction::RatioFunInstruction};
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
    let instruction = RatioFunInstruction::try_from_slice(instruction_data)
        .map_err(|_| RatioFunError::InvalidInstruction)?;

    match instruction {
        RatioFunInstruction::CreatePoll { .. } => {
            msg!("ratio.fun CreatePoll: not yet implemented");
        }
        RatioFunInstruction::Vote { .. } => {
            msg!("ratio.fun Vote: not yet implemented");
        }
        RatioFunInstruction::Resolve { .. } => {
            msg!("ratio.fun Resolve: not yet implemented");
        }
        RatioFunInstruction::Claim => {
            msg!("ratio.fun Claim: not yet implemented");
        }
    }

    Ok(())
}

