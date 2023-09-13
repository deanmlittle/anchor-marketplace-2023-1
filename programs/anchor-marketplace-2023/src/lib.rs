#[macro_use]
mod errors;
use errors::*;

mod helpers;

mod state;
use state::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer, transfer as spl_transfer}, metadata::{MetadataAccount, Metadata}
};

declare_id!("Rx94NM1t1bK1UXPZanDYZLF4vjrEBEpZfsxTdMnSiqR");

#[program]
pub mod anchor_marketplace_2023 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        require!(name.len()<33 && name.len()>3, MarketplaceError::InvalidName);
        ctx.accounts.marketplace.admin = *ctx.accounts.admin.key;
        ctx.accounts.marketplace.fee = fee;
        ctx.accounts.marketplace.name = name;
        Ok(())
    }

    pub fn add_collection(ctx: Context<AddCollection>) -> Result<()> {
        ctx.accounts.whitelist.collection_mint = ctx.accounts.mint.key();
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        validate_nft!(
            ctx.accounts.metadata.collection, 
            ctx.accounts.whitelist.collection_mint
        );
        ctx.accounts.listing.owner = ctx.accounts.maker.key();
        ctx.accounts.listing.mint = ctx.accounts.mint.key();
        ctx.accounts.listing.price = price;

        let accounts = SplTransfer {
            from: ctx.accounts.maker_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info()
        };

        let ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            accounts
        );

        spl_transfer(ctx, 1)
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        let accounts = SplTransfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.maker_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info()
        };

        let seeds = [b"auth", &ctx.accounts.listing.key().to_bytes()[..]];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            accounts,
            signer_seeds
        );

        spl_transfer(ctx, 1)
    }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn offer(ctx: Context<Offer>) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = Marketplace::LEN
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,     
        mint::authority = rewards
    )]
    rewards: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddCollection<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        has_one = admin,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [marketplace.key().as_ref(), mint.key().as_ref()],
        bump,
        space = Whitelist::LEN
    )]
    whitelist: Account<'info, Whitelist>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        has_one = collection_mint,
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = mint
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        token::authority = maker,
        token::mint = mint
    )]
    vault: Account<'info, TokenAccount>,
    mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata: Account<'info, MetadataAccount>,
    collection_mint: Box<Account<'info, Mint>>,
    listing: Account<'info, Listing>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, Token>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        has_one = collection_mint,
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = mint
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"auth", whitelist.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe, we only use it for signing
    auth: UncheckedAccount<'info>,
    #[account(
        token::authority = auth,
        token::mint = mint
    )]
    vault: Account<'info, TokenAccount>,
    mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata: Account<'info, MetadataAccount>,
    collection_mint: Box<Account<'info, Mint>>,
    listing: Account<'info, Listing>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, Token>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}