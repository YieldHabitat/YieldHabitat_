use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

// Validate property data during creation
pub fn validate_property_data(
    name: &str,
    address: &str,
    total_value: u64,
    token_supply: u64,
    yield_percentage: u8,
) -> Result<(), ProgramError> {
    if name.is_empty() || name.len() > 64 {
        msg!("Property name must be between 1 and 64 characters");
        return Err(ProgramError::InvalidArgument);
    }
    
    if address.is_empty() || address.len() > 128 {
        msg!("Property address must be between 1 and 128 characters");
        return Err(ProgramError::InvalidArgument);
    }
    
    if total_value == 0 {
        msg!("Property value must be greater than zero");
        return Err(ProgramError::InvalidArgument);
    }
    
    if token_supply == 0 {
        msg!("Token supply must be greater than zero");
        return Err(ProgramError::InvalidArgument);
    }
    
    if yield_percentage > 100 {
        msg!("Yield percentage must be between 0 and 100");
        return Err(ProgramError::InvalidArgument);
    }
    
    Ok(())
}

// Calculate token price based on property value and token supply
pub fn calculate_token_price(property_value: u64, token_supply: u64) -> u64 {
    if token_supply == 0 {
        return 0;
    }
    property_value / token_supply
}

// Calculate yield amount for a token holder
pub fn calculate_yield_amount(
    tokens_owned: u64,
    total_tokens: u64,
    yield_amount: u64,
) -> u64 {
    if total_tokens == 0 {
        return 0;
    }
    (tokens_owned * yield_amount) / total_tokens
}

// Convert string to fixed size array with padding
pub fn string_to_fixed_array<const N: usize>(s: &str) -> [u8; N] {
    let mut result = [0u8; N];
    let bytes = s.as_bytes();
    let len = bytes.len().min(N);
    result[..len].copy_from_slice(&bytes[..len]);
    result
}

// Check if an account is owned by the program
pub fn check_account_owner(account: &Pubkey, program_id: &Pubkey) -> Result<(), ProgramError> {
    if account != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
} 