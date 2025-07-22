use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{error::*, state::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateTx<'info> { 
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub transfer_to: SystemAccount<'info>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault",creator.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultPda>,
    #[account(
        init_if_needed,
        payer = signer, 
        space = 8 + TransactionState::INIT_SPACE
    )]
    pub vault_tx: Account<'info, TransactionState>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

pub fn create_transaction(ctx: Context<CreateTx>, seed: u64, amount:u64) -> Result<()> {

    require!( !ctx.accounts.vault_tx.is_initialise, MultiSigError::TransactionExists);

    let signer = ctx.accounts.signer.key();
    let transaction_state = &mut ctx.accounts.vault_tx;

    transaction_state.signers.push(signer);
    transaction_state.amount = amount;
    transaction_state.is_initialise = true;
    transaction_state.mint = ctx.accounts.mint.key();
    transaction_state.transfer_to = ctx.accounts.transfer_to.key();
    Ok(())    
}