// This would normally interact with a database
// For this example, we'll use an in-memory array

export interface User {
  id: string;
  walletAddress: string;
  email: string;
  firstName?: string;
  lastName?: string;
  avatar?: string;
  createdAt: Date;
  updatedAt: Date;
  kycVerified: boolean;
  properties: string[]; // Array of property IDs the user has invested in
  transactions: string[]; // Array of transaction IDs
}

// Sample data
const users: User[] = [
  {
    id: 'user1',
    walletAddress: '58JUozWgY6D9nzrKvNfYyFu5KHrHx8PJKTr5CPYxT123',
    email: 'john.doe@example.com',
    firstName: 'John',
    lastName: 'Doe',
    avatar: '/assets/images/avatars/avatar1.jpg',
    createdAt: new Date('2023-01-10T12:00:00'),
    updatedAt: new Date('2023-01-10T12:00:00'),
    kycVerified: true,
    properties: ['prop1', 'prop2'],
    transactions: ['txn1', 'txn2'],
  },
  {
    id: 'user2',
    walletAddress: '9f4JUozWgY6D9nzrKvNfYyFu5KHrHx8PJKTr5CPYxT456',
    email: 'jane.smith@example.com',
    firstName: 'Jane',
    lastName: 'Smith',
    avatar: '/assets/images/avatars/avatar2.jpg',
    createdAt: new Date('2023-02-15T09:30:00'),
    updatedAt: new Date('2023-02-15T09:30:00'),
    kycVerified: true,
    properties: ['prop1'],
    transactions: ['txn3'],
  },
  {
    id: 'user3',
    walletAddress: '7bKLmWZ58oM3XsNu5UJAhRiNkdLp4eCqZoMeJjQn789',
    email: 'michael.brown@example.com',
    firstName: 'Michael',
    lastName: 'Brown',
    createdAt: new Date('2023-03-20T15:45:00'),
    updatedAt: new Date('2023-03-20T15:45:00'),
    kycVerified: false,
    properties: [],
    transactions: [],
  },
];

const UserModel = {
  // Get all users
  getAll: async (): Promise<User[]> => {
    return users;
  },
  
  // Get user by ID
  getById: async (id: string): Promise<User | undefined> => {
    return users.find(user => user.id === id);
  },
  
  // Get user by wallet address
  getByWalletAddress: async (walletAddress: string): Promise<User | undefined> => {
    return users.find(user => user.walletAddress === walletAddress);
  },
  
  // Get user by email
  getByEmail: async (email: string): Promise<User | undefined> => {
    return users.find(user => user.email === email);
  },
  
  // Create a new user
  create: async (userData: Omit<User, 'id' | 'createdAt' | 'updatedAt' | 'properties' | 'transactions'>): Promise<User> => {
    const newId = `user${users.length + 1}`;
    const now = new Date();
    
    const newUser: User = {
      id: newId,
      walletAddress: userData.walletAddress,
      email: userData.email,
      firstName: userData.firstName,
      lastName: userData.lastName,
      avatar: userData.avatar,
      createdAt: now,
      updatedAt: now,
      kycVerified: userData.kycVerified || false,
      properties: [],
      transactions: [],
    };
    
    users.push(newUser);
    return newUser;
  },
  
  // Update a user
  update: async (id: string, userData: Partial<Omit<User, 'id' | 'createdAt'>>): Promise<User | undefined> => {
    const index = users.findIndex(user => user.id === id);
    
    if (index === -1) {
      return undefined;
    }
    
    const updatedUser = {
      ...users[index],
      ...userData,
      updatedAt: new Date(),
    };
    
    users[index] = updatedUser;
    return updatedUser;
  },
  
  // Delete a user
  delete: async (id: string): Promise<boolean> => {
    const index = users.findIndex(user => user.id === id);
    
    if (index === -1) {
      return false;
    }
    
    users.splice(index, 1);
    return true;
  },
};

export default UserModel; 