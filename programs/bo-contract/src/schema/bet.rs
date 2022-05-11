use anchor_lang::prelude::*;

#[account]
pub struct Bet {
  pub authority: Pubkey,
  pub candidate: Pubkey,
  pub amount: u64,
}

impl Bet {
  pub const SIZE: usize = 8 + 32 + 32 + 8;
}
