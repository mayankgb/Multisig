use anchor_lang::prelude::*;

use crate::{MultiSigError,state::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct ApproveTx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
    #[account(mut)]
    pub vault_tx: Account<'info, TransactionState>,
    #[account(
        mut, 
        seeds = [b"vault", creator.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultPda>,
    pub system_program: Program<'info, System>
}

pub fn approve_transaction( ctx: Context<ApproveTx>, seed: u64) -> Result<()> {
    require!(ctx.accounts.vault_tx.is_initialise, MultiSigError::TransactionStateDoesNotExists);

    let signer = ctx.accounts.signer.key();
    let vault = &mut ctx.accounts.vault;
    let transaction_state = &mut ctx.accounts.vault_tx;
    let signers_iter = &mut vault.signers.iter();

    let mut is_signer: bool = false;

    for accounts in signers_iter {
        if *accounts == signer {
            is_signer = true; 
            break;
        }   
    };
    require!(is_signer, MultiSigError::UnauthorisedAccount);
    let transaction_state_signer = &mut transaction_state.signers.iter();
    for transaction_acc in transaction_state_signer { 
        if *transaction_acc == signer { 
            return Err(MultiSigError::SignatureAlreadyPresent.into());
        }
    }

    transaction_state.signers.push(signer);


    Ok(())
}