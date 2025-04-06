// Allow large result errors
#![allow(clippy::result_large_err)]

// Anchor is a framework for building Solana programs
use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

// Program is a macro that defines the program, it is used to define the entrypoint of the program
#[program]
pub mod voting {

  // This is a macro that imports the solana_program::entrypoint::ProgramResult type
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    // Super is a macro that imports the parent module and all its items
    use super::*;

    // this is a function that initializes the poll account
    pub fn initialize_poll(
      _ctx: Context<InitializePoll>, 
      _poll_id: u64, 
      _description: String, 
      _poll_start: u64, 
      _poll_end: u64) -> ProgramResult{
        // Initialize the poll account 
        let poll = &mut _ctx.accounts.poll;
        poll.poll_id = _poll_id;
        poll.description = _description;
        poll.poll_start = _poll_start;
        poll.poll_end = _poll_end;
        poll.candidate_amount = 0;  
      // Ok is a macro that returns a Result<(), Error> Ok is a macro that returns a Result<(), Error>
      Ok(())
    }
}

// Derive is a macro that derives the Accounts trait for the InitializePoll struct
// Instruction is a macro that derives the Accounts trait for the InitializePoll struct
// InitializePoll is a struct that contains the accounts for the InitializePoll instruction
#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct InitializePoll<'info>{
  // Mut is a macro that makes the account mutable
  // Signer is a macro that makes the account a signer
  // Account is a macro that makes the account an account
  // Program is a macro that makes the account a program
  // System is a macro that makes the account a system  
  #[account(mut)]
  pub signer: Signer<'info>,
  #[account(
    init,
    payer = signer,
    space = 8 + Poll::INIT_SPACE,
    seeds = [pool_id.to_le_bytes().as_ref()],
    bump,
  )]
  pub poll: Account<'info, Poll>, 

  pub system_program: Program<'info, System>,

}

#[account]
#[derive(InitSpace)]
pub struct Poll{
  pub poll_id: u64,
  #[max_len(280)]
  pub description: String,
  pub poll_start: u64,
  pub poll_end: u64,
  pub candidate_amount: u64,
}

