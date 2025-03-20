use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

// Program entrypoint's implementation
pub mod processor;

// Program instructions and data structures
pub mod instruction;

// State management
pub mod state;

// Constants and utility functions
pub mod utils;

// Entry point declaration
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("YieldHabitat Property Tokenization: Processing instruction...");
    processor::process_instruction(program_id, accounts, instruction_data)
} 