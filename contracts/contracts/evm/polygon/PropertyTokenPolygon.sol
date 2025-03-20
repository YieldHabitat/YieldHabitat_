// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title PropertyTokenPolygon
 * @dev ERC20 token representing a tokenized real estate property on Polygon
 * Gas-optimized for Polygon network
 */
contract PropertyTokenPolygon is ERC20, ERC20Burnable, Pausable, AccessControl {
    using Counters for Counters.Counter;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    
    // Property details - packed for gas optimization
    string public propertyId;
    string public propertyURI;
    uint256 public tokenPrice;
    uint256 public totalTokenSupply;
    uint256 public availableTokens;
    address public treasury;
    
    enum PropertyStatus { Available, Pending, Sold }
    PropertyStatus public status;
    
    struct PropertyDetails {
        string title;
        string description;
        string propertyAddress; // Changed from 'address' to avoid conflict
        string city;
        string state;
        string country;
        string zipCode;
        string propertyType;
        uint32 squareFeet;
        uint8 bedrooms;
        uint8 bathrooms;
        uint16 yearBuilt;
        uint16 rentYield; // In basis points (e.g., 500 = 5.00%)
        uint16 appreciationPotential; // In basis points
    }
    
    PropertyDetails public details;
    
    // Transaction tracking with gas optimizations
    Counters.Counter private _transactionCounter;
    
    struct Transaction {
        uint128 id; // Reduced from uint256 for gas savings
        address buyer;
        uint128 amount; // Reduced from uint256 for gas savings
        uint128 price; // Reduced from uint256 for gas savings
        uint64 timestamp; // Reduced from uint256 for gas savings
    }
    
    mapping(uint128 => Transaction) public transactions;
    
    // Events
    event PropertyStatusChanged(PropertyStatus previousStatus, PropertyStatus newStatus);
    event TokensPurchased(address indexed buyer, uint128 amount, uint128 price, uint128 transactionId);
    event PropertyDetailsUpdated();
    event TokenPriceUpdated(uint256 oldPrice, uint256 newPrice);
    
    /**
     * @dev Initialize the property token with its details
     */
    constructor(
        string memory name,
        string memory symbol,
        string memory _propertyId,
        string memory _propertyURI,
        uint256 _totalTokens,
        uint256 _tokenPrice,
        address _treasury,
        PropertyDetails memory _details
    ) ERC20(name, symbol) {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        
        propertyId = _propertyId;
        propertyURI = _propertyURI;
        totalTokenSupply = _totalTokens;
        tokenPrice = _tokenPrice;
        availableTokens = _totalTokens;
        status = PropertyStatus.Available;
        treasury = _treasury;
        details = _details;
        
        // Mint all tokens to the treasury
        _mint(treasury, totalTokenSupply);
    }
    
    /**
     * @dev Purchase tokens from the treasury
     * Gas-optimized for Polygon network
     */
    function purchaseTokens(uint128 amount) external payable whenNotPaused {
        require(status == PropertyStatus.Available, "Property not available");
        require(amount <= availableTokens, "Not enough tokens");
        require(msg.value >= uint256(amount) * tokenPrice, "Insufficient payment");
        
        // Transfer tokens from treasury to buyer
        _transfer(treasury, msg.sender, amount);
        
        // Update available tokens
        availableTokens -= amount;
        
        // Update property status if all tokens are sold
        if (availableTokens == 0) {
            PropertyStatus oldStatus = status;
            status = PropertyStatus.Sold;
            emit PropertyStatusChanged(oldStatus, status);
        }
        
        // Record transaction (gas optimized)
        uint128 transactionId = uint128(_transactionCounter.current());
        transactions[transactionId] = Transaction({
            id: transactionId,
            buyer: msg.sender,
            amount: amount,
            price: uint128(tokenPrice),
            timestamp: uint64(block.timestamp)
        });
        _transactionCounter.increment();
        
        emit TokensPurchased(msg.sender, amount, uint128(tokenPrice), transactionId);
        
        // Send funds to treasury
        (bool success, ) = payable(treasury).call{value: msg.value}("");
        require(success, "Transfer to treasury failed");
    }
    
    /**
     * @dev Update property details
     */
    function updatePropertyDetails(PropertyDetails calldata _details) external onlyRole(ADMIN_ROLE) {
        details = _details;
        emit PropertyDetailsUpdated();
    }
    
    /**
     * @dev Update token price
     */
    function updateTokenPrice(uint256 _tokenPrice) external onlyRole(ADMIN_ROLE) {
        uint256 oldPrice = tokenPrice;
        tokenPrice = _tokenPrice;
        emit TokenPriceUpdated(oldPrice, tokenPrice);
    }
    
    /**
     * @dev Update property status
     */
    function updatePropertyStatus(PropertyStatus _status) external onlyRole(ADMIN_ROLE) {
        PropertyStatus oldStatus = status;
        status = _status;
        emit PropertyStatusChanged(oldStatus, status);
    }
    
    /**
     * @dev Pause token transfers
     */
    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }
    
    /**
     * @dev Unpause token transfers
     */
    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }
    
    function _beforeTokenTransfer(address from, address to, uint256 amount)
        internal
        whenNotPaused
        override
    {
        super._beforeTokenTransfer(from, to, amount);
    }
} 