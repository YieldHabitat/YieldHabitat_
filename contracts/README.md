# YieldHabitat Smart Contracts

This directory contains the blockchain smart contracts for YieldHabitat - a real estate tokenization platform.

## Overview

YieldHabitat's smart contracts are designed to provide a secure, transparent, and compliant system for tokenizing real estate assets across multiple blockchains. The contracts handle property tokenization, marketplace functionality, and a registry system for property verification.

## Architecture

The smart contract architecture is divided into three main programs:

### 1. Property Tokenization Program

This program handles the creation and management of tokens that represent fractional ownership in real estate properties.

**Key Features:**
- Creation of property tokens with detailed metadata
- Minting and distribution of tokens
- Token transfers with compliance checks
- Yield distribution to token holders
- Token burning and redemption

### 2. Marketplace Program

This program facilitates the buying and selling of property tokens in a secure and transparent manner.

**Key Features:**
- Listing property tokens for sale
- Processing token purchases
- Escrow system for secure transactions
- Auction functionality for initial token offerings
- Fee management and distribution
- Secondary market trading

### 3. Registry Program

This program maintains a verified registry of properties and handles verification processes.

**Key Features:**
- Property registration with legal documentation
- Verification by approved authorities
- Status tracking of properties
- Linking on-chain tokens to off-chain legal entities
- Compliance with regulatory requirements

## Multi-Chain Implementation

YieldHabitat supports multiple blockchain networks to provide flexibility and reach to different user bases:

### Solana Implementation
- Uses Anchor framework for secure and efficient programs
- SPL token integration for token management
- Metaplex integration for NFT metadata

### Ethereum Implementation
- ERC-20 tokens for fungible property tokens
- ERC-721 tokens for property NFTs
- ERC-1155 for hybrid token implementations
- OpenZeppelin contracts for security standards

### Binance Smart Chain Implementation
- BEP-20 tokens for property tokenization
- Lower fee structure for accessibility

### Polygon Implementation
- Gas-efficient implementation of Ethereum standards
- Optimized for higher transaction throughput

### Cross-Chain Bridge
- Custom bridge implementation for token transfers between chains
- Liquidity pools for cross-chain swaps
- Security measures for cross-chain validation

## Directory Structure

```
contracts/
├── programs/                 # Solana programs
│   ├── property-tokenization/ # Property token program
│   │   ├── src/              # Source code
│   │   └── Cargo.toml        # Rust dependencies
│   ├── marketplace/          # Marketplace program
│   │   ├── src/              # Source code
│   │   └── Cargo.toml        # Rust dependencies
│   └── registry/             # Registry program
│       ├── src/              # Source code
│       └── Cargo.toml        # Rust dependencies
├── contracts/                # EVM contracts
│   ├── evm/                  # Ethereum, BSC, Polygon contracts
│   │   ├── ethereum/         # Ethereum-specific contracts
│   │   ├── bsc/              # BSC-specific contracts
│   │   └── polygon/          # Polygon-specific contracts
│   └── bridge/               # Cross-chain bridge contracts
├── src/                      # Shared Rust code
├── scripts/                  # Deployment and utility scripts
│   ├── deploy-solana.ts      # Solana deployment script
│   ├── deploy-evm.ts         # EVM deployment script
│   ├── generate-idl.ts       # IDL generation script
│   └── bridge/               # Bridge deployment scripts
├── tests/                    # Contract tests
│   ├── solana/               # Solana tests
│   ├── ethereum/             # Ethereum tests
│   ├── bsc/                  # BSC tests
│   ├── polygon/              # Polygon tests
│   └── integration/          # Cross-chain integration tests
├── Anchor.toml               # Anchor configuration
├── hardhat.config.ts         # Hardhat configuration
└── README.md                 # This file
```

## Key Contract Components

### Property Token Contract

```rust
// Solana implementation (simplified)
#[program]
pub mod property_tokenization {
    use super::*;

    pub fn initialize_property(
        ctx: Context<InitializeProperty>,
        property_data: PropertyData,
        total_supply: u64,
    ) -> Result<()> {
        // Implementation details
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // Implementation details
    }

    pub fn distribute_yield(ctx: Context<DistributeYield>, yield_amount: u64) -> Result<()> {
        // Implementation details
    }
}
```

