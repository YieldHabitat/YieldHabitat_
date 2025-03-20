use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Mrkt1111111111111111111111111111111111111");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        marketplace_fee: u16, // in basis points (e.g., 100 = 1.00%)
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.treasury = ctx.accounts.treasury.key();
        marketplace.fee = marketplace_fee;
        marketplace.active_listings = 0;
        marketplace.total_volume = 0;
        marketplace.created_at = Clock::get()?.unix_timestamp;
        marketplace.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn create_listing(
        ctx: Context<CreateListing>,
        price_per_token: u64,
        token_amount: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let seller = &ctx.accounts.seller;
        let property_token_mint = &ctx.accounts.property_token_mint;
        
        listing.seller = seller.key();
        listing.mint = property_token_mint.key();
        listing.price_per_token = price_per_token;
        listing.token_amount = token_amount;
        listing.status = ListingStatus::Active;
        listing.created_at = Clock::get()?.unix_timestamp;
        listing.updated_at = Clock::get()?.unix_timestamp;
        
        // Update marketplace stats
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.active_listings = marketplace.active_listings.checked_add(1)
            .ok_or(MarketplaceError::ArithmeticError)?;
        marketplace.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        token_amount: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let marketplace = &mut ctx.accounts.marketplace;
        
        // Check if listing is active and amount is valid
        require!(
            listing.status == ListingStatus::Active,
            MarketplaceError::ListingNotActive
        );
        
        require!(
            token_amount <= listing.token_amount,
            MarketplaceError::InsufficientTokenAmount
        );
        
        // Calculate transaction amounts
        let total_price = listing.price_per_token
            .checked_mul(token_amount)
            .ok_or(MarketplaceError::ArithmeticError)?;
        
        let fee_amount = (total_price as u128)
            .checked_mul(marketplace.fee as u128)
            .ok_or(MarketplaceError::ArithmeticError)?
            .checked_div(10000)
            .ok_or(MarketplaceError::ArithmeticError)? as u64;
        
        let seller_amount = total_price
            .checked_sub(fee_amount)
            .ok_or(MarketplaceError::ArithmeticError)?;
        
        // Transfer tokens from seller to buyer
        let token_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.seller_authority.to_account_info(),
        };
        
        let token_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_accounts,
        );
        
        token::transfer(token_ctx, token_amount)?;
        
        // Transfer funds from buyer to seller
        if seller_amount > 0 {
            let buyer_lamports = **ctx.accounts.buyer.to_account_info().lamports.borrow();
            require!(
                buyer_lamports >= seller_amount,
                MarketplaceError::InsufficientFunds
            );
            
            // Transfer to seller
            **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= seller_amount;
            **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_amount;
        }
        
        // Transfer fee to marketplace treasury if applicable
        if fee_amount > 0 {
            let buyer_lamports = **ctx.accounts.buyer.to_account_info().lamports.borrow();
            require!(
                buyer_lamports >= fee_amount,
                MarketplaceError::InsufficientFunds
            );
            
            // Transfer to treasury
            **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= fee_amount;
            **ctx.accounts.marketplace_treasury.to_account_info().try_borrow_mut_lamports()? += fee_amount;
        }
        
        // Update listing state
        listing.token_amount = listing.token_amount
            .checked_sub(token_amount)
            .ok_or(MarketplaceError::ArithmeticError)?;
        
        if listing.token_amount == 0 {
            listing.status = ListingStatus::Completed;
            
            // Update marketplace stats
            marketplace.active_listings = marketplace.active_listings
                .checked_sub(1)
                .ok_or(MarketplaceError::ArithmeticError)?;
        }
        
        // Update total volume in marketplace
        marketplace.total_volume = marketplace.total_volume
            .checked_add(total_price)
            .ok_or(MarketplaceError::ArithmeticError)?;
        
        listing.updated_at = Clock::get()?.unix_timestamp;
        marketplace.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn cancel_listing(
        ctx: Context<CancelListing>,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let marketplace = &mut ctx.accounts.marketplace;
        
        // Only the seller can cancel their listing
        require!(
            listing.seller == ctx.accounts.seller.key(),
            MarketplaceError::Unauthorized
        );
        
        // Check if listing is active
        require!(
            listing.status == ListingStatus::Active,
            MarketplaceError::ListingNotActive
        );
        
        // Update listing state
        listing.status = ListingStatus::Cancelled;
        listing.updated_at = Clock::get()?.unix_timestamp;
        
        // Update marketplace stats
        marketplace.active_listings = marketplace.active_listings
            .checked_sub(1)
            .ok_or(MarketplaceError::ArithmeticError)?;
        marketplace.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn update_marketplace_fee(
        ctx: Context<UpdateMarketplace>,
        fee: u16,
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        
        // Check that fee is reasonable (max 10%)
        require!(
            fee <= 1000,
            MarketplaceError::FeeTooHigh
        );
        
        marketplace.fee = fee;
        marketplace.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Marketplace::LEN,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    
    /// The marketplace treasury that will receive fees
    pub treasury: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        init,
        payer = seller,
        space = Listing::LEN,
        seeds = [
            b"listing", 
            seller.key().as_ref(),
            property_token_mint.key().as_ref(),
            &Clock::get()?.unix_timestamp.to_be_bytes()
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    pub property_token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    
    #[account(mut)]
    pub seller_authority: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = listing.status == ListingStatus::Active,
        constraint = listing.seller == seller.key()
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    
    #[account(mut)]
    pub marketplace_treasury: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = seller_token_account.mint == listing.mint,
        constraint = seller_token_account.owner == seller.key()
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.mint == listing.mint,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        constraint = listing.seller == seller.key() @ MarketplaceError::Unauthorized
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
}

#[derive(Accounts)]
pub struct UpdateMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump,
        constraint = marketplace.authority == authority.key() @ MarketplaceError::Unauthorized
    )]
    pub marketplace: Account<'info, Marketplace>,
}

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub fee: u16, // in basis points (e.g., 100 = 1.00%)
    pub active_listings: u64,
    pub total_volume: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Marketplace {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority pubkey
        32 + // treasury pubkey
        2 + // fee
        8 + // active_listings
        8 + // total_volume
        8 + // created_at
        8; // updated_at
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price_per_token: u64,
    pub token_amount: u64,
    pub status: ListingStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Listing {
    pub const LEN: usize = 8 + // discriminator
        32 + // seller pubkey
        32 + // mint pubkey
        8 + // price_per_token
        8 + // token_amount
        1 + // status
        8 + // created_at
        8; // updated_at
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ListingStatus {
    Active,
    Completed,
    Cancelled,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Listing is not active")]
    ListingNotActive,
    
    #[msg("Insufficient token amount")]
    InsufficientTokenAmount,
    
    #[msg("Arithmetic error")]
    ArithmeticError,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Fee too high")]
    FeeTooHigh,
} 