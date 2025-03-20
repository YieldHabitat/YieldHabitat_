# YieldHabitat Shared Code

This directory contains shared code used by both the frontend and backend applications.

## Directory Structure

```
shared/
├── types/           # Shared TypeScript type definitions
│   ├── property.ts  # Property-related types
│   ├── user.ts      # User-related types
│   └── transaction.ts # Transaction-related types
├── constants/       # Shared constants
│   ├── api.ts       # API-related constants
│   └── blockchain.ts # Blockchain-related constants
├── utils/           # Shared utility functions
│   ├── validation.ts # Validation utilities
│   └── formatting.ts # Formatting utilities
└── README.md        # This file
```

## Purpose

The shared directory exists to:

1. Maintain consistent types across frontend and backend
2. Avoid code duplication
3. Ensure a single source of truth for common constants
4. Share utility functions that are needed in both environments

## Usage

### From Frontend

```typescript
import { Property } from '@shared/types/property';
import { formatCurrency } from '@shared/utils/formatting';
```

### From Backend

```typescript
import { Property } from '../../../shared/types/property';
import { formatCurrency } from '../../../shared/utils/formatting';
```

## Guidelines

When adding code to the shared directory:

1. Only add code that is truly needed by both frontend and backend
2. Keep dependencies minimal to avoid conflicts
3. Ensure code is well-tested and documented
4. Maintain backward compatibility when making changes 