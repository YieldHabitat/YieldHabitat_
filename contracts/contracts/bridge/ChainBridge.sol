// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title ChainBridge
 * @dev Bridge contract for cross-chain property token transfers between Ethereum, BSC, Polygon, and Solana
 */
contract ChainBridge is AccessControl, ReentrancyGuard, Pausable {
    using SafeERC20 for IERC20;
    using ECDSA for bytes32;
    using Counters for Counters.Counter;

    bytes32 public constant RELAYER_ROLE = keccak256("RELAYER_ROLE");
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    
    // Bridge transaction counter
    Counters.Counter private _transferCounter;
    
    // Bridge transaction status
    enum TransferStatus { Pending, Completed, Failed, Refunded }
    
    // Supported chain IDs
    enum ChainId { 
        Ethereum, // 0
        BSC,      // 1
        Polygon,  // 2
        Solana    // 3
    }
    
    // Bridge transaction record
    struct BridgeTransfer {
        uint256 id;
        address sender;
        bytes32 recipient; // Can be address (for EVM) or public key (for Solana)
        uint256 amount;
        address tokenAddress;
        ChainId sourceChain;
        ChainId targetChain;
        TransferStatus status;
        uint256 timestamp;
        bytes32 transactionHash; // Hash of the transaction on target chain once completed
    }
    
    // Mapping to store bridge transactions
    mapping(uint256 => BridgeTransfer) public bridgeTransfers;
    
    // Mapping from chain ID to token address
    mapping(ChainId => mapping(string => address)) public supportedTokens;
    
    // Bridge fee settings
    uint256 public bridgeFee = 50; // 0.5% in basis points
    address public feeCollector;
    
    // Events
    event TokensLocked(
        uint256 indexed transferId,
        address indexed sender,
        bytes32 recipient,
        uint256 amount,
        address tokenAddress,
        ChainId sourceChain,
        ChainId targetChain
    );
    
    event TokensReleased(
        uint256 indexed transferId,
        address indexed recipient,
        uint256 amount,
        address tokenAddress,
        ChainId sourceChain,
        ChainId targetChain,
        bytes32 transactionHash
    );
    
    event BridgeTransferCompleted(
        uint256 indexed transferId,
        TransferStatus status,
        bytes32 transactionHash
    );
    
    event TokenAdded(
        ChainId chainId,
        string symbol,
        address tokenAddress
    );
    
    event BridgeFeeUpdated(
        uint256 oldFee,
        uint256 newFee
    );
    
    /**
     * @dev Initializes the bridge contract
     */
    constructor(address _feeCollector) {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
        _grantRole(RELAYER_ROLE, msg.sender);
        
        feeCollector = _feeCollector;
    }
    
    /**
     * @dev Initiate token transfer from current chain to target chain
     */
    function bridgeTokens(
        bytes32 recipient,
        uint256 amount,
        address tokenAddress,
        ChainId targetChain
    ) external nonReentrant whenNotPaused {
        require(amount > 0, "Amount must be greater than 0");
        require(isSupportedToken(tokenAddress), "Token not supported");
        
        // Determine source chain (current chain)
        ChainId sourceChain = getSourceChain();
        
        // Calculate and apply bridge fee
        uint256 feeAmount = (amount * bridgeFee) / 10000;
        uint256 transferAmount = amount - feeAmount;
        
        // Lock tokens in this contract
        IERC20(tokenAddress).safeTransferFrom(msg.sender, address(this), transferAmount);
        
        // Transfer fee to fee collector
        if (feeAmount > 0) {
            IERC20(tokenAddress).safeTransferFrom(msg.sender, feeCollector, feeAmount);
        }
        
        // Create bridge transfer record
        uint256 transferId = _transferCounter.current();
        bridgeTransfers[transferId] = BridgeTransfer({
            id: transferId,
            sender: msg.sender,
            recipient: recipient,
            amount: transferAmount,
            tokenAddress: tokenAddress,
            sourceChain: sourceChain,
            targetChain: targetChain,
            status: TransferStatus.Pending,
            timestamp: block.timestamp,
            transactionHash: bytes32(0)
        });
        _transferCounter.increment();
        
        emit TokensLocked(
            transferId,
            msg.sender,
            recipient,
            transferAmount,
            tokenAddress,
            sourceChain,
            targetChain
        );
    }
    
    /**
     * @dev Complete token transfer on target chain (only called by relayer)
     */
    function releaseTokens(
        uint256 transferId,
        address recipient,
        uint256 amount,
        address tokenAddress,
        ChainId sourceChain,
        bytes32 transactionHash,
        bytes calldata signature
    ) external nonReentrant whenNotPaused onlyRole(RELAYER_ROLE) {
        // Verify the release request
        bytes32 messageHash = keccak256(abi.encodePacked(
            transferId,
            recipient,
            amount,
            tokenAddress,
            sourceChain,
            getSourceChain()
        ));
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        address signer = ethSignedMessageHash.recover(signature);
        
        require(hasRole(RELAYER_ROLE, signer), "Invalid signature");
        
        // Release tokens on this chain
        IERC20(tokenAddress).safeTransfer(recipient, amount);
        
        emit TokensReleased(
            transferId,
            recipient,
            amount,
            tokenAddress,
            sourceChain,
            getSourceChain(),
            transactionHash
        );
    }
    
    /**
     * @dev Update bridge transfer status (only called by relayer)
     */
    function updateBridgeTransferStatus(
        uint256 transferId,
        TransferStatus status,
        bytes32 transactionHash
    ) external onlyRole(RELAYER_ROLE) {
        require(transferId < _transferCounter.current(), "Invalid transfer ID");
        require(bridgeTransfers[transferId].status == TransferStatus.Pending, "Transfer not pending");
        
        bridgeTransfers[transferId].status = status;
        bridgeTransfers[transferId].transactionHash = transactionHash;
        
        emit BridgeTransferCompleted(transferId, status, transactionHash);
    }
    
    /**
     * @dev Add supported token for a specific chain
     */
    function addSupportedToken(
        ChainId chainId,
        string calldata symbol,
        address tokenAddress
    ) external onlyRole(ADMIN_ROLE) {
        require(tokenAddress != address(0), "Invalid token address");
        supportedTokens[chainId][symbol] = tokenAddress;
        
        emit TokenAdded(chainId, symbol, tokenAddress);
    }
    
    /**
     * @dev Update bridge fee
     */
    function updateBridgeFee(uint256 newFee) external onlyRole(ADMIN_ROLE) {
        require(newFee <= 500, "Fee too high"); // Max 5%
        
        uint256 oldFee = bridgeFee;
        bridgeFee = newFee;
        
        emit BridgeFeeUpdated(oldFee, newFee);
    }
    
    /**
     * @dev Update fee collector address
     */
    function updateFeeCollector(address _feeCollector) external onlyRole(ADMIN_ROLE) {
        require(_feeCollector != address(0), "Invalid address");
        feeCollector = _feeCollector;
    }
    
    /**
     * @dev Pause the bridge
     */
    function pause() external onlyRole(ADMIN_ROLE) {
        _pause();
    }
    
    /**
     * @dev Unpause the bridge
     */
    function unpause() external onlyRole(ADMIN_ROLE) {
        _unpause();
    }
    
    /**
     * @dev Check if token is supported
     */
    function isSupportedToken(address tokenAddress) public view returns (bool) {
        ChainId sourceChain = getSourceChain();
        
        // Check each supported token on this chain
        string[3] memory symbols = ["PROP", "YHB", "HABITAT"];
        for (uint i = 0; i < symbols.length; i++) {
            if (supportedTokens[sourceChain][symbols[i]] == tokenAddress) {
                return true;
            }
        }
        
        return false;
    }
    
    /**
     * @dev Get current chain ID
     */
    function getSourceChain() public view returns (ChainId) {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        
        if (chainId == 1 || chainId == 5 || chainId == 11155111) {
            return ChainId.Ethereum;
        } else if (chainId == 56 || chainId == 97) {
            return ChainId.BSC;
        } else if (chainId == 137 || chainId == 80001) {
            return ChainId.Polygon;
        } else {
            revert("Unsupported chain");
        }
    }
} 