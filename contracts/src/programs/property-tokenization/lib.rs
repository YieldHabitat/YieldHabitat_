use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("PTok1111111111111111111111111111111111111");

#[program]
pub mod property_tokenization {
    use super::*;

    pub fn initialize_property(
        ctx: Context<InitializeProperty>,
        property_id: String,
        property_uri: String,
        total_tokens: u64,
        token_price: u64,
        property_details: PropertyDetails,
    ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        property.owner = ctx.accounts.owner.key();
        property.mint = ctx.accounts.mint.key();
        property.property_id = property_id;
        property.property_uri = property_uri;
        property.total_tokens = total_tokens;
        property.available_tokens = total_tokens;
        property.token_price = token_price;
        property.status = PropertyStatus::Available;
        property.details = property_details;
        property.created_at = Clock::get()?.unix_timestamp;
        property.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn purchase_tokens(
        ctx: Context<PurchaseTokens>,
        amount: u64,
    ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        
        require!(
            property.status == PropertyStatus::Available,
            PropertyError::PropertyNotAvailable
        );
        
        require!(
            amount <= property.available_tokens,
            PropertyError::InsufficientTokens
        );

        // Transfer tokens from mint authority to buyer
        let cpi_accounts = Transfer {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Update property state
        property.available_tokens = property.available_tokens.checked_sub(amount)
            .ok_or(PropertyError::ArithmeticError)?;
        
        if property.available_tokens == 0 {
            property.status = PropertyStatus::Sold;
        }
        
        property.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn update_property_details(
        ctx: Context<UpdateProperty>,
        property_details: PropertyDetails,
    ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        
        // Only the owner can update property details
        require!(
            property.owner == ctx.accounts.owner.key(),
            PropertyError::Unauthorized
        );
        
        property.details = property_details;
        property.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn update_property_status(
        ctx: Context<UpdateProperty>,
        status: PropertyStatus,
    ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        
        // Only the owner can update property status
        require!(
            property.owner == ctx.accounts.owner.key(),
            PropertyError::Unauthorized
        );
        
        property.status = status;
        property.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(property_id: String)]
pub struct InitializeProperty<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = Property::LEN,
        seeds = [b"property", property_id.as_bytes()],
        bump
    )]
    pub property: Account<'info, Property>,
    
    pub mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct PurchaseTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    pub property: Account<'info, Property>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
        constraint = buyer_token_account.mint == property.mint,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub treasury_authority: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateProperty<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        constraint = property.owner == owner.key() @ PropertyError::Unauthorized
    )]
    pub property: Account<'info, Property>,
}

#[account]
pub struct Property {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub property_id: String,
    pub property_uri: String,
    pub total_tokens: u64,
    pub available_tokens: u64,
    pub token_price: u64,
    pub status: PropertyStatus,
    pub details: PropertyDetails,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Property {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner pubkey
        32 + // mint pubkey
        36 + // property_id (max 32 chars)
        256 + // property_uri (max 256 chars)
        8 + // total_tokens
        8 + // available_tokens
        8 + // token_price
        1 + // status
        PropertyDetails::LEN + // details
        8 + // created_at
        8; // updated_at
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PropertyStatus {
    Available,
    Pending,
    Sold,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PropertyDetails {
    pub title: String,
    pub description: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip_code: String,
    pub property_type: String,
    pub square_feet: u32,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub year_built: u16,
    pub rent_yield: u16, // in basis points (e.g., 500 = 5.00%)
    pub appreciation_potential: u16, // in basis points
}

impl PropertyDetails {
    pub const LEN: usize = 
        64 + // title (max 64 chars)
        256 + // description (max 256 chars)
        128 + // address (max 128 chars)
        64 + // city (max 64 chars)
        32 + // state (max 32 chars)
        32 + // country (max 32 chars)
        16 + // zip_code (max 16 chars)
        32 + // property_type (max 32 chars)
        4 + // square_feet
        1 + // bedrooms
        1 + // bathrooms
        2 + // year_built
        2 + // rent_yield
        2; // appreciation_potential
}

#[error_code]
pub enum PropertyError {
    #[msg("Property is not available")]
    PropertyNotAvailable,
    
    #[msg("Insufficient tokens available")]
    InsufficientTokens,
    
    #[msg("Arithmetic error")]
    ArithmeticError,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
} 