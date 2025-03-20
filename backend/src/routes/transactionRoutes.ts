import express from 'express';
import { getTransactions, getTransactionById, createTransaction } from '../controllers/transactionController';

const router = express.Router();

// Get all transactions or filter by wallet or property
router.get('/', getTransactions);

// Get transaction by ID
router.get('/:id', getTransactionById);

// Create a new transaction
router.post('/', createTransaction);

export default router; 