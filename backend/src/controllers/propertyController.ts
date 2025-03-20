import { Request, Response } from 'express';
import PropertyModel, { Property } from '../models/PropertyModel';

// Get all properties
export const getProperties = async (req: Request, res: Response): Promise<void> => {
  try {
    const properties = await PropertyModel.getAll();
    res.status(200).json(properties);
  } catch (error) {
    res.status(500).json({ message: 'Error retrieving properties', error });
  }
};

// Get property by ID
export const getPropertyById = async (req: Request, res: Response): Promise<void> => {
  try {
    const property = await PropertyModel.getById(req.params.id);
    
    if (!property) {
      res.status(404).json({ message: 'Property not found' });
      return;
    }
    
    res.status(200).json(property);
  } catch (error) {
    res.status(500).json({ message: 'Error retrieving property', error });
  }
};

// Create a new property
export const createProperty = async (req: Request, res: Response): Promise<void> => {
  try {
    // Validate required fields
    const requiredFields = [
      'title', 'description', 'address', 'city', 'state', 
      'zipCode', 'country', 'propertyType', 'price', 
      'tokenPrice', 'totalTokens', 'availableTokens', 
      'images', 'features', 'status', 'rentYield', 
      'appreciationPotential'
    ];
    
    const missingFields = requiredFields.filter(field => !req.body[field]);
    
    if (missingFields.length > 0) {
      res.status(400).json({ 
        message: 'Missing required fields', 
        missingFields 
      });
      return;
    }

    const propertyData = req.body;
    const newProperty = await PropertyModel.create(propertyData);
    
    res.status(201).json({
      message: 'Property created successfully',
      property: newProperty
    });
  } catch (error) {
    res.status(500).json({ message: 'Error creating property', error });
  }
};

// Update a property
export const updateProperty = async (req: Request, res: Response): Promise<void> => {
  try {
    const updatedProperty = await PropertyModel.update(req.params.id, req.body);
    
    if (!updatedProperty) {
      res.status(404).json({ message: 'Property not found' });
      return;
    }
    
    res.status(200).json({
      message: 'Property updated successfully',
      property: updatedProperty
    });
  } catch (error) {
    res.status(500).json({ message: 'Error updating property', error });
  }
};

// Delete a property
export const deleteProperty = async (req: Request, res: Response): Promise<void> => {
  try {
    const success = await PropertyModel.delete(req.params.id);
    
    if (!success) {
      res.status(404).json({ message: 'Property not found' });
      return;
    }
    
    res.status(200).json({
      message: 'Property deleted successfully'
    });
  } catch (error) {
    res.status(500).json({ message: 'Error deleting property', error });
  }
}; 