use anchor_lang::prelude::*;

declare_id!("J5QfhmCuNNJtYvFjwzSTxGxoQ9yRKhvKodddh8KLBrzF");

#[program]
pub mod solana_alkes_contract {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_words = 0;
    Ok(())
  }

  pub fn add_word(ctx: Context<AddWord>, word: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let item = ItemStruct {
      word: word.to_string(),
      user_address: *base_account.to_account_info().key,
    };
    base_account.contributer_list.push(item);
    base_account.total_words += 1;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddWord<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_words: u64,
    pub contributer_list: Vec<ItemStruct>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub word: String,
    pub user_address: Pubkey,
}