use anchor_lang::prelude::*;

mod state;
mod contexts;
mod error;

use error::*;
use contexts::*;

declare_id!("GQTchzJDUpEhVQDtqNSTjj5nQecPyPiwv48ytzycR4BV");

#[program]
pub mod multisig {
    use super::*;

    pub fn initialize_multisig(ctx: Context<CreateMultiSig>, seed:u64, threshold: u64) -> Result<()> {
        create_multisig(ctx, seed, threshold)?;
        Ok(())
    }
    pub fn initialise_transaction(ctx: Context<CreateTx>, seed: u64, amount: u64) -> Result<()> {
        create_transaction(ctx, seed, amount)?;
        Ok(())
    }

    pub fn approve_tx(ctx:Context<ApproveTx>, seed: u64) -> Result<()> {
        approve_transaction(ctx, seed)?;
        Ok(())
    }

    pub fn withdraw_tx(ctx:Context<WithDraw>, seed: u64) -> Result<()> {
        process_withdraw(ctx, seed)?;
        Ok(())
    }
}
