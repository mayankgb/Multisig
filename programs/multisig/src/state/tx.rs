use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct TransactionState { 
    #[max_len(3)]
    pub signers: Vec<Pubkey>,
    pub amount: u64,
    pub mint: Pubkey, 
    pub transfer_to : Pubkey, 
    pub is_initialise: bool
}