use anchor_lang::prelude::*;

declare_id!("FovnXa6fTHiskU1jU6XBFUkuMzWBMr8FUGQRuDWh6Yfq");

pub mod instructions;
pub use instructions::*;

pub mod schema;
pub use schema::*;

#[program]
pub mod predictionContract {
  use super::*;
}

#[derive(Accounts)]
pub struct Initialize {}
