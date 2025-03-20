import { Request, Response } from 'express';
import UserModel from '../models/UserModel';

// Get all users
export const getUsers = async (req: Request, res: Response) => {
  try {
    const users = await UserModel.getAll();
    res.status(200).json(users);
  } catch (error) {
    console.error('Error getting users:', error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Get user by ID
export const getUserById = async (req: Request, res: Response) => {
  try {
    const id = req.params.id;
    const user = await UserModel.getById(id);
    
    if (!user) {
      return res.status(404).json({ message: 'User not found' });
    }
    
    res.status(200).json(user);
  } catch (error) {
    console.error(`Error getting user with ID ${req.params.id}:`, error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Create a new user
export const createUser = async (req: Request, res: Response) => {
  try {
    const userData = req.body;
    
    // Validate required fields
    if (!userData.walletAddress || !userData.email) {
      return res.status(400).json({ 
        message: 'Missing required fields: walletAddress and email are required' 
      });
    }
    
    // Check if user with this wallet address already exists
    const existingUserByWallet = await UserModel.getByWalletAddress(userData.walletAddress);
    if (existingUserByWallet) {
      return res.status(400).json({ 
        message: 'User with this wallet address already exists' 
      });
    }
    
    // Check if user with this email already exists
    const existingUserByEmail = await UserModel.getByEmail(userData.email);
    if (existingUserByEmail) {
      return res.status(400).json({ 
        message: 'User with this email already exists' 
      });
    }
    
    const newUser = await UserModel.create(userData);
    
    res.status(201).json({
      success: true,
      user: newUser,
      message: 'User created successfully',
    });
  } catch (error) {
    console.error('Error creating user:', error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Update a user
export const updateUser = async (req: Request, res: Response) => {
  try {
    const id = req.params.id;
    const userData = req.body;
    
    const updatedUser = await UserModel.update(id, userData);
    
    if (!updatedUser) {
      return res.status(404).json({ message: 'User not found' });
    }
    
    res.status(200).json({
      success: true,
      user: updatedUser,
      message: 'User updated successfully',
    });
  } catch (error) {
    console.error(`Error updating user with ID ${req.params.id}:`, error);
    res.status(500).json({ message: 'Server error' });
  }
};

// Delete a user
export const deleteUser = async (req: Request, res: Response) => {
  try {
    const id = req.params.id;
    const deleted = await UserModel.delete(id);
    
    if (!deleted) {
      return res.status(404).json({ message: 'User not found' });
    }
    
    res.status(200).json({
      success: true,
      message: 'User deleted successfully',
    });
  } catch (error) {
    console.error(`Error deleting user with ID ${req.params.id}:`, error);
    res.status(500).json({ message: 'Server error' });
  }
}; 