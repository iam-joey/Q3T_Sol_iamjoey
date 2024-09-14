use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Bet's not availabe")]
    BetDoNotExists,
    #[msg("Deposit amount equal to the odds")]
    AmountNotSufficient,
    #[msg("Invalid Odds")]
    InvalidOdds,
    #[msg("You can't join because event alreday started")]
    EventAlreadyStarted,
    #[msg("Can't cancel event started")]
    EventCantCancel,
}
