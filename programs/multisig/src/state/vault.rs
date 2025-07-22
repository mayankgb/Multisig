use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultPda { 
    #[max_len(3)]
    pub signers: Vec<Pubkey>,
    pub bump: u8,
    pub seed: u64,
    pub creator: Pubkey,
    pub threshold: u64
}