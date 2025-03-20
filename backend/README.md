# YieldHabitat Backend API

This is the backend API server for the YieldHabitat platform, a real estate tokenization application built on the Solana blockchain.

## Tech Stack

- TypeScript
- Node.js
- Express.js

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

2. Set up environment variables by creating a `.env` file in the root directory with the following variables:

```
PORT=5000
NODE_ENV=development
```

3. Start the development server:

```bash
npm run dev
```

or with yarn:

```bash
yarn dev
```

The server will start on port 5000 by default (or whatever port you specified in the .env file).

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

## API Endpoints

### Properties

- `GET /api/properties` - Get all properties
- `GET /api/properties/:id` - Get a specific property
- `POST /api/properties` - Create a new property
- `PUT /api/properties/:id` - Update a property
- `DELETE /api/properties/:id` - Delete a property

### Users

- `GET /api/users` - Get all users
- `GET /api/users/:id` - Get a specific user
- `POST /api/users` - Create a new user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

### Transactions

- `GET /api/transactions` - Get all transactions or filter by wallet or property
- `GET /api/transactions/:id` - Get a specific transaction
- `POST /api/transactions` - Create a new transaction (purchase tokens)

## Development

### Building the Project

```bash
npm run build
```

or with yarn:

```bash
yarn build
```

### Running in Production

```bash
npm start
```

or with yarn:

```bash
yarn start
```

## Future Implementations

- Database integration (MongoDB/PostgreSQL)
- Solana wallet integration
- Authentication and authorization
- Smart contract interaction 