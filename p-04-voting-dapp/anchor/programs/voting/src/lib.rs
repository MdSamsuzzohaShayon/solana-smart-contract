#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize_poll(_ctx: Context<InitializePoll>, _poll_id: u64) -> ProgramResult{
      // Ok is a macro that returns a Result<(), Error> Ok is a macro that returns a Result<(), Error>
      Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePoll<'Info>{
  #[account(mut)]
  pub signer: Signer<'Info>,
  #[account(
    init,
    payer = payer,
    space = 8 + Poll::INIT_SPACE,
    seeds = [b"poll".as_ref()],
  )]
  pub poll: Account<'Info, Poll>,

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



// Till - https://youtu.be/amAq-WHAFs8?t=4105 01:08:00