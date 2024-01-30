use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey, msg, program_error::ProgramError, system_program, system_instruction, program::invoke_signed,
};


// to_pubkey_account
// pda_account
// system_program_account

const TOKEN_ADDRESS: &str = "DFL1zNkaGPWm1BqAVqRjCZvHmwTFrEaJtbzJWgseoNJh";
const PAYER_SEED: &[u8] = b"payer_seeds";

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    msg!("processing instruction for: {}, instruction count: {}, instruction data: {:?}", 
        program_id, 
        accounts.len(), 
        instruction_data
    );

    let iter = &mut accounts.iter();

    let pda_account = next_account_info(iter)?;
    let recepient_account = next_account_info(iter)?;
    let system_program_account = next_account_info(iter)?;

    let token_program_account = next_account_info(iter)?;
    let pda_token_account = next_account_info(iter)?;
    let destination_token_account = next_account_info(iter)?;
    

    let (pda, bump) = Pubkey::find_program_address(&[PAYER_SEED], program_id);

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if !system_program::check_id(system_program_account.key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("pda: {}, bump: {}",pda, bump);

    if instruction_data.len() != 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix = system_instruction::transfer(
        pda_account.key, 
        recepient_account.key, 
        1000);

    invoke_signed(&ix, 
        &[
        pda_account.clone(),
        recepient_account.clone(),
        system_program_account.clone()
    ], &[
        &[
            &b"payer_seeds"[..],
            &[bump]
        ],
    ])?;

    if spl_token::check_id(token_program_account.key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("pda_token_account: {}", pda_token_account.key);
    msg!("signer token account: {}", destination_token_account.key);
    msg!("pda account: {}", pda_account.key);
    msg!("token program account: {}", token_program_account.key);

    let ix = spl_token::instruction::transfer(
        token_program_account.key, 
        pda_token_account.key, 
        destination_token_account.key, 
        pda_account.key, 
        &[], 
        1000
    )?;

    invoke_signed(&ix, 
        &[
            pda_token_account.clone(),
            destination_token_account.clone(),
            pda_account.clone(),
            token_program_account.clone()
    ], &[
        &[
            &b"payer_seeds"[..],
            &[bump]
        ],
    ])?;

    Ok(())
}
