use anchor_lang::prelude::*;

#[error_code]
pub enum MarketPlaceErrorCodes {
    #[msg("Name is too long")]
    NameIsTooLong,
}
