use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

declare_id!("9JZVRWwQntJVwk1t5mDHkggtHoNW8NkpFtAVqCiorY4Z");

#[program]
pub mod spotify_dapp {
    use super::*;

    pub fn accept_payment(ctx: Context<PayerContext>, data: u64) -> ProgramResult {
        let payer_wallet = &mut ctx.accounts.payer_wallet;
        payer_wallet.wallet = ctx.accounts.authority.key();
        
        let ix = anchor_lang::solana_program::system_instruction::transfer

    }
}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    #[account(
        init, 
        seeds = [b"payer".as_ref(), authority.key.as_ref()],
        bump 
        payer = signer, 
        space = sizeof::<PayerAccount> + 8
    )]
    
    pub payer_wallet: Account<'info, PayerAccount>,
    
    #[account(mut)]
    pub receiver: AccountInfo<'info>

    // Authority
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>


    // Token Program
    #[account(constraint = token_program.key == &token:ID)]
    pub token_program: Program<'info, Token>

    // Clock to save time
    pub clock: Sysvar<'info, Clock>
}


#[account]
pub struct PayerAccount {
    pub wallet::Pubkey,
}
