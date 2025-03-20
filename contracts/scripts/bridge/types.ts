import { ethers } from 'ethers';
import { Connection } from '@solana/web3.js';

/**
 * Bridge transfer status enum
 */
export enum TransferStatus {
  Pending = 0,
  Completed = 1,
  Failed = 2,
  Refunded = 3
}

/**
 * Configuration for a blockchain network
 */
export interface ChainConfig {
  rpcUrl: string;
  chainId?: number;
  bridgeAddress?: string;
  privateKey?: string;
  programId?: string;
  keypair?: string;
  provider?: ethers.providers.JsonRpcProvider | null;
  wallet?: ethers.Wallet | null;
  bridgeContract?: ethers.Contract | null;
  connection?: Connection | null;
}

/**
 * Bridge event data structure
 */
export interface BridgeEvent {
  transferId: string;
  sender: string;
  recipient: string;
  amount: string;
  tokenAddress: string;
  sourceChain: number;
  targetChain: number;
  transactionHash?: string;
}

/**
 * Overall bridge configuration
 */
export interface BridgeConfig {
  ethereum: ChainConfig;
  bsc: ChainConfig;
  polygon: ChainConfig;
  solana: ChainConfig;
  gasLimit?: number;
  confirmations?: number;
  bridgeFee?: number;
}

/**
 * Token mapping interface
 */
export interface TokenMapping {
  symbol: string;
  name: string;
  decimals: number;
  addresses: {
    ethereum?: string;
    bsc?: string;
    polygon?: string;
    solana?: string;
  };
} 