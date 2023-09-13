#[macro_export]
macro_rules! validate_nft {
    ($metadata:expr,$mint:expr) => {
        require!(
            $metadata.is_some(), 
            MarketplaceError::CollectionNotSet
        );
        
        require_keys_eq!(
            $metadata.clone().unwrap().key, 
            $mint, 
            MarketplaceError::InvalidCollection
        );

        require!(
            $metadata.clone().unwrap().verified, 
            MarketplaceError::InvalidCollection
        );
    };
}

#[macro_export]
macro_rules! seeds {
    ($($seed:expr),* $(,)?) => {{
        let inner_seeds: &[&[u8]] = &$($seed),*;
        let outer_seeds = &[inner_seeds];
        outer_seeds
    }};
}