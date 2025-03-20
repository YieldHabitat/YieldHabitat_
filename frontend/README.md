# YieldHabitat Frontend

This directory contains the frontend application for YieldHabitat - a real estate tokenization platform.

## Architecture

The frontend is built using a modern React stack with Next.js for server-side rendering and improved SEO performance. The architecture follows these principles:

- **Component-Based Design**: Reusable UI components organized by function and complexity
- **State Management**: Context API for app-wide state with local state for component-specific needs
- **Responsive Design**: Mobile-first approach using Tailwind CSS
- **Multi-Wallet Integration**: Support for multiple blockchain wallets

## Directory Structure

```
frontend/
├── public/                # Static files (images, fonts)
├── src/
│   ├── components/        # Reusable UI components
│   │   ├── common/        # Shared components (buttons, inputs, etc.)
│   │   ├── layout/        # Layout components (header, footer, etc.)
│   │   ├── property/      # Property-related components
│   │   ├── wallet/        # Wallet integration components
│   │   └── dashboard/     # Dashboard components
│   ├── context/           # React Context for state management
│   ├── hooks/             # Custom React hooks
│   ├── pages/             # Next.js pages
│   ├── services/          # API and blockchain services
│   ├── styles/            # Global styles and Tailwind configuration
│   ├── types/             # TypeScript type definitions
│   └── utils/             # Utility functions
└── tests/                 # Test files
```

## Key Features

### Multi-Wallet Integration
The frontend supports multiple blockchain wallets:
- Phantom (Solana)
- MetaMask (Ethereum, BSC, Polygon)
- Solflare (Solana)
- WalletConnect (Multiple chains)

### Property Marketplace
- Browse listings with filter and sort capabilities
- Detailed property views with metrics and documentation
- Interactive maps for property locations
- Real-time data on available tokens and pricing

### Investor Dashboard
- Portfolio overview with visualization charts
- Transaction history with filtering options
- Yield tracking and distribution history
- Property management for token owners

### Authentication and Security
- Wallet-based authentication with signature verification
- KYC integration for regulatory compliance
- Role-based access control
- Secure API communication with JWT

## Getting Started

### Prerequisites
- Node.js 16.x or higher
- npm or yarn
- A modern web browser

### Installation

1. Install dependencies:
```bash
npm install
```

2. Set up environment variables:
```bash
cp .env.example .env.local
```
Edit the `.env.local` file with appropriate values.

3. Start the development server:
```bash
npm run dev
```

The application will be available at http://localhost:3000.

## Building for Production

```bash
npm run build
npm run start
```

## Testing

```bash
# Run unit tests
npm run test

# Run end-to-end tests
npm run test:e2e

# Run test coverage
npm run test:coverage
```

## Styling

This project uses Tailwind CSS for styling. The configuration can be found in `tailwind.config.js`.

To add or modify styles:
1. For component-specific styles, use Tailwind utility classes directly in the component
2. For reusable styles, create components in the `components/common` directory
3. For global styles, edit the files in the `styles` directory

## State Management

The application uses a combination of:
- React Context API for global state (user, wallet, selected property)
- Local component state for UI-specific functionality
- SWR for data fetching and caching

## Blockchain Integration

Blockchain integration is handled through:
- Wallet adapter components in `components/wallet`
- Blockchain-specific services in `services/blockchain`
- Web3 utility functions in `utils/web3`

## Contribution Guidelines

1. Create a feature branch off the `dev` branch
2. Make your changes following the code style guide
3. Write tests for your changes
4. Submit a pull request
5. Update documentation as needed

## Performance Optimizations

- Component code splitting for reduced bundle sizes
- Image optimization with Next.js Image component
- Server-side rendering for improved SEO and initial load time
- Caching strategies for blockchain and API data 