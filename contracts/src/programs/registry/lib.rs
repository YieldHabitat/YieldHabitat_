use anchor_lang::prelude::*;

declare_id!("Rgst1111111111111111111111111111111111111");

#[program]
pub mod registry {
    use super::*;

    pub fn initialize_registry(
        ctx: Context<InitializeRegistry>
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.property_count = 0;
        registry.verifier_count = 0;
        registry.created_at = Clock::get()?.unix_timestamp;
        registry.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn add_verifier(
        ctx: Context<ManageVerifier>,
        verifier_name: String,
        verifier_url: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let verifier_account = &mut ctx.accounts.verifier_account;
        
        // Only registry authority can add verifiers
        require!(
            registry.authority == ctx.accounts.authority.key(),
            RegistryError::Unauthorized
        );
        
        verifier_account.name = verifier_name;
        verifier_account.url = verifier_url;
        verifier_account.authority = ctx.accounts.authority.key();
        verifier_account.is_active = true;
        verifier_account.verified_properties = 0;
        verifier_account.created_at = Clock::get()?.unix_timestamp;
        verifier_account.updated_at = Clock::get()?.unix_timestamp;
        
        // Update registry stats
        registry.verifier_count = registry.verifier_count
            .checked_add(1)
            .ok_or(RegistryError::ArithmeticError)?;
        registry.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn remove_verifier(
        ctx: Context<ManageVerifier>
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let verifier_account = &mut ctx.accounts.verifier_account;
        
        // Only registry authority can remove verifiers
        require!(
            registry.authority == ctx.accounts.authority.key(),
            RegistryError::Unauthorized
        );
        
        // Mark as inactive instead of completely removing
        verifier_account.is_active = false;
        verifier_account.updated_at = Clock::get()?.unix_timestamp;
        
        // Update registry stats
        registry.verifier_count = registry.verifier_count
            .checked_sub(1)
            .ok_or(RegistryError::ArithmeticError)?;
        registry.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn register_property(
        ctx: Context<RegisterProperty>,
        property_id: String,
        property_address: String,
        legal_details: LegalDetails,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let property_record = &mut ctx.accounts.property_record;
        
        property_record.property_id = property_id;
        property_record.token_mint = ctx.accounts.token_mint.key();
        property_record.owner = ctx.accounts.owner.key();
        property_record.address = property_address;
        property_record.legal_details = legal_details;
        property_record.verification_status = VerificationStatus::Pending;
        property_record.created_at = Clock::get()?.unix_timestamp;
        property_record.updated_at = Clock::get()?.unix_timestamp;
        
        // Update registry stats
        registry.property_count = registry.property_count
            .checked_add(1)
            .ok_or(RegistryError::ArithmeticError)?;
        registry.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn verify_property(
        ctx: Context<VerifyProperty>,
        verification_details: VerificationDetails,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let property_record = &mut ctx.accounts.property_record;
        let verifier = &mut ctx.accounts.verifier;
        
        // Only active verifiers can verify properties
        require!(
            verifier.is_active,
            RegistryError::VerifierNotActive
        );
        
        // Only verifier authority can submit verifications
        require!(
            verifier.authority == ctx.accounts.verifier_authority.key(),
            RegistryError::Unauthorized
        );
        
        // Update property record
        property_record.verification_status = VerificationStatus::Verified;
        property_record.verification_details = Some(verification_details);
        property_record.verifier = Some(ctx.accounts.verifier.key());
        property_record.verified_at = Some(Clock::get()?.unix_timestamp);
        property_record.updated_at = Clock::get()?.unix_timestamp;
        
        // Update verifier stats
        verifier.verified_properties = verifier.verified_properties
            .checked_add(1)
            .ok_or(RegistryError::ArithmeticError)?;
        verifier.updated_at = Clock::get()?.unix_timestamp;
        
        // Update registry stats
        registry.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn update_property_status(
        ctx: Context<UpdatePropertyStatus>,
        status: VerificationStatus,
    ) -> Result<()> {
        let property_record = &mut ctx.accounts.property_record;
        let registry = &mut ctx.accounts.registry;
        
        // Only registry authority can update status
        require!(
            registry.authority == ctx.accounts.authority.key(),
            RegistryError::Unauthorized
        );
        
        property_record.verification_status = status;
        property_record.updated_at = Clock::get()?.unix_timestamp;
        registry.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Registry::LEN,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, Registry>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ManageVerifier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"registry"],
        bump,
        constraint = registry.authority == authority.key() @ RegistryError::Unauthorized
    )]
    pub registry: Account<'info, Registry>,
    
    #[account(
        init_if_needed,
        payer = authority,
        space = Verifier::LEN,
        seeds = [b"verifier", authority.key().as_ref()],
        bump
    )]
    pub verifier_account: Account<'info, Verifier>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(property_id: String)]
pub struct RegisterProperty<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, Registry>,
    
    #[account(
        init,
        payer = owner,
        space = PropertyRecord::LEN,
        seeds = [b"property", property_id.as_bytes()],
        bump
    )]
    pub property_record: Account<'info, PropertyRecord>,
    
    pub token_mint: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct VerifyProperty<'info> {
    #[account(mut)]
    pub verifier_authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = verifier.authority == verifier_authority.key() @ RegistryError::Unauthorized,
        constraint = verifier.is_active @ RegistryError::VerifierNotActive
    )]
    pub verifier: Account<'info, Verifier>,
    
    #[account(mut)]
    pub property_record: Account<'info, PropertyRecord>,
    
    #[account(
        mut,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, Registry>,
}

