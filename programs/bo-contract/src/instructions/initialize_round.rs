use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct InitializeRound<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(
    init,
    payer = authority,
    space = Round::SIZE,
  )]
  pub round: Account<'info, Round>,
  #[account(seeds = [b"treasurer".as_ref(), &round.key().to_bytes()], bump)]
  /// CHECK: Just a pure account
  pub treasurer: AccountInfo<'info>,
  pub mint: Box<Account<'info, token::Mint>>,
  #[account(
    init,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = treasurer
  )]
  pub round_token_account: Account<'info, token::TokenAccount>,
  // System Program Address
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<InitializeRound>, start: i64, end: i64) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.start = start;
  round.end = end;
  round.amount = 0;
  round.mint = ctx.accounts.mint.key();
  Ok(())
}
