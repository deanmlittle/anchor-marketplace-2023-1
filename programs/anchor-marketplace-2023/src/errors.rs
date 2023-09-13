use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid name. Must be >3 and <33 chars")]
    InvalidName,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Token doesn't belong to a collection")]
    CollectionNotSet
}