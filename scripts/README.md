# YieldHabitat Scripts

This directory contains various utility scripts for development, deployment, and maintenance of the YieldHabitat platform.

## Scripts Overview

```
scripts/
├── deployment/       # Deployment scripts
│   ├── deploy-dev.sh # Deploy to development environment
│   ├── deploy-prod.sh # Deploy to production environment
│   └── setup-env.sh  # Set up environment variables
├── development/      # Development scripts
│   ├── setup-local.sh # Set up local development environment
│   └── seed-data.js  # Seed database with sample data
├── blockchain/       # Blockchain-related scripts
│   ├── deploy-contracts.js # Deploy smart contracts
│   └── verify-contracts.js # Verify contracts on block explorer
└── README.md         # This file
```

## Usage

### Development Scripts

To set up the local development environment:

```bash
./scripts/development/setup-local.sh
```

To seed the database with sample data:

```bash
node scripts/development/seed-data.js
```

### Deployment Scripts

To deploy to the development environment:

```bash
./scripts/deployment/deploy-dev.sh
```

To deploy to the production environment:

```bash
./scripts/deployment/deploy-prod.sh
```

### Blockchain Scripts

To deploy smart contracts:

```bash
node scripts/blockchain/deploy-contracts.js
```

## Contributing Scripts

When adding new scripts:

1. Place them in the appropriate subdirectory
2. Make shell scripts executable (`chmod +x script.sh`)
3. Add documentation in this README
4. Use consistent naming conventions
5. Include proper error handling and logging 