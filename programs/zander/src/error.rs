use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Stake below minimum")]
    InsufficientStake,
    #[msg("Voting closed")]
    VotingClosed,
    #[msg("Voting Phase still active")]
    VotingPhaseStillActive,
    #[msg("Already finalized")]
    AlreadyFinalized,
    #[msg("Not a verifier")]
    NotVerifier,
    #[msg("Not enough votes")]
    NotEnoughVotes,
    #[msg("Verifier is missing from accounts")]
    MissingVerifier,
    #[msg("Mathematical overflow during operation")]
    MathOverflow,
    #[msg("Verifier vault is missing from accounts")]
    VaultNotFound,
}
