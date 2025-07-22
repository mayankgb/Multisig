use anchor_lang::{ prelude::*};

use crate::{error::*,state::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateMultiSig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer, 
        space = 8 + VaultPda::INIT_SPACE,
        seeds = [b"vault",signer.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump
    )]
    pub vault: Account<'info, VaultPda>,
    pub system_program: Program<'info,System>
}


pub fn create_multisig(ctx: Context<CreateMultiSig>, seed: u64, threshold: u64) -> Result<()> {
    require!( ctx.remaining_accounts.len() <= 3 , MultiSigError::MaximumAccountLengthError);
    require!( ctx.remaining_accounts.len() > 1, MultiSigError::MinimumAccountLengthError);
    require!( threshold > 1, MultiSigError::MinimumThresholdError);
    require!( threshold <= (ctx.remaining_accounts.len() as u64) , MultiSigError::MaximumThresholdError);
    

    
    let vault =&mut ctx.accounts.vault;

    vault.bump = ctx.bumps.vault;
    vault.seed = seed;

    for account in ctx.remaining_accounts.iter() { 
        vault.signers.push(*account.key);
    }

    vault.creator = ctx.accounts.signer.key();
    vault.threshold = threshold;

    Ok(())
}