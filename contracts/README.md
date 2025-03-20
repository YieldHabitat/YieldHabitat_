# YieldHabitat Smart Contracts

This directory contains the Solana smart contracts for the YieldHabitat platform, implementing the blockchain infrastructure for real estate tokenization.

## Architecture

YieldHabitat's blockchain architecture is built on Solana using the Anchor framework, consisting of three main programs:

1. **Property Tokenization Program** - Manages the creation, configuration, and distribution of property tokens
2. **Marketplace Program** - Facilitates the buying and selling of property tokens on a secondary market
3. **Registry Program** - Maintains a global registry of properties, ensuring compliance and verification

## Technical Stack

- **Solana Blockchain** - High-throughput, low-cost Layer 1 blockchain
- **Anchor Framework** - Development framework for building Solana programs
- **SPL Token Standard** - For property token implementation
- **Metaplex** - For NFT metadata for properties

## Contract Functionality

### Property Tokenization Program

- Create tokenized properties with detailed metadata
- Configure token distribution parameters (price, quantity, etc.)
- Purchase property tokens
- Track ownership and distribution status
- Update property details and documentation

### Marketplace Program

- List property tokens for sale
- Execute token trades between buyers and sellers
- Implement fee mechanisms for platform sustainability
- Cancel listings
- Track marketplace analytics (volume, active listings, etc.)

### Registry Program (Planned)

- Register new properties with verification
- Enforce regulatory compliance checks
- Manage property status transitions
- Integrate with off-chain legal frameworks

## Directory Structure

```
contracts/
├── src/                    # Source code
│   ├── programs/           # Solana programs
│   │   ├── property-tokenization/  # Property tokenization program
│   │   ├── marketplace/    # Secondary market program
│   │   └── registry/       # Property registry program
│   ├── idl/                # Interface Definition Language files
│   └── tests/              # Program tests
├── scripts/                # Deployment and utility scripts
├── lib/                    # Shared libraries
├── target/                 # Build artifacts
├── Anchor.toml             # Anchor configuration
├── package.json            # NPM configuration
├── tsconfig.json           # TypeScript configuration
└── README.md               # This file
```

## Setting Up Development Environment

### Prerequisites

- Node.js 16+ and npm/yarn
- Rust and Cargo
- Solana CLI Tools
- Anchor CLI

### Installation

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Solana CLI tools:
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.14.18/install)"
```

3. Install Anchor:
```bash
npm install -g @coral-xyz/anchor-cli
```

4. Install dependencies:
```bash
cd contracts
npm install
```

## Development Workflow

### Building the programs

```bash
anchor build
```

This will compile the Rust programs and generate IDL files.

### Generating TypeScript bindings

After building, generate the TypeScript bindings for frontend integration:

```bash
npm run idl:generate
```

### Testing

Run the test suite:

```bash
anchor test
```

### Deployment

Deploy to Solana devnet:

```bash
anchor deploy --provider.cluster devnet
```

Or use the npm script:

```bash
npm run deploy:devnet
```

## Security Considerations

- All contracts will undergo thorough security audits before mainnet deployment
- Multi-signature governance for program upgrades
- Rigorous testing on devnet and testnet before production use
- Token accounts use associated token account standard for predictable derivation

## Integration with Frontend

The contracts expose a set of instructions that can be called from the frontend:

1. The frontend uses the generated IDL files to interact with the on-chain programs
2. Wallet adapters (Phantom, Solflare, etc.) handle transaction signing
3. The shared TypeScript types ensure type safety across the stack

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Blockchain Integration

YieldHabitat is built to operate across multiple blockchains, providing flexibility and broader market reach:

### Supported Blockchains

- **Solana** - Primary blockchain using Anchor framework
- **Ethereum** - Support via EVM-compatible smart contracts
- **Binance Smart Chain** - For lower transaction fees and faster confirmation
- **Polygon** - For scalability and reduced gas costs

### Cross-Chain Functionality

The platform uses a custom bridge solution to enable token transfers across chains:

1. **Property Tokenization** - Available on all supported chains with chain-specific optimizations
2. **Cross-Chain Bridge** - Secure transfer of property tokens between blockchains
3. **Unified Dashboard** - Single interface to manage tokens across all chains

### Bridge Architecture

The bridge mechanism works through a relayer network with the following components:

- **ChainBridge Contract** - Deployed on Ethereum, BSC, and Polygon
- **Solana Bridge Program** - For Solana integration
- **Relayer Service** - A TypeScript application that monitors events across chains and facilitates cross-chain transfers
- **Status Verification** - Transaction status tracking and verification across chains

## Multi-Chain Development Setup

To work with the multi-chain functionality:

### Environment Setup

Ensure your development environment supports all target chains:

```bash
# Install chain-specific dependencies
npm install

# Setup chain-specific networks in .env (example)
echo "ETHEREUM_RPC_URL=https://goerli.infura.io/v3/your-api-key" >> .env
echo "BSC_RPC_URL=https://data-seed-prebsc-1-s1.binance.org:8545" >> .env
echo "POLYGON_RPC_URL=https://rpc-mumbai.maticvigil.com" >> .env
echo "SOLANA_RPC_URL=https://api.devnet.solana.com" >> .env
```

### Deployment

Deploy to multiple chains with provided scripts:

```bash
# Deploy to Solana
anchor deploy

# Deploy to Ethereum testnet (Goerli)
npm run deploy:ethereum

# Deploy to BSC testnet
npm run deploy:bsc

# Deploy to Polygon testnet (Mumbai)
npm run deploy:polygon
```

### Running the Bridge

Start the bridge relay service to enable cross-chain functionality:

```bash
npm run bridge:start
```

## Chain-Specific Considerations

### Solana

- Uses Anchor framework for program development
- SPL tokens represent property tokens
- Lower transaction fees, but more complex program development

### Ethereum

- Standard ERC20 tokens with role-based access
- Higher security and established ecosystem, but higher gas fees
- Uses OpenZeppelin contracts for standardization

### Binance Smart Chain

- Compatible with Ethereum tools (Metamask, Hardhat)
- Lower transaction fees than Ethereum
- Optimized fee structure for property transactions

### Polygon

- Scaled Ethereum solution with high transaction throughput
- Gas-optimized contracts specifically for the Polygon network
- Ideal for smaller property token transactions 