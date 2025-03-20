use solana_program::{
    program_error::ProgramError,
    msg,
};
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyInstruction {
    // Create a new property with the provided details
    CreateProperty {
        property_id: [u8; 32],
        name: String,
        address: String,
        total_value: u64,
        token_supply: u64,
        yield_percentage: u8,
    },
    
    // Purchase property tokens
    PurchaseTokens {
        amount: u64,
    },
    
    // Distribute yield to token holders
    DistributeYield {
        amount: u64,
    },
    
    // Update property value (for revaluations)
    UpdatePropertyValue {
        new_value: u64,
    },
}

impl PropertyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                // Unpack CreateProperty instruction data
                // This is a simplified example - real implementation would need more logic
                msg!("Unpacking CreateProperty instruction");
                Self::CreateProperty {
                    property_id: [0; 32], // Placeholder
                    name: "Sample Property".to_string(), // Placeholder
                    address: "123 Main St".to_string(), // Placeholder
                    total_value: 1000000, // Placeholder
                    token_supply: 1000, // Placeholder
                    yield_percentage: 5, // Placeholder
                }
            }
            1 => {
                // Unpack PurchaseTokens instruction data
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                
                Self::PurchaseTokens { amount }
            }
            2 => {
                // Unpack DistributeYield instruction data
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                
                Self::DistributeYield { amount }
            }
            3 => {
                // Unpack UpdatePropertyValue instruction data
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let new_value = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                
                Self::UpdatePropertyValue { new_value }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
} 