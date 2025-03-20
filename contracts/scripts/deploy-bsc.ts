import { ethers } from 'hardhat';
import * as dotenv from 'dotenv';
import { PropertyTokenBSC__factory } from '../typechain-types';
import { ChainBridge__factory } from '../typechain-types';

dotenv.config();

async function main() {
  console.log('Deploying YieldHabitat contracts to BSC...');

  const [deployer] = await ethers.getSigners();
  console.log(`Deploying contracts with the account: ${deployer.address}`);

  const balance = await deployer.getBalance();
  console.log(`Account balance: ${ethers.utils.formatEther(balance)} BNB`);

  // Deploy PropertyTokenBSC contract
  const propertyTokenFactory = await ethers.getContractFactory('PropertyTokenBSC');
  
  const propertyDetails = {
    title: "Luxury Villa #1",
    description: "Luxury beachfront villa in Phuket",
    propertyAddress: "123 Beach Road, Phuket, Thailand",
    city: "Phuket",
    state: "Phuket",
    country: "Thailand",
    zipCode: "83000",
    propertyType: "Villa",
    squareFeet: 3500,
    bedrooms: 4,
    bathrooms: 5,
    yearBuilt: 2020,
    rentYield: 600, // 6.00%
    appreciationPotential: 800 // 8.00%
  };
  
  const platformTreasury = process.env.PLATFORM_TREASURY || deployer.address;
  
  const propertyToken = await propertyTokenFactory.deploy(
    "Luxury Villa #1", // Token name
    "VILLA1", // Token symbol
    "prop-villa-1", // Property ID
    "https://yieldhabitat.io/properties/villa-1", // Property URI
    ethers.utils.parseEther("1000"), // 1000 tokens total supply
    ethers.utils.parseEther("0.1"), // 0.1 BNB per token
    deployer.address, // Treasury
    platformTreasury, // Platform treasury
    propertyDetails
  );

  await propertyToken.deployed();
  console.log(`PropertyTokenBSC deployed to: ${propertyToken.address}`);

  // Deploy ChainBridge contract
  const bridgeFactory = await ethers.getContractFactory('ChainBridge');
  const bridge = await bridgeFactory.deploy(platformTreasury);

  await bridge.deployed();
  console.log(`ChainBridge deployed to: ${bridge.address}`);

  // Add token to bridge's supported tokens
  const tx = await bridge.addSupportedToken(
    1, // ChainId.BSC
    "VILLA1",
    propertyToken.address
  );
  await tx.wait();
  console.log('Added PropertyToken to ChainBridge supported tokens');

  console.log('Deployment to BSC complete!');

  // Verify contracts on BSCScan (if not on a local testnet)
  if (process.env.BSCSCAN_API_KEY) {
    console.log('Waiting for block confirmations...');
    await propertyToken.deployTransaction.wait(6);
    await bridge.deployTransaction.wait(6);
    
    console.log('Verifying contracts on BSCScan...');
    try {
      await hre.run('verify:verify', {
        address: propertyToken.address,
        constructorArguments: [
          "Luxury Villa #1", // Token name
          "VILLA1", // Token symbol
          "prop-villa-1", // Property ID
          "https://yieldhabitat.io/properties/villa-1", // Property URI
          ethers.utils.parseEther("1000"), // 1000 tokens total supply
          ethers.utils.parseEther("0.1"), // 0.1 BNB per token
          deployer.address, // Treasury
          platformTreasury, // Platform treasury
          propertyDetails
        ],
      });
      
      await hre.run('verify:verify', {
        address: bridge.address,
        constructorArguments: [platformTreasury],
      });
      
      console.log('Contracts verified on BSCScan');
    } catch (error) {
      console.error('Error verifying contracts:', error);
    }
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 