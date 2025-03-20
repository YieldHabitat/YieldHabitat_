# YieldHabitat API Documentation

This document provides detailed information about the YieldHabitat RESTful API endpoints, their usage, parameters, and response formats.

## Base URL

All API endpoints are relative to the base URL:

```
https://api.yieldhabitat.online/api/v1
```

For development environments:
```
http://localhost:5000/api/v1
```

## Authentication

Most API endpoints require authentication. The API uses JWT (JSON Web Token) authentication.

### Authentication Process

1. **Wallet-Based Authentication**: Users sign a message with their wallet
2. **Token Generation**: The server validates the signature and generates a JWT
3. **Token Usage**: The JWT is included in the Authorization header for API requests

### Authentication Headers

For authenticated endpoints, include the following header:

```
Authorization: Bearer <your_jwt_token>
```

## Response Format

All API responses follow a consistent JSON format:

### Success Response

```json
{
  "status": "success",
  "data": {
    // Response data specific to the endpoint
  }
}
```

### Error Response

```json
{
  "status": "error",
  "code": 400, // HTTP status code
  "message": "Error message",
  "details": {
    // Optional detailed error information
  }
}
```

## Rate Limiting

API requests are rate-limited to prevent abuse:

- 100 requests per minute for authenticated users
- 20 requests per minute for unauthenticated users

