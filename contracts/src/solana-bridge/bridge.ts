import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  SystemProgram,
} from '@solana/web3.js';
import * as borsh from 'borsh';
import BN from 'bn.js';

// Define instruction types for the bridge program
export enum BridgeInstruction {
  Initialize = 0,
  Deposit = 1,
  Withdraw = 2,
  UpdateTransferStatus = 3,
}

// Borsh schema for serializing/deserializing bridge instructions
class InitializeArgs {
  constructor(public authority: Uint8Array) {}
}

class DepositArgs {
  constructor(
    public amount: BN,
    public targetChain: number,
    public targetAddress: Uint8Array
  ) {}
}

class WithdrawArgs {
  constructor(
    public amount: BN,
    public sourceChain: number,
    public transferId: BN,
    public signature: Uint8Array
  ) {}
}

class UpdateTransferStatusArgs {
  constructor(
    public transferId: BN,
    public status: number,
    public txHash: Uint8Array
  ) {}
}

// Serialize instruction data
function serializeInitializeArgs(args: InitializeArgs): Buffer {
  const schema = new Map([
    [
      InitializeArgs,
      {
        kind: 'struct',
        fields: [['authority', [32]]],
      },
    ],
  ]);
  
  const buffer = borsh.serialize(schema, args);
  const instructionData = Buffer.alloc(buffer.length + 1);
  instructionData.writeUInt8(BridgeInstruction.Initialize, 0);
  buffer.copy(instructionData, 1);
  
  return instructionData;
}

function serializeDepositArgs(args: DepositArgs): Buffer {
  const schema = new Map([
    [
      DepositArgs,
      {
        kind: 'struct',
        fields: [
          ['amount', 'u64'],
          ['targetChain', 'u8'],
          ['targetAddress', [32]],
        ],
      },
    ],
  ]);
  
  const buffer = borsh.serialize(schema, args);
  const instructionData = Buffer.alloc(buffer.length + 1);
  instructionData.writeUInt8(BridgeInstruction.Deposit, 0);
  buffer.copy(instructionData, 1);
  
  return instructionData;
}

function serializeWithdrawArgs(args: WithdrawArgs): Buffer {
  const schema = new Map([
    [
      WithdrawArgs,
      {
        kind: 'struct',
        fields: [
          ['amount', 'u64'],
          ['sourceChain', 'u8'],
          ['transferId', 'u64'],
          ['signature', [64]],
        ],
      },
    ],
  ]);
  
  const buffer = borsh.serialize(schema, args);
  const instructionData = Buffer.alloc(buffer.length + 1);
  instructionData.writeUInt8(BridgeInstruction.Withdraw, 0);
  buffer.copy(instructionData, 1);
  
  return instructionData;
}

function serializeUpdateTransferStatusArgs(args: UpdateTransferStatusArgs): Buffer {
  const schema = new Map([
    [
      UpdateTransferStatusArgs,
      {
        kind: 'struct',
        fields: [
          ['transferId', 'u64'],
          ['status', 'u8'],
          ['txHash', [32]],
        ],
      },
    ],
  ]);
  
  const buffer = borsh.serialize(schema, args);
  const instructionData = Buffer.alloc(buffer.length + 1);
  instructionData.writeUInt8(BridgeInstruction.UpdateTransferStatus, 0);
  buffer.copy(instructionData, 1);
  
  return instructionData;
}

/**
 * SolanaBridge class provides methods to interact with the Solana bridge program
 */
export class SolanaBridge {
  constructor(
    private connection: Connection,
    private programId: PublicKey,
    private payer: Keypair
  ) {}
  
  /**
   * Initialize the bridge program with an authority
   */
  async initialize(authority: PublicKey): Promise<string> {
    const bridgeAccountKeypair = Keypair.generate();
    const bridgeAccount = bridgeAccountKeypair.publicKey;
    
    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: this.payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: bridgeAccount, isSigner: true, isWritable: true },
        { pubkey: authority, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data: serializeInitializeArgs(new InitializeArgs(authority.toBuffer())),
    });
    
    const transaction = new Transaction().add(instruction);
    
    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer, bridgeAccountKeypair]
    );
    
    return signature;
  }
  
  /**
   * Deposit tokens to be bridged to another chain
   */
  async deposit(
    bridgeAccount: PublicKey,
    tokenAccount: PublicKey,
    amount: BN,
    targetChain: number,
    targetAddress: Uint8Array
  ): Promise<string> {
    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: this.payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: bridgeAccount, isSigner: false, isWritable: true },
        { pubkey: tokenAccount, isSigner: false, isWritable: true },
      ],
      programId: this.programId,
      data: serializeDepositArgs(new DepositArgs(amount, targetChain, targetAddress)),
    });
    
    const transaction = new Transaction().add(instruction);
    
    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer]
    );
    
    return signature;
  }
  
  /**
   * Withdraw tokens that were bridged from another chain
   */
  async withdraw(
    bridgeAccount: PublicKey,
    tokenAccount: PublicKey,
    recipient: PublicKey,
    amount: BN,
    sourceChain: number,
    transferId: BN,
    signature: Uint8Array
  ): Promise<string> {
    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: this.payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: bridgeAccount, isSigner: false, isWritable: true },
        { pubkey: tokenAccount, isSigner: false, isWritable: true },
        { pubkey: recipient, isSigner: false, isWritable: true },
      ],
      programId: this.programId,
      data: serializeWithdrawArgs(new WithdrawArgs(amount, sourceChain, transferId, signature)),
    });
    
    const transaction = new Transaction().add(instruction);
    
    const txSignature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer]
    );
    
    return txSignature;
  }
  
  /**
   * Update the status of a bridge transfer
   */
  async updateTransferStatus(
    bridgeAccount: PublicKey,
    transferId: BN,
    status: number,
    txHash: Uint8Array
  ): Promise<string> {
    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: this.payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: bridgeAccount, isSigner: false, isWritable: true },
      ],
      programId: this.programId,
      data: serializeUpdateTransferStatusArgs(new UpdateTransferStatusArgs(transferId, status, txHash)),
    });
    
    const transaction = new Transaction().add(instruction);
    
    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer]
    );
    
    return signature;
  }
} 