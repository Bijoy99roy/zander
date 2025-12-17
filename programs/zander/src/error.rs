use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Stake below minimum")]
    InsufficientStake,
    #[msg("Voting closed")]
    VotingClosed,
    #[msg("Phase still active")]
    PhaseStillActive,
    #[msg("Already finalized")]
    AlreadyFinalized,
    #[msg("Not a verifier")]
    NotVerifier,
}
