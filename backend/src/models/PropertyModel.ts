// This would normally interact with a database
// For this example, we'll use an in-memory array

// Interface for Property data
export interface Property {
  id: string;
  title: string;
  description: string;
  address: string;
  city: string;
  state: string;
  zipCode: string;
  country: string;
  propertyType: string;
  price: number;
  tokenPrice: number;
  totalTokens: number;
  availableTokens: number;
  images: string[];
  features: string[];
  status: 'available' | 'sold' | 'pending';
  rentYield: number;
  appreciationPotential: number;
  documents: {
    title: string;
    url: string;
    type: string;
  }[];
  createdAt: Date;
  updatedAt: Date;
}

// Sample data (will be replaced with database in production)
const properties: Property[] = [
  {
    id: '1',
    title: 'Luxury Apartment in Downtown',
    description: 'A beautiful luxury apartment in the heart of downtown with amazing views and amenities.',
    address: '123 Main St',
    city: 'Miami',
    state: 'FL',
    zipCode: '33101',
    country: 'USA',
    propertyType: 'Apartment',
    price: 500000,
    tokenPrice: 50,
    totalTokens: 10000,
    availableTokens: 8500,
    images: [
      'https://example.com/images/property1-1.jpg',
      'https://example.com/images/property1-2.jpg',
    ],
    features: ['Doorman', 'Gym', 'Pool', 'Parking'],
    status: 'available',
    rentYield: 5.2,
    appreciationPotential: 3.8,
    documents: [
      {
        title: 'Property Deed',
        url: 'https://example.com/documents/property1-deed.pdf',
        type: 'legal'
      },
      {
        title: 'Inspection Report',
        url: 'https://example.com/documents/property1-inspection.pdf',
        type: 'inspection'
      }
    ],
    createdAt: new Date('2023-05-10'),
    updatedAt: new Date('2023-05-15')
  },
  {
    id: '2',
    title: 'Commercial Building in Tech Hub',
    description: 'A prime commercial property in the growing tech district with stable tenants.',
    address: '456 Business Ave',
    city: 'Austin',
    state: 'TX',
    zipCode: '78701',
    country: 'USA',
    propertyType: 'Commercial',
    price: 2000000,
    tokenPrice: 100,
    totalTokens: 20000,
    availableTokens: 15000,
    images: [
      'https://example.com/images/property2-1.jpg',
      'https://example.com/images/property2-2.jpg',
    ],
    features: ['High-speed Internet', 'Conference Rooms', 'Security System', 'Parking Garage'],
    status: 'available',
    rentYield: 7.5,
    appreciationPotential: 4.2,
    documents: [
      {
        title: 'Property Deed',
        url: 'https://example.com/documents/property2-deed.pdf',
        type: 'legal'
      },
      {
        title: 'Tenant Contracts',
        url: 'https://example.com/documents/property2-tenants.pdf',
        type: 'contracts'
      }
    ],
    createdAt: new Date('2023-04-20'),
    updatedAt: new Date('2023-04-25')
  },
  {
    id: '3',
    title: 'Vacation Home in Coastal Area',
    description: 'Beautiful vacation property with ocean views and consistent rental income throughout the year.',
    address: '789 Beach Blvd',
    city: 'Malibu',
    state: 'CA',
    zipCode: '90265',
    country: 'USA',
    propertyType: 'Residential',
    price: 1200000,
    tokenPrice: 80,
    totalTokens: 15000,
    availableTokens: 12000,
    images: [
      'https://example.com/images/property3-1.jpg',
      'https://example.com/images/property3-2.jpg',
    ],
    features: ['Private Beach Access', 'Pool', 'Outdoor Kitchen', 'Smart Home Features'],
    status: 'available',
    rentYield: 6.8,
    appreciationPotential: 5.5,
    documents: [
      {
        title: 'Property Deed',
        url: 'https://example.com/documents/property3-deed.pdf',
        type: 'legal'
      },
      {
        title: 'Rental History',
        url: 'https://example.com/documents/property3-rental-history.pdf',
        type: 'financial'
      }
    ],
    createdAt: new Date('2023-03-15'),
    updatedAt: new Date('2023-03-20')
  }
];

// Property Model
const PropertyModel = {
  // Get all properties
  getAll: async (): Promise<Property[]> => {
    return properties;
  },

  // Get property by id
  getById: async (id: string): Promise<Property | null> => {
    const property = properties.find(p => p.id === id);
    return property || null;
  },

  // Create new property
  create: async (propertyData: Omit<Property, 'id' | 'createdAt' | 'updatedAt'>): Promise<Property> => {
    const newProperty: Property = {
      ...propertyData,
      id: (properties.length + 1).toString(),
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    properties.push(newProperty);
    return newProperty;
  },

  // Update property
  update: async (id: string, propertyData: Partial<Property>): Promise<Property | null> => {
    const index = properties.findIndex(p => p.id === id);
    
    if (index === -1) {
      return null;
    }
    
    properties[index] = {
      ...properties[index],
      ...propertyData,
      updatedAt: new Date()
    };
    
    return properties[index];
  },

  // Delete property
  delete: async (id: string): Promise<boolean> => {
    const index = properties.findIndex(p => p.id === id);
    
    if (index === -1) {
      return false;
    }
    
    properties.splice(index, 1);
    return true;
  }
};

export default PropertyModel; 