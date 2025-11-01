use crate::error::RatioFunError;
use crate::state::{PollAccount, PollStatus};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    system_instruction,
};

pub fn process_create_poll(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    question_uri: [u8; 64],
    starts_at_unix_ts: i64,
    ends_at_unix_ts: i64,
    fee_lamports: u64,
    fee_bps: u16,
    creator_fee_bps: u16,
) -> ProgramResult {
    msg!("ratio.fun: Creating poll");

    let account_info_iter = &mut accounts.iter();
    let creator = next_account_info(account_info_iter)?;
    let poll_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Validations
    if !creator.is_signer {
        return Err(RatioFunError::Unauthorized.into());
    }

    if !poll_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    if ends_at_unix_ts <= starts_at_unix_ts {
        return Err(ProgramError::InvalidArgument);
    }

    if fee_bps > 10000 || creator_fee_bps > 10000 {
        return Err(ProgramError::InvalidArgument);
    }

    // Derive poll PDA
    let (poll_pda, bump) = Pubkey::find_program_address(
        &[b"poll", creator.key.as_ref(), question_uri.as_ref()],
        program_id,
    );

    if poll_pda != *poll_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if account is already initialized
    if poll_account.data.borrow().len() > 0 {
        return Err(RatioFunError::InvalidAccountData.into());
    }

    // Initialize account with rent
    let rent = Rent::get()?;
    let required_lamports = rent
        .minimum_balance(std::mem::size_of::<PollAccount>())
        .max(1);

    let poll_seeds = &[b"poll", creator.key.as_ref(), question_uri.as_ref(), &[bump]];

    // Create the account if it doesn't exist
    if poll_account.lamports() == 0 {
        invoke_signed(
            &system_instruction::create_account(
                creator.key,
                poll_account.key,
                required_lamports,
                std::mem::size_of::<PollAccount>() as u64,
                program_id,
            ),
            &[creator.clone(), poll_account.clone(), system_program.clone()],
            &[poll_seeds],
        )?;
    } else {
        // Account exists but needs more rent
        let lamports_needed = required_lamports.saturating_sub(poll_account.lamports());
        if lamports_needed > 0 {
            invoke_signed(
                &system_instruction::transfer(creator.key, poll_account.key, lamports_needed),
                &[creator.clone(), poll_account.clone(), system_program.clone()],
                &[],
            )?;
        }
        // Allocate space for the account data
        invoke_signed(
            &system_instruction::allocate(poll_account.key, std::mem::size_of::<PollAccount>() as u64),
            &[poll_account.clone(), system_program.clone()],
            &[poll_seeds],
        )?;
        invoke_signed(
            &system_instruction::assign(poll_account.key, program_id),
            &[poll_account.clone(), system_program.clone()],
            &[poll_seeds],
        )?;
    }

    // Write poll data
    let poll_data = PollAccount {
        creator: *creator.key,
        resolver_authority: *creator.key, // Creator is resolver by default
        question_uri,
        starts_at_unix_ts,
        ends_at_unix_ts,
        fee_lamports,
        fee_bps,
        creator_fee_bps,
        total_yes: 0,
        total_no: 0,
        status: PollStatus::Active,
        bump,
    };

    let mut data = poll_account.try_borrow_mut_data()?;
    poll_data.serialize(&mut &mut data[..])?;

    msg!("ratio.fun: Poll created at {}", poll_account.key);
    Ok(())
}

