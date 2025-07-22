use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::{error::*,state::{TransactionState, VaultPda}};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct WithDraw<'info> {
    #[account(mut)] 
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = signer
    )]
    pub vault_tx: Account<'info, TransactionState>,
    #[account(mut)]
    pub creator_acc: SystemAccount<'info>,
    #[account(
        mut, 
        seeds = [b"vault", creator_acc.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, VaultPda>,
    #[account(
        mut,
        constraint = withdraw_acc.key() == vault_tx.transfer_to
    )]
    pub withdraw_acc: SystemAccount<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed, 
        payer = signer, 
        associated_token::mint = mint, 
        associated_token::authority = withdraw_acc, 
        associated_token::token_program = token_program 
    )]
    pub withdraw_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        associated_token::mint = mint, 
        associated_token::authority = vault
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info, System>
}


pub fn process_withdraw(ctx: Context<WithDraw>, seed: u64) -> Result<()> { 
    require!( ctx.accounts.vault_tx.signers.len() as u64 >= ctx.accounts.vault.threshold, MultiSigError::RequiredSigners );
    require!( ctx.accounts.vault_tx.is_initialise, MultiSigError::TransactionStateDoesNotExists);

    // let vault_tx_acc = &mut ctx.accounts.vault_tx;
    let signer = ctx.accounts.signer.key();
    let signer_iter = ctx.accounts.vault.signers.iter();
    let vault_pda_bump = ctx.accounts.vault.bump;
    let creator = ctx.accounts.creator_acc.key();
    let amount = ctx.accounts.vault_tx.amount.clone();

    let mut is_valid_signer = false;

    for accounts in signer_iter { 
        if *accounts == signer {
             is_valid_signer = true;
        }
    }

    require!(is_valid_signer, MultiSigError::UnauthorisedAccount);

    let accounts = TransferChecked { 
        from: ctx.accounts.vault_ata.to_account_info(), 
        to: ctx.accounts.withdraw_ata.to_account_info().clone(), 
        mint: ctx.accounts.mint.to_account_info(), 
        authority: ctx.accounts.vault.to_account_info()
    };
    let random_seed = seed.to_le_bytes();

    let seeds = &[b"vault", creator.as_ref(),random_seed.as_ref() , &[vault_pda_bump]];
    let signer_seeds: &[&[&[u8]]]= &[seeds];

    let token_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    transfer_checked(token_cpi_ctx, amount, ctx.accounts.mint.decimals)?;

    Ok(())
}