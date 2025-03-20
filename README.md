# YieldHabitat

YieldHabitat is a revolutionary real estate tokenization platform built on multiple blockchains, including Solana, Ethereum, Binance Smart Chain, and Polygon. It allows users to purchase fractional ownership in premium real estate properties through tokenization, creating opportunities for small investors to access high-value real estate markets.

<div align="center">
  <img src="assets/logo.png" alt="YieldHabitat Logo" width="200" />
</div>

## Core Value Proposition

YieldHabitat democratizes real estate investing by:
- **Lowering Entry Barriers**: Invest in premium real estate with as little as $50
- **Increasing Liquidity**: Trade property tokens 24/7 without traditional real estate sale delays
- **Expanding Geographic Access**: Invest in international properties without legal complexity
- **Providing Passive Income**: Earn rental yields paid directly to your wallet
- **Offering Asset Diversification**: Spread investment across multiple properties and markets

## Minimum Viable Product (MVP)

Our initial release includes:

1. **Multi-Chain Property Tokenization**
   - Tokenize real estate on Solana, Ethereum, BSC, and Polygon networks
   - Cross-chain bridge for token transfers between blockchains
   - Automated market makers for token liquidity

2. **Investor Dashboard**
   - Portfolio overview with real-time property valuations
   - Transaction history and yield tracking
   - Property discovery and investment interface
   - KYC verification for regulatory compliance

3. **Property Management System**
   - Detailed property listings with financial metrics
   - Transparent yield calculation and distribution
   - Verification of property ownership records
   - Investment performance analytics

4. **Smart Contract Architecture**
   - Property tokenization contracts with legal compliance
   - Marketplace contracts for secondary trading
   - Registry system for verified properties
   - Cross-chain bridge for token portability

## Project Structure

This project is organized with the following structure:

```
/
├── .github/          # GitHub workflows and configurations
├── assets/           # Shared assets and images
├── backend/          # Backend Express API
├── contracts/        # Smart contracts for multiple blockchains
├── docs/             # Project documentation
├── frontend/         # Frontend React application
├── scripts/          # Utility scripts
├── shared/           # Shared code between frontend and backend
├── docker-compose.yml # Docker Compose configuration
├── .gitignore        # Git ignore file
├── LICENSE           # Project license
├── package.json      # Root package.json for project-wide scripts
└── README.md         # This file
```

## Technology Stack

### Frontend
- React with TypeScript
- Next.js for server-side rendering
- Tailwind CSS for styling
- Wallet adapters for multiple blockchains
- Real-time data visualization with Chart.js

### Backend
- Node.js with Express and TypeScript
- MongoDB for data storage
- RESTful API architecture
- JWT authentication with wallet signatures
- Real-time notifications with Socket.io

### Blockchain Integration
- **Solana**: Primary network for high-performance tokenization
- **Ethereum**: ERC-20 tokens with established market access
- **Binance Smart Chain**: Low-fee alternative for smaller investments
- **Polygon**: Scaling solution for gas-efficient transactions
- **Cross-Chain Bridge**: Custom bridge solution for token transfers

## Key Features

### For Investors
- Browse verified real estate properties with detailed metrics
- Purchase property tokens using multiple cryptocurrency wallets
- Track investment performance and rental yields
- Trade tokens on the secondary marketplace
- Receive real-time notifications on property events
- Access tax documents and investment reports

### For Property Owners
- Tokenize real estate assets with legal documentation
- Set initial token offerings with customizable parameters
- Track investor data and token distributions
- Manage property information and updates
- Distribute rental income automatically through smart contracts

### For Administrators
- Verify property ownership and legal status
- Monitor system health and transaction volumes
- Manage user KYC/AML verification
- Generate platform analytics and reports
- Configure system parameters and fee structures

## Getting Started

### Prerequisites

- Node.js 16.x or higher
- npm or yarn
- Docker and Docker Compose (for containerized development)
- Solana CLI tools (for Solana contracts)
- Hardhat (for EVM contracts)

### Installation

#### Using Docker (Recommended)

1. Clone the repository:

```bash
git clone https://github.com/YieldHabitat/YieldHabitat_.git
cd YieldHabitat_
```

2. Start the Docker containers:

```bash
docker-compose up
```

This will start all services:
- Frontend: http://localhost:3000
- Backend: http://localhost:5000
- MongoDB: mongodb://localhost:27017
- Mongo Express (DB Admin): http://localhost:8081

#### Manual Setup

1. Clone the repository:

```bash
git clone https://github.com/YieldHabitat/YieldHabitat_.git
cd YieldHabitat_
```

2. Install dependencies for all components:

```bash
# Install root dependencies
npm install

# Install backend dependencies
cd backend
npm install

# Install frontend dependencies
cd ../frontend
npm install

# Install contract dependencies
cd ../contracts
npm install
```

3. Set up environment variables by copying the example files and updating them with your configuration:

```bash
# Backend environment variables
cd ../backend
cp .env.example .env

# Frontend environment variables
cd ../frontend
cp .env.example .env

# Contracts environment variables
cd ../contracts
cp .env.example .env
```

4. Start the development servers:

```bash
# Start backend (from backend directory)
npm run dev

# Start frontend (from frontend directory)
npm run dev
```

The frontend will be available at http://localhost:3000 and the backend at http://localhost:5000.

## Documentation

For more detailed information:

- [Frontend Documentation](./frontend/README.md)
- [Backend Documentation](./backend/README.md)
- [Smart Contracts Documentation](./contracts/README.md)
- [API Documentation](./docs/api.md)
- [Multi-Chain Integration](./docs/blockchain-integration.md)

## Roadmap

### Phase 1 - Foundation
- Launch MVP on Solana blockchain
- Implement basic property tokenization
- Develop investor dashboard
- Establish legal framework

### Phase 2 - Expansion
- Add Ethereum blockchain support
- Implement secondary marketplace
- Expand property offerings
- Enhance user experience

### Phase 3 - Ecosystem
- Launch on BSC and Polygon networks
- Implement cross-chain bridge
- Introduce yield farming opportunities
- Add mobile application

### Phase 4 - Growth
- Add institutional investment tools
- Implement DAO governance features
- Expand to additional markets
- Enhanced analytics and reporting

## Contact & Social Media

- Website: [yieldhabitat.online](https://yieldhabitat.online)
- Twitter: [@YieldHabitat_](https://x.com/YieldHabitat_)
- GitHub: [YieldHabitat](https://github.com/YieldHabitat/YieldHabitat_)

## License

This project is licensed under the MIT License - see the LICENSE file for details. 