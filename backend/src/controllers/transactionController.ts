import { Request, Response } from 'express';
import TransactionModel from '../models/TransactionModel';
import PropertyModel from '../models/PropertyModel';

// Get all transactions
export const getTransactions = async (req: Request, res: Response) => {
  try {
    const walletAddress = req.query.wallet as string;
    const propertyId = req.query.property as string;
    
    let transactions;
    
    if (walletAddress) {
      transactions = await TransactionModel.getByWallet(walletAddress);
    } else if (propertyId) {
      transactions = await TransactionModel.getByProperty(propertyId);
    } else {
      transactions = await TransactionModel.getAll();
    }
    
    res.status(200).json(transactions);
  } catch (error) {
    console.error('Error getting transactions:', error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Get transaction by ID
export const getTransactionById = async (req: Request, res: Response) => {
  try {
    const id = req.params.id;
    const transaction = await TransactionModel.getById(id);
    
    if (!transaction) {
      return res.status(404).json({ message: 'Transaction not found' });
    }
    
    res.status(200).json(transaction);
  } catch (error) {
    console.error(`Error getting transaction with ID ${req.params.id}:`, error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Create a new transaction (purchase tokens)
export const createTransaction = async (req: Request, res: Response) => {
  try {
    const { propertyId, tokenAmount, walletAddress } = req.body;
    
    // Validate required fields
    if (!propertyId || !tokenAmount || !walletAddress) {
      return res.status(400).json({ 
        message: 'Missing required fields: propertyId, tokenAmount, and walletAddress are required' 
      });
    }
    
    // Get property details
    const property = await PropertyModel.getById(propertyId);
    
    if (!property) {
      return res.status(404).json({ message: 'Property not found' });
    }
    
    // Check if property is active
    if (property.status !== 'Active') {
      return res.status(400).json({ 
        message: 'Property is not available for purchase' 
      });
    }
    
    // Check if enough tokens are available
    const availableTokens = property.totalTokens - property.tokensSold;
    
    if (tokenAmount > availableTokens) {
      return res.status(400).json({ 
        message: `Not enough tokens available. Only ${availableTokens} tokens left.` 
      });
    }
    
    // Calculate transaction amount
    const transactionAmount = property.tokenPrice * tokenAmount;
    
    // Create transaction
    const transaction = await TransactionModel.create({
      propertyId,
      walletAddress,
      tokenAmount,
      transactionAmount,
      timestamp: new Date(),
      status: 'Completed',
      transactionType: 'Purchase',
    });
    
    // Update property tokens sold
    await PropertyModel.update(propertyId, {
      tokensSold: property.tokensSold + tokenAmount
    });
    
    res.status(201).json({
      success: true,
      transaction,
      message: 'Transaction completed successfully',
    });
  } catch (error) {
    console.error('Error creating transaction:', error);
    res.status(500).json({ message: 'Server error' });
  }
}; 