Rate limit headers are included in all responses:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 98
X-RateLimit-Reset: 1623456789
```

## API Endpoints

### Authentication API

#### Register User

```
POST /auth/register
```

Register a new user with their wallet address.

**Request Body:**
```json
{
  "walletAddress": "string (required)",
  "email": "string (required)",
  "firstName": "string (optional)",
  "lastName": "string (optional)"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "user": {
      "id": "string",
      "walletAddress": "string",
      "email": "string",
      "firstName": "string",
      "lastName": "string",
      "createdAt": "string (ISO date)"
    }
  }
}
```

#### Login (Request Challenge)

```
POST /auth/login/challenge
```

Request a challenge message to sign with wallet.

**Request Body:**
```json
{
  "walletAddress": "string (required)"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "challenge": "string",
    "expiresAt": "string (ISO date)"
  }
}
```

#### Login (Verify Signature)

```
POST /auth/login/verify
```

Verify the signed challenge message and issue JWT token.

**Request Body:**
```json
{
  "walletAddress": "string (required)",
  "signature": "string (required)"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "token": "string (JWT)",
    "expiresAt": "string (ISO date)",
    "user": {
      "id": "string",
      "walletAddress": "string",
      "email": "string",
      "firstName": "string",
      "lastName": "string"
    }
  }
}
```

#### Get Current User

```
GET /auth/me
```

Get the currently authenticated user's profile.

**Response:**
```json
{
  "status": "success",
  "data": {
    "user": {
      "id": "string",
      "walletAddress": "string",
      "email": "string",
      "firstName": "string",
      "lastName": "string",
      "kycVerified": "boolean",
      "createdAt": "string (ISO date)",
      "updatedAt": "string (ISO date)"
    }
  }
}
```

### Properties API

#### List Properties

```
GET /properties
```

Get a list of all properties with optional filtering.

**Query Parameters:**
- `page` (integer, default: 1) - Page number for pagination
- `limit` (integer, default: 10) - Number of items per page
- `sort` (string, default: "createdAt") - Field to sort by
- `order` (string, default: "desc") - Sort order ("asc" or "desc")
- `propertyType` (string) - Filter by property type
- `priceMin` (number) - Minimum price per token
- `priceMax` (number) - Maximum price per token
- `yieldMin` (number) - Minimum expected yield
- `yieldMax` (number) - Maximum expected yield
- `location` (string) - Filter by location

**Response:**
```json
{
  "status": "success",
  "data": {
    "properties": [
      {
        "id": "string",
        "title": "string",
        "description": "string",
        "location": {
          "address": "string",
          "city": "string",
          "state": "string",
          "country": "string",
          "zip": "string",
          "coordinates": {
            "latitude": "number",
            "longitude": "number"
          }
        },
        "propertyType": "string",
        "tokenAddress": "string",
        "totalTokens": "number",
        "availableTokens": "number",
        "pricePerToken": "number",
        "expectedYield": "number",
        "status": "string",
        "images": ["string"],
        "createdAt": "string (ISO date)",
        "updatedAt": "string (ISO date)"
      }
    ],
    "pagination": {
      "total": "number",
      "pages": "number",
      "page": "number",
      "limit": "number"
    }
  }
}
```

#### Get Property by ID

```
GET /properties/:id
```

Get detailed information about a specific property.

**URL Parameters:**
- `id` (string, required) - Property ID

**Response:**
```json
{
  "status": "success",
  "data": {
    "property": {
      "id": "string",
      "title": "string",
      "description": "string",
      "location": {
        "address": "string",
        "city": "string",
        "state": "string",
        "country": "string",
        "zip": "string",
        "coordinates": {
          "latitude": "number",
          "longitude": "number"
        }
      },
      "propertyType": "string",
      "tokenAddress": "string",
      "totalTokens": "number",
      "availableTokens": "number",
      "pricePerToken": "number",
      "expectedYield": "number",
      "status": "string",
      "images": ["string"],
      "documents": ["string"],
      "features": ["string"],
      "financials": {
        "purchasePrice": "number",
        "renovationCost": "number",
        "annualRentalIncome": "number",
        "annualExpenses": "number",
        "netOperatingIncome": "number",
        "capRate": "number"
      },
      "createdAt": "string (ISO date)",
      "updatedAt": "string (ISO date)"
    }
  }
}
```

#### Create Property (Admin)

```
POST /properties
```

Create a new property listing (admin access required).

**Request Body:**
```json
{
  "title": "string (required)",
  "description": "string (required)",
  "location": {
    "address": "string (required)",
    "city": "string (required)",
    "state": "string (required)",
    "country": "string (required)",
    "zip": "string (required)",
    "coordinates": {
      "latitude": "number (required)",
      "longitude": "number (required)"
    }
  },
  "propertyType": "string (required)",
  "tokenAddress": "string (required)",
  "totalTokens": "number (required)",
  "pricePerToken": "number (required)",
  "expectedYield": "number (required)",
  "images": ["string"],
  "documents": ["string"],
  "features": ["string"],
  "financials": {
    "purchasePrice": "number",
    "renovationCost": "number",
    "annualRentalIncome": "number",
    "annualExpenses": "number",
    "netOperatingIncome": "number",
    "capRate": "number"
  }
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "property": {
      "id": "string",
      "title": "string",
      "description": "string",
      // ... full property object
    }
  }
}
```

#### Update Property (Admin)

```
PUT /properties/:id
```

Update an existing property (admin access required).

**URL Parameters:**
- `id` (string, required) - Property ID

**Request Body:**
Fields to update (all fields are optional)

**Response:**
```json
{
  "status": "success",
  "data": {
    "property": {
      "id": "string",
      "title": "string",
      "description": "string",
      // ... updated property object
    }
  }
}
```

#### Delete Property (Admin)

```
DELETE /properties/:id
```

Delete a property (admin access required).

**URL Parameters:**
- `id` (string, required) - Property ID

**Response:**
```json
{
  "status": "success",
  "data": {
    "message": "Property deleted successfully"
  }
}
```

### Transactions API

#### List Transactions

```
GET /transactions
```

Get a list of transactions with optional filtering.

**Query Parameters:**
- `page` (integer, default: 1) - Page number for pagination
- `limit` (integer, default: 10) - Number of items per page
- `walletAddress` (string) - Filter by wallet address
- `propertyId` (string) - Filter by property ID
- `status` (string) - Filter by status
- `startDate` (string) - Filter by start date
- `endDate` (string) - Filter by end date

**Response:**
```json
{
  "status": "success",
  "data": {
    "transactions": [
      {
        "id": "string",
        "propertyId": "string",
        "walletAddress": "string",
        "tokenAmount": "number",
        "transactionAmount": "number",
        "timestamp": "string (ISO date)",
        "status": "string",
        "transactionType": "string",
        "transactionHash": "string",
        "property": {
          "id": "string",
          "title": "string"
        }
      }
    ],
    "pagination": {
      "total": "number",
      "pages": "number",
      "page": "number",
      "limit": "number"
    }
  }
}
```

#### Get Transaction by ID

```
GET /transactions/:id
```

Get details of a specific transaction.

**URL Parameters:**
- `id` (string, required) - Transaction ID

**Response:**
```json
{
  "status": "success",
  "data": {
    "transaction": {
      "id": "string",
      "propertyId": "string",
      "walletAddress": "string",
      "tokenAmount": "number",
      "transactionAmount": "number",
      "timestamp": "string (ISO date)",
      "status": "string",
      "transactionType": "string",
      "transactionHash": "string",
      "property": {
        "id": "string",
        "title": "string",
        "tokenAddress": "string",
        "pricePerToken": "number",
        "images": ["string"]
      }
    }
  }
}
```

#### Create Transaction

```
POST /transactions
```

Create a new transaction to purchase property tokens.

**Request Body:**
```json
{
  "propertyId": "string (required)",
  "tokenAmount": "number (required)"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "transaction": {
      "id": "string",
      "propertyId": "string",
      "walletAddress": "string",
      "tokenAmount": "number",
      "transactionAmount": "number",
      "timestamp": "string (ISO date)",
      "status": "string",
      "transactionType": "string",
      "transactionHash": "string"
    },
    "paymentInstructions": {
      "chain": "string",
      "contractAddress": "string",
      "functionName": "string",
      "parameters": {
        "tokenAmount": "number",
        "paymentAmount": "number"
      }
    }
  }
}
```

### Users API

#### Update User Profile

```
PUT /users/me
```

Update the current user's profile.

**Request Body:**
```json
{
  "email": "string",
  "firstName": "string",
  "lastName": "string",
  "avatar": "string"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "user": {
      "id": "string",
      "walletAddress": "string",
      "email": "string",
      "firstName": "string",
      "lastName": "string",
      "avatar": "string",
      "kycVerified": "boolean",
      "createdAt": "string (ISO date)",
      "updatedAt": "string (ISO date)"
    }
  }
}
```

#### Get User's Properties

```
GET /users/me/properties
```

Get properties associated with the current user.

**Response:**
```json
{
  "status": "success",
  "data": {
    "properties": [
      {
        "id": "string",
        "title": "string",
        "tokenAddress": "string",
        "tokenAmount": "number",
        "pricePerToken": "number",
        "currentValue": "number",
        "expectedYield": "number",
        "acquiredAt": "string (ISO date)",
        "images": ["string"]
      }
    ]
  }
}
```

#### Get User's Transactions

```
GET /users/me/transactions
```

Get transactions associated with the current user.

**Query Parameters:**
- `page` (integer, default: 1) - Page number for pagination
- `limit` (integer, default: 10) - Number of items per page
- `propertyId` (string) - Filter by property ID
- `status` (string) - Filter by status

**Response:**
```json
{
  "status": "success",
  "data": {
    "transactions": [
      {
        "id": "string",
        "propertyId": "string",
        "tokenAmount": "number",
        "transactionAmount": "number",
        "timestamp": "string (ISO date)",
        "status": "string",
        "transactionType": "string",
        "transactionHash": "string",
        "property": {
          "id": "string",
          "title": "string",
          "images": ["string"]
        }
      }
    ],
    "pagination": {
      "total": "number",
      "pages": "number",
      "page": "number",
      "limit": "number"
    }
  }
}
```

### Blockchain API

#### Get Property Token Info

```
GET /blockchain/property/:id/tokens
```

Get token information for a specific property.

**URL Parameters:**
- `id` (string, required) - Property ID

**Query Parameters:**
- `chain` (string, default: "solana") - Blockchain chain

**Response:**
```json
{
  "status": "success",
  "data": {
    "tokenInfo": {
      "tokenAddress": "string",
      "chain": "string",
      "totalSupply": "number",
      "circulatingSupply": "number",
      "tokenHolders": "number",
      "transactions": "number",
      "latestPrice": "number",
      "priceChange24h": "number"
    }
  }
}
```

#### Get User's Token Balance

```
GET /blockchain/balance/:walletAddress
```

Get token balances for a wallet address across all properties.

**URL Parameters:**
- `walletAddress` (string, required) - Wallet address

**Query Parameters:**
- `chain` (string, default: "solana") - Blockchain chain

**Response:**
```json
{
  "status": "success",
  "data": {
    "balances": [
      {
        "propertyId": "string",
        "propertyTitle": "string",
        "tokenAddress": "string",
        "chain": "string",
        "balance": "number",
        "valueUSD": "number"
      }
    ],
    "totalValueUSD": "number"
  }
}
```

### Analytics API

#### Get Property Analytics

```
GET /analytics/property/:id
```

Get analytics data for a specific property.

**URL Parameters:**
- `id` (string, required) - Property ID

**Query Parameters:**
- `period` (string, default: "month") - Time period ("day", "week", "month", "year")

**Response:**
```json
{
  "status": "success",
  "data": {
    "tokenPrice": {
      "current": "number",
      "change": "number",
      "history": [
        {
          "date": "string (ISO date)",
          "price": "number"
        }
      ]
    },
    "transactions": {
      "count": "number",
      "volume": "number",
      "history": [
        {
          "date": "string (ISO date)",
          "count": "number",
          "volume": "number"
        }
      ]
    },
    "tokenHolders": {
      "count": "number",
      "change": "number",
      "history": [
        {
          "date": "string (ISO date)",
          "count": "number"
        }
      ]
    }
  }
}
```

## Webhook API

YieldHabitat provides webhooks for real-time event notifications.

### Register Webhook

```
POST /webhooks
```

Register a new webhook endpoint.

**Request Body:**
```json
{
  "url": "string (required)",
  "events": ["string (required)"],
  "secret": "string (required)"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "webhook": {
      "id": "string",
      "url": "string",
      "events": ["string"],
      "createdAt": "string (ISO date)"
    }
  }
}
```

### Webhook Event Types

- `property.created` - New property created
- `property.updated` - Property information updated
- `property.tokenized` - Property tokenization completed
- `transaction.created` - New transaction created
- `transaction.completed` - Transaction status changed to completed
- `transaction.failed` - Transaction status changed to failed
- `user.registered` - New user registered
- `user.verified` - User completed KYC verification

### Webhook Payload Format

```json
{
  "event": "string",
  "timestamp": "string (ISO date)",
  "data": {
    // Event-specific data
  },
  "signature": "string"
}
```

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource already exists |
| 422 | Unprocessable Entity | Validation error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

## API Versioning

The API version is included in the URL path (e.g., `/api/v1/`). When breaking changes are introduced, a new API version will be released, and the previous version will be supported for at least 6 months.

## Support

For API support, contact [support@yieldhabitat.online](mailto:support@yieldhabitat.online) or join our [Discord community](https://discord.gg/yieldhabitat). 