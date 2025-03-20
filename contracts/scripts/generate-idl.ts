import * as fs from 'fs';
import * as path from 'path';

// Define paths
const targetPath = path.join(__dirname, '../target/idl');
const idlOutputPath = path.join(__dirname, '../src/idl');

// Ensure output directory exists
if (!fs.existsSync(idlOutputPath)) {
  fs.mkdirSync(idlOutputPath, { recursive: true });
}

// Function to process an IDL file
function processIdl(filename: string) {
  const idlPath = path.join(targetPath, filename);
  const idlData = JSON.parse(fs.readFileSync(idlPath, 'utf8'));
  
  // Add additional metadata (optional)
  idlData.metadata = {
    generatedAt: new Date().toISOString(),
    project: 'YieldHabitat',
  };
  
  // Save the processed IDL
  const outputFilename = filename;
  const outputPath = path.join(idlOutputPath, outputFilename);
  fs.writeFileSync(outputPath, JSON.stringify(idlData, null, 2));
  
  console.log(`Processed IDL: ${filename}`);
  
  // Generate TypeScript types (optional)
  generateTypes(idlData, filename.replace('.json', '.ts'));
}

// Function to generate TypeScript types from IDL
function generateTypes(idlData: any, outputFilename: string) {
  const outputPath = path.join(idlOutputPath, outputFilename);
  
  // Generate TypeScript interfaces from IDL accounts
  let typesContent = `// Generated from IDL - ${new Date().toISOString()}
import { PublicKey } from '@solana/web3.js';

`;

  // Generate account interfaces
  if (idlData.accounts) {
    idlData.accounts.forEach((account: any) => {
      typesContent += `export interface ${account.name} {\n`;
      account.type.fields.forEach((field: any) => {
        let tsType = mapSolanaTypeToTypeScript(field.type);
        typesContent += `  ${field.name}: ${tsType};\n`;
      });
      typesContent += `}\n\n`;
    });
  }
  
  // Generate instruction parameter interfaces
  if (idlData.instructions) {
    idlData.instructions.forEach((instruction: any) => {
      if (instruction.args && instruction.args.length > 0) {
        typesContent += `export interface ${instruction.name}Args {\n`;
        instruction.args.forEach((arg: any) => {
          let tsType = mapSolanaTypeToTypeScript(arg.type);
          typesContent += `  ${arg.name}: ${tsType};\n`;
        });
        typesContent += `}\n\n`;
      }
    });
  }
  
  fs.writeFileSync(outputPath, typesContent);
  console.log(`Generated TypeScript types: ${outputFilename}`);
}

// Helper function to map Solana types to TypeScript types
function mapSolanaTypeToTypeScript(type: any): string {
  if (typeof type === 'string') {
    switch (type) {
      case 'bool': return 'boolean';
      case 'u8':
      case 'i8':
      case 'u16':
      case 'i16':
      case 'u32':
      case 'i32':
      case 'u64':
      case 'i64':
      case 'u128':
      case 'i128':
        return 'number';
      case 'string':
        return 'string';
      case 'publicKey':
        return 'PublicKey';
      default:
        return 'any';
    }
  } else if (type.array) {
    return `${mapSolanaTypeToTypeScript(type.array[0])}[]`;
  } else if (type.option) {
    return `${mapSolanaTypeToTypeScript(type.option)} | null`;
  } else if (type.defined) {
    return type.defined;
  }
  
  return 'any';
}

// Main execution
try {
  console.log('Generating IDL and TypeScript types...');
  
  // Check if target directory exists
  if (!fs.existsSync(targetPath)) {
    console.error(`Target IDL directory not found: ${targetPath}`);
    console.log('Run "anchor build" first to generate IDL files.');
    process.exit(1);
  }
  
  // Process each IDL file
  const files = fs.readdirSync(targetPath).filter(file => file.endsWith('.json'));
  
  if (files.length === 0) {
    console.log('No IDL files found. Run "anchor build" first.');
    process.exit(1);
  }
  
  files.forEach(processIdl);
  console.log('IDL processing complete!');
} catch (error) {
  console.error('Error generating IDL:', error);
  process.exit(1);
} 