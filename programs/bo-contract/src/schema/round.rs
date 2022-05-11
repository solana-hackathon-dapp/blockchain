use anchor_lang::prelude::*;

#[account]
pub struct Round {
  pub mint: Pubkey,
  pub amount: u64,
  pub start: i64,
  pub end: i64,
}

impl Round {
  pub const SIZE: usize = 8 + 32 + 8 + 8 + 8;
}
