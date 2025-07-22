use anchor_lang::prelude::*;

#[error_code]
pub enum MultiSigError {
    #[msg("Account length must equal or less than 3")]
    MaximumAccountLengthError,
    #[msg("Account length must be greater than 0")]
    MinimumAccountLengthError, 
    #[msg("Invalid account for signing this transaction")]
    UnauthorisedAccount,
    #[msg("You already signed the transaction")]
    SignatureAlreadyPresent,
    #[msg("This transaction is already intialise")]
    TransactionExists,
    #[msg("Transaction does not exits")]
    TransactionStateDoesNotExists,
    #[msg("threshold must be less than the total signers for multisig")]
    MaximumThresholdError,
    #[msg("threshold must be greater than 2 for initialising a multisig ")]
    MinimumThresholdError,
    #[msg("Required number of signers is not present")]
    RequiredSigners
}