### Marketplace Contract

```solidity
// Ethereum implementation (simplified)
contract PropertyMarketplace {
    // State variables
    mapping(uint256 => PropertyListing) public listings;
    
    // Functions
    function listProperty(
        address tokenAddress,
        uint256 amount,
        uint256 pricePerToken
    ) external {
        // Implementation details
    }
    
    function purchaseTokens(uint256 listingId, uint256 amount) external payable {
        // Implementation details
    }
    
    function cancelListing(uint256 listingId) external {
        // Implementation details
    }
}
```

### Registry Contract

```rust
// Solana implementation (simplified)
#[program]
pub mod property_registry {
    use super::*;

    pub fn register_property(
        ctx: Context<RegisterProperty>,
        property_data: PropertyData,
        legal_documents: Vec<LegalDocument>,
    ) -> Result<()> {
        // Implementation details
    }

    pub fn verify_property(
        ctx: Context<VerifyProperty>,
        verification_data: VerificationData,
    ) -> Result<()> {
        // Implementation details
    }
}
```

## Security Features

The contracts implement several security features:

1. **Role-Based Access Control**: Different levels of permissions for various actions
2. **Pausable Functionality**: Ability to pause critical functions in emergency situations
3. **Multi-Signature Requirements**: Key operations require multiple signatures
4. **Reentrancy Protection**: Guards against reentrancy attacks
5. **Rate Limiting**: Prevention of transaction spam
6. **Formal Verification**: For critical contract components
7. **Upgradability Patterns**: For future improvements without data loss

## Compliance Features

The contracts include several compliance features:

1. **KYC/AML Integration**: Hooks for KYC/AML verification
2. **Transfer Restrictions**: Configurable restrictions based on regulations
3. **Regulatory Reporting**: Events for regulatory reporting
4. **Jurisdictional Settings**: Configurable parameters based on jurisdictions
5. **Legal Document Hashing**: On-chain verification of legal documents

## Development Setup

### Prerequisites
- Rust and Cargo (for Solana contracts)
- Node.js and npm (for scripts and EVM contracts)
- Solana CLI tools
- Anchor framework
- Hardhat (for EVM development)

### Installation

1. Clone the repository and navigate to the contracts directory:
```bash
cd contracts
```

2. Install dependencies:
```bash
npm install
```

3. Build the Solana programs:
```bash
anchor build
```

4. Generate IDL files:
```bash
npm run generate-idl
```

### Testing

Run tests for all contract implementations:

```bash
# Run Solana tests
anchor test

# Run EVM tests
npx hardhat test
```

### Deployment

Deploy to Solana devnet:
```bash
anchor deploy --provider.cluster devnet
```

Deploy to Ethereum testnet:
```bash
npx hardhat run scripts/deploy-evm.ts --network goerli
```

## Contract Interaction Examples

### Creating a Property Token (Solana)

```typescript
// Using @project-serum/anchor
const createProperty = async () => {
  const propertyData = {
    name: "Luxury Condo",
    location: "Miami, FL",
    squareFootage: 2000,
    expectedYield: 8.5,
    propertyType: PropertyType.RESIDENTIAL,
  };
  
  await program.methods
    .initializeProperty(propertyData, new BN(1000000))
    .accounts({
      property: propertyKeypair.publicKey,
      mint: mintKeypair.publicKey,
      authority: wallet.publicKey,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .signers([propertyKeypair, mintKeypair])
    .rpc();
};
```

### Purchasing Tokens (Ethereum)

```typescript
// Using ethers.js
const purchaseTokens = async (listingId, amount) => {
  const marketplace = new ethers.Contract(
    marketplaceAddress,
    marketplaceAbi,
    signer
  );
  
  const listing = await marketplace.listings(listingId);
  const price = listing.pricePerToken.mul(amount);
  
  await marketplace.purchaseTokens(listingId, amount, {
    value: price
  });
};
```

## Audit Status

All contracts undergo thorough security audits before deployment to mainnet. Current audit status:

- Solana Contracts: Audited by [Audit Firm]
- Ethereum Contracts: Audit in progress
- BSC Contracts: Pending audit
- Polygon Contracts: Pending audit
- Cross-Chain Bridge: Pending audit 