use crate::errors::ErrorCode;
use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct BetUp<'info> {
  // TODO: Customize account address
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(mut, has_one = mint)]
  // round accounts
  pub round: Account<'info, Round>,
  #[account(seeds = [b"treasurer".as_ref(), &round.key().to_bytes()], bump)]
  /// CHECK: Just a pure account
  pub treasurer: AccountInfo<'info>,
  pub mint: Box<Account<'info, token::Mint>>,
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = treasurer
  )]
  pub round_token_account: Account<'info, token::TokenAccount>,
  #[account(
    init_if_needed,
    payer = authority,
    space = Bet::SIZE,
    seeds = [b"ballot".as_ref(), &round.key().to_bytes(), &authority.key().to_bytes()], 
    bump
  )]
  pub ballot: Account<'info, Bet>,
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = authority
  )]
  pub beter_token_account: Account<'info, token::TokenAccount>,
  // System Program Address
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<BetUp>, amount: u64) -> Result<()> {
  let round = &mut ctx.accounts.round;
  let ballot = &mut ctx.accounts.ballot;

  let now = Clock::get().unwrap().unix_timestamp;
  if now < round.start_date || now > round.end_date {
    //return err!(ErrorCode::NotActiveround);
  }

  let transfer_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
      from: ctx.accounts.beter_token_account.to_account_info(),
      to: ctx.accounts.round_token_account.to_account_info(),
      authority: ctx.accounts.authority.to_account_info(),
    },
  );
  token::transfer(transfer_ctx, amount)?;

  round.amount += amount;

  ballot.authority = ctx.accounts.authority.key();
  ballot.round = round.key();
  ballot.amount += amount;

  Ok(())
}
