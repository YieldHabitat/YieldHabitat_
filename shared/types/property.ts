/**
 * Shared type definitions for Property-related data
 */

/**
 * Property status types
 */
export enum PropertyStatus {
  AVAILABLE = 'available',
  PENDING = 'pending',
  SOLD = 'sold'
}

/**
 * Property type enumeration
 */
export enum PropertyType {
  RESIDENTIAL = 'residential',
  COMMERCIAL = 'commercial',
  INDUSTRIAL = 'industrial',
  LAND = 'land'
}

/**
 * Document interface for property-related documents
 */
export interface PropertyDocument {
  id: string;
  title: string;
  url: string;
  type: 'legal' | 'financial' | 'inspection' | 'other';
  createdAt: Date;
}

/**
 * Main Property interface used throughout the application
 */
export interface Property {
  id: string;
  title: string;
  description: string;
  address: string;
  city: string;
  state: string;
  zipCode: string;
  country: string;
  propertyType: PropertyType;
  price: number;
  tokenPrice: number;
  totalTokens: number;
  availableTokens: number;
  images: string[];
  features: string[];
  status: PropertyStatus;
  rentYield: number;
  appreciationPotential: number;
  documents: PropertyDocument[];
  createdAt: Date;
  updatedAt: Date;
}

/**
 * Interface for creating a new property (omits auto-generated fields)
 */
export type CreatePropertyDto = Omit<Property, 'id' | 'createdAt' | 'updatedAt'>;

/**
 * Interface for updating an existing property (all fields optional)
 */
export type UpdatePropertyDto = Partial<Property>;

/**
 * Interface for property list item (with fewer details)
 */
export interface PropertyListItem {
  id: string;
  title: string;
  propertyType: PropertyType;
  city: string;
  state: string;
  country: string;
  price: number;
  tokenPrice: number;
  availableTokens: number;
  totalTokens: number;
  rentYield: number;
  status: PropertyStatus;
  mainImage: string;
} 