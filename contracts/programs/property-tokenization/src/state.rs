use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq)]
pub struct Property {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub property_id: [u8; 32],
    pub name: [u8; 64],  // Fixed size for simplicity
    pub address: [u8; 128],  // Fixed size for simplicity
    pub total_value: u64,
    pub token_supply: u64,
    pub tokens_sold: u64,
    pub yield_percentage: u8,
    pub last_yield_distribution: u64,  // Unix timestamp
}

impl Sealed for Property {}

impl IsInitialized for Property {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Property {
    const LEN: usize = 1 + 32 + 32 + 64 + 128 + 8 + 8 + 8 + 1 + 8;
    
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Property::LEN];
        let (
            is_initialized,
            owner,
            property_id,
            name,
            address,
            total_value,
            token_supply,
            tokens_sold,
            yield_percentage,
            last_yield_distribution,
        ) = array_refs![src, 1, 32, 32, 64, 128, 8, 8, 8, 1, 8];
        
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        
        Ok(Property {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            property_id: *property_id,
            name: *name,
            address: *address,
            total_value: u64::from_le_bytes(*total_value),
            token_supply: u64::from_le_bytes(*token_supply),
            tokens_sold: u64::from_le_bytes(*tokens_sold),
            yield_percentage: yield_percentage[0],
            last_yield_distribution: u64::from_le_bytes(*last_yield_distribution),
        })
    }
    
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Property::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            property_id_dst,
            name_dst,
            address_dst,
            total_value_dst,
            token_supply_dst,
            tokens_sold_dst,
            yield_percentage_dst,
            last_yield_distribution_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 64, 128, 8, 8, 8, 1, 8];
        
        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        property_id_dst.copy_from_slice(&self.property_id);
        name_dst.copy_from_slice(&self.name);
        address_dst.copy_from_slice(&self.address);
        *total_value_dst = self.total_value.to_le_bytes();
        *token_supply_dst = self.token_supply.to_le_bytes();
        *tokens_sold_dst = self.tokens_sold.to_le_bytes();
        yield_percentage_dst[0] = self.yield_percentage;
        *last_yield_distribution_dst = self.last_yield_distribution.to_le_bytes();
    }
}

#[derive(Debug, PartialEq)]
pub struct PropertyToken {
    pub is_initialized: bool,
    pub property_id: [u8; 32],
    pub owner: Pubkey,
    pub amount: u64,
    pub purchase_price: u64,
    pub purchase_date: u64,  // Unix timestamp
    pub last_yield_claim: u64,  // Unix timestamp
}

impl Sealed for PropertyToken {}

impl IsInitialized for PropertyToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for PropertyToken {
    const LEN: usize = 1 + 32 + 32 + 8 + 8 + 8 + 8;
    
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PropertyToken::LEN];
        let (
            is_initialized,
            property_id,
            owner,
            amount,
            purchase_price,
            purchase_date,
            last_yield_claim,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8];
        
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        
        Ok(PropertyToken {
            is_initialized,
            property_id: *property_id,
            owner: Pubkey::new_from_array(*owner),
            amount: u64::from_le_bytes(*amount),
            purchase_price: u64::from_le_bytes(*purchase_price),
            purchase_date: u64::from_le_bytes(*purchase_date),
            last_yield_claim: u64::from_le_bytes(*last_yield_claim),
        })
    }
    
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PropertyToken::LEN];
        let (
            is_initialized_dst,
            property_id_dst,
            owner_dst,
            amount_dst,
            purchase_price_dst,
            purchase_date_dst,
            last_yield_claim_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8];
        
        is_initialized_dst[0] = self.is_initialized as u8;
        property_id_dst.copy_from_slice(&self.property_id);
        owner_dst.copy_from_slice(self.owner.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *purchase_price_dst = self.purchase_price.to_le_bytes();
        *purchase_date_dst = self.purchase_date.to_le_bytes();
        *last_yield_claim_dst = self.last_yield_claim.to_le_bytes();
    }
} 