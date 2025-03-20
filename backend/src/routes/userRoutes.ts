import express from 'express';
import { getUsers, getUserById, createUser, updateUser, deleteUser } from '../controllers/userController';

const router = express.Router();

// Get all users
router.get('/', getUsers);

// Get user by ID
router.get('/:id', getUserById);

// Create a new user
router.post('/', createUser);

// Update a user
router.put('/:id', updateUser);

// Delete a user
router.delete('/:id', deleteUser);

export default router; 