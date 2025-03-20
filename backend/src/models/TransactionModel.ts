// This would normally interact with a database
// For this example, we'll use an in-memory array

export interface Transaction {
  id: string;
  propertyId: string;
  walletAddress: string;
  tokenAmount: number;
  transactionAmount: number;
  timestamp: Date;
  status: 'Pending' | 'Completed' | 'Failed';
  transactionType: 'Purchase' | 'Sale' | 'Yield';
  transactionHash?: string;
}

// Sample data
const transactions: Transaction[] = [
  {
    id: 'txn1',
    propertyId: 'prop1',
    walletAddress: '58JUozWgY6D9nzrKvNfYyFu5KHrHx8PJKTr5CPYxT123',
    tokenAmount: 100,
    transactionAmount: 5000,
    timestamp: new Date('2023-03-15T10:30:00'),
    status: 'Completed',
    transactionType: 'Purchase',
    transactionHash: '4zH9uzf2zX7AJPnGFmPjZKf1j9VZzZjrDoAGiC3k5FQUVk3X5ycutzkSQE2YaMDCeaRZrUZfJ3cu2vJJoRHgfwXt',
  },
  {
    id: 'txn2',
    propertyId: 'prop2',
    walletAddress: '58JUozWgY6D9nzrKvNfYyFu5KHrHx8PJKTr5CPYxT123',
    tokenAmount: 200,
    transactionAmount: 6400,
    timestamp: new Date('2023-04-20T14:15:00'),
    status: 'Completed',
    transactionType: 'Purchase',
    transactionHash: '5zGBFTuDSh4tBuQyvrZxGW2EkYqy5K1YsyYrcfK5UcTsNZuL9u99s8EcGkVCG99FkLm9XTfHvPRXMJrh7mBLBwxZ',
  },
  {
    id: 'txn3',
    propertyId: 'prop1',
    walletAddress: '9f4JUozWgY6D9nzrKvNfYyFu5KHrHx8PJKTr5CPYxT456',
    tokenAmount: 50,
    transactionAmount: 2500,
    timestamp: new Date('2023-05-05T09:45:00'),
    status: 'Completed',
    transactionType: 'Purchase',
    transactionHash: '2rEVnpj3WmZ3nMpD5PvybmxHm4kjwJDEKsdCFVWNL7RzhJjaSAg9QnbFZP2QsJJVHBNjLZKxVBYudMkX8Gvf1scs',
  },
];

const TransactionModel = {
  // Get all transactions
  getAll: async (): Promise<Transaction[]> => {
    return transactions;
  },
  
  // Get transaction by ID
  getById: async (id: string): Promise<Transaction | undefined> => {
    return transactions.find(transaction => transaction.id === id);
  },
  
  // Get transactions by wallet address
  getByWallet: async (walletAddress: string): Promise<Transaction[]> => {
    return transactions.filter(transaction => transaction.walletAddress === walletAddress);
  },
  
  // Get transactions by property ID
  getByProperty: async (propertyId: string): Promise<Transaction[]> => {
    return transactions.filter(transaction => transaction.propertyId === propertyId);
  },
  
  // Create a new transaction
  create: async (transactionData: Omit<Transaction, 'id'>): Promise<Transaction> => {
    const newId = `txn${transactions.length + 1}`;
    const newTransaction = {
      id: newId,
      ...transactionData,
    };
    
    transactions.push(newTransaction);
    return newTransaction;
  },
};

export default TransactionModel; 