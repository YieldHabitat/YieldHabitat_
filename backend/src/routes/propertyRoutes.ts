import express from 'express';
import { getProperties, getPropertyById, createProperty, updateProperty, deleteProperty } from '../controllers/propertyController';

const router = express.Router();

// Get all properties
router.get('/', getProperties);

// Get property by ID
router.get('/:id', getPropertyById);

// Create a new property
router.post('/', createProperty);

// Update a property
router.put('/:id', updateProperty);

// Delete a property
router.delete('/:id', deleteProperty);

export default router; 