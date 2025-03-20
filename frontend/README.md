# YieldHabitat Frontend

This is the frontend application for the YieldHabitat platform, a real estate tokenization platform built on the Solana blockchain.

## Tech Stack

- Next.js 13
- React
- TypeScript
- Tailwind CSS
- Solana Web3.js

## Getting Started

### Prerequisites

- Node.js 16.x or higher
- npm or yarn

### Installation

1. Install dependencies:

```bash
npm install
```

or with yarn:

```bash
yarn
```

2. Set up environment variables by creating a `.env.local` file in the root directory with the following variables:

```
NEXT_PUBLIC_API_URL=http://localhost:5000/api
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.devnet.solana.com
```

3. Start the development server:

```bash
npm run dev
```

or with yarn:

```bash
yarn dev
```

The application will be accessible at http://localhost:3000.

## Project Structure

```
frontend/
├── public/            # Static assets
├── src/
│   ├── app/           # Next.js app router components
│   ├── components/    # Reusable UI components
│   ├── contexts/      # React contexts (wallet, auth, etc.)
│   ├── hooks/         # Custom React hooks
│   ├── lib/           # Utilities and helpers
│   ├── services/      # API service functions
│   ├── styles/        # Global styles
│   └── types/         # TypeScript type definitions
├── node_modules/
├── package.json
├── tailwind.config.js
├── postcss.config.js
├── tsconfig.json
└── README.md
```

## Features

- Property browsing and detailed views
- Token purchase user interface
- Wallet integration for Solana transactions
- User profile and transaction history
- Responsive design for all device sizes

## Development

### Building for Production

```bash
npm run build
```

or with yarn:

```bash
yarn build
```

### Running Production Build Locally

```bash
npm start
```

or with yarn:

```bash
yarn start
```

### Linting

```bash
npm run lint
```

or with yarn:

```bash
yarn lint
```

## Future Implementations

- More comprehensive wallet integration features
- Real-time property updates
- Expanded property filtering and searching
- Investment portfolio analytics 