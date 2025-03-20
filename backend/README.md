# YieldHabitat Backend

This directory contains the backend API server for YieldHabitat - a real estate tokenization platform.

## Architecture

The backend follows a modular architecture built with Express.js and TypeScript, designed to handle API requests for property data, user management, and blockchain interactions. Key architectural features include:

- **MVC Pattern**: Models, controllers, and routes are clearly separated
- **Middleware-Based Logic**: Authentication, validation, and error handling via middleware
- **Multi-Database Support**: Primary MongoDB database with blockchain data indexing
- **API-First Design**: Comprehensive RESTful API with OpenAPI documentation

## Directory Structure

```
backend/
├── src/
│   ├── controllers/       # Request handlers
│   │   ├── propertyController.ts
│   │   ├── transactionController.ts
│   │   ├── userController.ts
│   │   └── authController.ts
│   ├── middleware/        # Express middleware
│   │   ├── auth.ts
│   │   ├── validation.ts
│   │   └── errorHandler.ts
│   ├── models/            # Data models
│   │   ├── PropertyModel.ts
│   │   ├── TransactionModel.ts
│   │   └── UserModel.ts
│   ├── routes/            # API route definitions
│   │   ├── propertyRoutes.ts
│   │   ├── transactionRoutes.ts
│   │   └── userRoutes.ts
│   ├── services/          # Business logic and external services
│   │   ├── blockchain/    # Blockchain integration services
│   │   ├── database.ts    # Database connection service
│   │   └── notification.ts # Notification service
│   ├── utils/             # Helper utilities
│   │   ├── validation.ts
│   │   ├── logger.ts
│   │   └── blockchain.ts
│   ├── types/             # TypeScript type definitions
│   ├── config/            # Configuration files
│   └── index.ts           # Application entry point
├── tests/                 # Unit and integration tests
└── docs/                  # API documentation
```

## API Endpoints

The backend provides the following key API endpoints:

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Authenticate user with wallet
- `POST /api/auth/verify` - Verify user's identity
- `GET /api/auth/profile` - Get authenticated user profile

### Properties
- `GET /api/properties` - List all properties with filters
- `GET /api/properties/:id` - Get property details
- `POST /api/properties` - Create a new property (admin)
- `PUT /api/properties/:id` - Update property information (admin)
- `DELETE /api/properties/:id` - Delete property (admin)

### Transactions
- `GET /api/transactions` - List transactions for authenticated user
- `GET /api/transactions/:id` - Get transaction details
- `POST /api/transactions` - Create a new transaction
- `GET /api/transactions/property/:id` - Get transactions for a property

### Users
- `GET /api/users/me` - Get current user profile
- `PUT /api/users/me` - Update user profile
- `GET /api/users/:id` - Get user by ID (admin)
- `PUT /api/users/:id` - Update user (admin)
- `DELETE /api/users/:id` - Delete user (admin)

### Blockchain Integration
- `GET /api/blockchain/property/:id/tokens` - Get token information for property
- `GET /api/blockchain/user/:address/tokens` - Get tokens owned by address

## Database Models

### Property
- `id`: Unique identifier
- `title`: Property title
- `description`: Detailed description
- `location`: Property location details
- `tokenAddress`: Blockchain token contract address
- `totalTokens`: Total number of tokens
- `pricePerToken`: Price per token
- `availableTokens`: Available tokens for purchase
- `expectedYield`: Expected annual yield
- `propertyType`: Type of property
- `status`: Property status (available, sold, etc.)
- `images`: Array of image URLs
- `documents`: Array of document URLs
- `createdAt`: Creation timestamp
- `updatedAt`: Last update timestamp

### Transaction
- `id`: Unique identifier
- `propertyId`: Associated property
- `walletAddress`: User's wallet address
- `tokenAmount`: Number of tokens purchased
- `transactionAmount`: Total transaction value
- `timestamp`: Transaction timestamp
- `status`: Transaction status
- `transactionType`: Type of transaction
- `transactionHash`: Blockchain transaction hash

### User
- `id`: Unique identifier
- `walletAddress`: Blockchain wallet address
- `email`: User email
- `firstName`: First name
- `lastName`: Last name
- `avatar`: Profile image URL
- `kycVerified`: KYC verification status
- `createdAt`: Creation timestamp
- `updatedAt`: Last update timestamp
- `transactions`: Array of transaction IDs
- `properties`: Array of owned property IDs

## Authentication

The backend uses a wallet-based authentication system:
1. User initiates login with wallet address
2. Backend generates a unique message for signing
3. User signs the message with their wallet
4. Backend verifies the signature
5. On success, a JWT token is issued for API access

## Error Handling

The API implements standardized error responses:
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (authentication required)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found (resource doesn't exist)
- `409` - Conflict (resource already exists)
- `500` - Internal Server Error

All errors follow a consistent format:
```json
{
  "status": "error",
  "code": 400,
  "message": "Validation failed",
  "details": {
    "field": "Specific error message"
  }
}
```

## Getting Started

### Prerequisites
- Node.js 16.x or higher
- npm or yarn
- MongoDB (local or cloud instance)
- Access to blockchain RPC nodes

### Installation

1. Install dependencies:
```bash
npm install
```

2. Set up environment variables:
```bash
cp .env.example .env
```
Edit the `.env` file with appropriate values.

3. Start the development server:
```bash
npm run dev
```

The API will be available at http://localhost:5000.

## Building for Production

```bash
npm run build
npm run start
```

## Testing

```bash
# Run unit tests
npm run test

# Run integration tests
npm run test:integration

# Run test coverage
npm run test:coverage
```

## Logging

The backend uses a structured logging system:
- Development: Console output with color coding
- Production: JSON format for log aggregation
- Log levels: error, warn, info, debug, trace

## Security Measures

- CORS configuration for frontend access
- Rate limiting to prevent abuse
- Input validation for all endpoints
- Helmet.js for HTTP header security
- JWT token expiration and refresh mechanism
- Environment-based security configurations

## Tech Stack

- TypeScript
- Node.js
- Express.js

## Project Structure

```
backend/
├── src/
│   ├── controllers/    # Request handlers
│   ├── models/         # Data models
│   ├── routes/         # API routes
│   ├── config/         # Configuration files
│   ├── middleware/     # Express middlewares
│   └── index.ts        # Entry point
├── dist/               # Compiled JavaScript files
├── node_modules/
├── package.json
├── tsconfig.json
└── README.md
```

## Future Implementations

- Database integration (MongoDB/PostgreSQL)
- Solana wallet integration
- Authentication and authorization
- Smart contract interaction 