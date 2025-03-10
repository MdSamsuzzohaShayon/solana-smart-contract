// Tutorial - https://www.youtube.com/watch?v=Ar-zN8kp5iU&list=PLndmg9UIKNoku3lbeSKyW-RqHt-HRclEu&index=7

use solana_program::{
    account_info::AccountInfo, entrypoint
    , entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
program_id: &Pubkey,
accounts: &[AccountInfo],
instruction_data: &[u8]
)-> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id, accounts.len(), instruction_data
    );
    Ok(())
}