#[derive(Accounts)]
pub struct UpdatePropertyStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"registry"],
        bump,
        constraint = registry.authority == authority.key() @ RegistryError::Unauthorized
    )]
    pub registry: Account<'info, Registry>,
    
    #[account(mut)]
    pub property_record: Account<'info, PropertyRecord>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub property_count: u64,
    pub verifier_count: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Registry {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority pubkey
        8 + // property_count
        8 + // verifier_count
        8 + // created_at
        8; // updated_at
}

#[account]
pub struct Verifier {
    pub authority: Pubkey,
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub verified_properties: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Verifier {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority pubkey
        64 + // name (max 64 chars)
        128 + // url (max 128 chars)
        1 + // is_active
        8 + // verified_properties
        8 + // created_at
        8; // updated_at
}

#[account]
pub struct PropertyRecord {
    pub property_id: String,
    pub token_mint: Pubkey,
    pub owner: Pubkey,
    pub address: String,
    pub legal_details: LegalDetails,
    pub verification_status: VerificationStatus,
    pub verification_details: Option<VerificationDetails>,
    pub verifier: Option<Pubkey>,
    pub verified_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl PropertyRecord {
    pub const LEN: usize = 8 + // discriminator
        36 + // property_id (max 32 chars)
        32 + // token_mint pubkey
        32 + // owner pubkey
        256 + // address (max 256 chars)
        LegalDetails::LEN + // legal_details
        1 + // verification_status
        (1 + VerificationDetails::LEN) + // Option<VerificationDetails>
        (1 + 32) + // Option<Pubkey>
        (1 + 8) + // Option<i64>
        8 + // created_at
        8; // updated_at
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LegalDetails {
    pub title_deed_url: String,
    pub owner_name: String,
    pub owner_id: String,
    pub legal_description: String,
    pub jurisdiction: String,
    pub property_type: String,
    pub zoning: String,
    pub last_sale_date: i64,
    pub last_sale_amount: u64,
}

impl LegalDetails {
    pub const LEN: usize = 
        128 + // title_deed_url (max 128 chars)
        64 + // owner_name (max 64 chars)
        32 + // owner_id (max 32 chars)
        256 + // legal_description (max 256 chars)
        32 + // jurisdiction (max 32 chars)
        32 + // property_type (max 32 chars)
        32 + // zoning (max 32 chars)
        8 + // last_sale_date
        8; // last_sale_amount
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VerificationDetails {
    pub verification_date: i64,
    pub verification_method: String,
    pub verification_notes: String,
    pub is_legal_compliance_verified: bool,
    pub is_property_condition_verified: bool,
    pub is_valuation_verified: bool,
    pub verification_expiry: i64,
}

impl VerificationDetails {
    pub const LEN: usize = 
        8 + // verification_date
        32 + // verification_method (max 32 chars)
        256 + // verification_notes (max 256 chars)
        1 + // is_legal_compliance_verified
        1 + // is_property_condition_verified
        1 + // is_valuation_verified
        8; // verification_expiry
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum VerificationStatus {
    Pending,
    InProgress,
    Verified,
    Rejected,
    Expired,
}

#[error_code]
pub enum RegistryError {
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("Arithmetic error")]
    ArithmeticError,
    
    #[msg("Verifier is not active")]
    VerifierNotActive,
} 