# ERC721/NFT Implementation: Solidity vs ink!

## Overview
A comprehensive Non-Fungible Token (NFT) implementation demonstrating unique token creation, ownership tracking, approval mechanisms, and metadata management. This example shows how to handle complex token operations, batch minting, and advanced ERC721 features in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleNFT
/// @notice A simple NFT contract implementing basic ERC721 functionality
/// @dev Demonstrates NFT minting, transfers, and metadata without external dependencies
contract SimpleNFT {
    // Token name and symbol
    string public name;
    string public symbol;
    
    // State variables
    uint256 public totalSupply;
    uint256 public nextTokenId;
    address public owner;
    string public baseURI;
    
    // Mappings
    mapping(uint256 => address) public ownerOf;
    mapping(address => uint256) public balanceOf;
    mapping(uint256 => address) public getApproved;
    mapping(address => mapping(address => bool)) public isApprovedForAll;
    mapping(uint256 => string) public tokenURI;
    
    // Events
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event TokenMinted(address indexed to, uint256 indexed tokenId, string tokenURI);
    event BaseURIChanged(string newBaseURI);
    
    // Custom errors
    error TokenDoesNotExist(uint256 tokenId);
    error NotAuthorized(address caller);
    error InvalidRecipient(address to);
    error TokenAlreadyExists(uint256 tokenId);
    
    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }
    
    modifier tokenExists(uint256 tokenId) {
        if (ownerOf[tokenId] == address(0)) {
            revert TokenDoesNotExist(tokenId);
        }
        _;
    }
    
    /// @notice Constructor sets name, symbol, and owner
    /// @param _name Token name
    /// @param _symbol Token symbol
    /// @param _baseURI Base URI for token metadata
    constructor(string memory _name, string memory _symbol, string memory _baseURI) {
        name = _name;
        symbol = _symbol;
        baseURI = _baseURI;
        owner = msg.sender;
        nextTokenId = 1;
    }
    
    /// @notice Mint a new NFT
    /// @param to Recipient address
    /// @param uri Token URI for metadata
    /// @return tokenId The ID of the newly minted token
    function mint(address to, string memory uri) public onlyOwner returns (uint256) {
        if (to == address(0)) {
            revert InvalidRecipient(to);
        }
        
        uint256 tokenId = nextTokenId;
        nextTokenId++;
        totalSupply++;
        
        ownerOf[tokenId] = to;
        balanceOf[to]++;
        tokenURI[tokenId] = uri;
        
        emit Transfer(address(0), to, tokenId);
        emit TokenMinted(to, tokenId, uri);
        
        return tokenId;
    }
    
    /// @notice Batch mint multiple NFTs
    /// @param to Recipient address
    /// @param uris Array of token URIs
    /// @return tokenIds Array of newly minted token IDs
    function batchMint(address to, string[] memory uris) public onlyOwner returns (uint256[] memory) {
        if (to == address(0)) {
            revert InvalidRecipient(to);
        }
        
        uint256[] memory tokenIds = new uint256[](uris.length);
        
        for (uint256 i = 0; i < uris.length; i++) {
            uint256 tokenId = nextTokenId;
            nextTokenId++;
            totalSupply++;
            
            ownerOf[tokenId] = to;
            tokenURI[tokenId] = uris[i];
            tokenIds[i] = tokenId;
            
            emit Transfer(address(0), to, tokenId);
            emit TokenMinted(to, tokenId, uris[i]);
        }
        
        balanceOf[to] += uris.length;
        
        return tokenIds;
    }
    
    /// @notice Transfer token from one address to another
    /// @param from Current owner
    /// @param to New owner
    /// @param tokenId Token ID to transfer
    function transferFrom(address from, address to, uint256 tokenId) public tokenExists(tokenId) {
        if (to == address(0)) {
            revert InvalidRecipient(to);
        }
        
        address tokenOwner = ownerOf[tokenId];
        if (from != tokenOwner) {
            revert NotAuthorized(msg.sender);
        }
        
        if (msg.sender != tokenOwner && 
            msg.sender != getApproved[tokenId] && 
            !isApprovedForAll[tokenOwner][msg.sender]) {
            revert NotAuthorized(msg.sender);
        }
        
        // Clear approval
        getApproved[tokenId] = address(0);
        
        // Transfer token
        ownerOf[tokenId] = to;
        balanceOf[from]--;
        balanceOf[to]++;
        
        emit Transfer(from, to, tokenId);
    }
    
    /// @notice Approve another address to transfer a specific token
    /// @param to Address to approve
    /// @param tokenId Token ID to approve
    function approve(address to, uint256 tokenId) public tokenExists(tokenId) {
        address tokenOwner = ownerOf[tokenId];
        if (msg.sender != tokenOwner && !isApprovedForAll[tokenOwner][msg.sender]) {
            revert NotAuthorized(msg.sender);
        }
        
        getApproved[tokenId] = to;
        emit Approval(tokenOwner, to, tokenId);
    }
    
    /// @notice Set approval for all tokens
    /// @param operator Address to set approval for
    /// @param approved Whether to approve or revoke approval
    function setApprovalForAll(address operator, bool approved) public {
        isApprovedForAll[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }
    
    /// @notice Burn a token
    /// @param tokenId Token ID to burn
    function burn(uint256 tokenId) public tokenExists(tokenId) {
        address tokenOwner = ownerOf[tokenId];
        if (msg.sender != tokenOwner && msg.sender != owner) {
            revert NotAuthorized(msg.sender);
        }
        
        // Clear approval
        getApproved[tokenId] = address(0);
        
        // Update balances
        balanceOf[tokenOwner]--;
        totalSupply--;
        
        // Remove token
        ownerOf[tokenId] = address(0);
        tokenURI[tokenId] = "";
        
        emit Transfer(tokenOwner, address(0), tokenId);
    }
    
    /// @notice Get token URI
    /// @param tokenId Token ID
    /// @return The token URI
    function getTokenURI(uint256 tokenId) public view tokenExists(tokenId) returns (string memory) {
        return tokenURI[tokenId];
    }
    
    /// @notice Get tokens owned by an address
    /// @param ownerAddr Address to query
    /// @return Array of token IDs owned by the address
    function tokensOfOwner(address ownerAddr) public view returns (uint256[] memory) {
        uint256 tokenCount = balanceOf[ownerAddr];
        uint256[] memory tokens = new uint256[](tokenCount);
        uint256 index = 0;
        
        for (uint256 i = 1; i < nextTokenId && index < tokenCount; i++) {
            if (ownerOf[i] == ownerAddr) {
                tokens[index] = i;
                index++;
            }
        }
        
        return tokens;
    }
    
    /// @notice Check if contract supports an interface
    /// @param interfaceId Interface ID to check
    /// @return True if interface is supported
    function supportsInterface(bytes4 interfaceId) public pure returns (bool) {
        return
            interfaceId == 0x80ac58cd || // ERC721
            interfaceId == 0x5b5e139f || // ERC721Metadata
            interfaceId == 0x01ffc9a7;   // ERC165
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc721 {
    use ink::{
        prelude::{
            string::String,
            vec::Vec,
        },
        storage::Mapping,
    };

    /// A token ID type
    pub type TokenId = u32;

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct Erc721 {
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
        /// Base URI for token metadata
        base_uri: String,
        /// Contract owner
        owner: AccountId,
        /// Next token ID to mint
        next_token_id: TokenId,
        /// Total supply of tokens
        total_supply: u32,
        /// Mapping from token ID to owner
        token_owner: Mapping<TokenId, AccountId>,
        /// Mapping from token ID to approved account
        token_approvals: Mapping<TokenId, AccountId>,
        /// Mapping from owner to number of owned tokens
        owned_tokens_count: Mapping<AccountId, u32>,
        /// Mapping from owner to operator approvals
        operator_approvals: Mapping<(AccountId, AccountId), ()>,
        /// Mapping from token ID to token URI
        token_uris: Mapping<TokenId, String>,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: TokenId,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: TokenId,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(event)]
    pub struct TokenMinted {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: TokenId,
        token_uri: String,
    }

    #[ink(event)]
    pub struct BaseURIChanged {
        new_base_uri: String,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        NotAllowed,
        InvalidRecipient,
        NotAuthorized,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc721 {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, base_uri: String) -> Self {
            let caller = Self::env().caller();
            Self {
                name,
                symbol,
                base_uri,
                owner: caller,
                next_token_id: 1,
                total_supply: 0,
                token_owner: Mapping::default(),
                token_approvals: Mapping::default(),
                owned_tokens_count: Mapping::default(),
                operator_approvals: Mapping::default(),
                token_uris: Mapping::default(),
            }
        }

        /// Get token name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Get token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Get total supply
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Returns the balance of the owner (number of tokens owned)
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.owned_tokens_count.get(owner).unwrap_or(0)
        }

        /// Returns the owner of the token
        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owner.get(id)
        }

        /// Returns the approved account ID for this token if any
        #[ink(message)]
        pub fn get_approved(&self, id: TokenId) -> Option<AccountId> {
            self.token_approvals.get(id)
        }

        /// Returns `true` if the operator is approved by the owner
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.operator_approvals.contains((owner, operator))
        }

        /// Mint a new NFT
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, uri: String) -> Result<TokenId> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let token_id = self.next_token_id;
            self.next_token_id += 1;
            self.total_supply += 1;

            // Check if token already exists
            if self.token_owner.contains(token_id) {
                return Err(Error::TokenExists);
            }

            // Set token owner
            self.token_owner.insert(token_id, &to);
            self.token_uris.insert(token_id, &uri);

            // Update owner's token count
            let count = self.owned_tokens_count.get(to).unwrap_or(0);
            self.owned_tokens_count.insert(to, &(count + 1));

            // Emit events
            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                id: token_id,
            });

            self.env().emit_event(TokenMinted {
                to,
                id: token_id,
                token_uri: uri,
            });

            Ok(token_id)
        }

        /// Batch mint multiple NFTs
        #[ink(message)]
        pub fn batch_mint(&mut self, to: AccountId, uris: Vec<String>) -> Result<Vec<TokenId>> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut token_ids = Vec::new();

            for uri in uris {
                let token_id = self.next_token_id;
                self.next_token_id += 1;
                self.total_supply += 1;

                self.token_owner.insert(token_id, &to);
                self.token_uris.insert(token_id, &uri);
                token_ids.push(token_id);

                self.env().emit_event(Transfer {
                    from: None,
                    to: Some(to),
                    id: token_id,
                });

                self.env().emit_event(TokenMinted {
                    to,
                    id: token_id,
                    token_uri: uri,
                });
            }

            // Update owner's token count
            let count = self.owned_tokens_count.get(to).unwrap_or(0);
            self.owned_tokens_count.insert(to, &(count + token_ids.len() as u32));

            Ok(token_ids)
        }

        /// Transfer token from one address to another
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;

            // Check if caller is authorized
            if !self.approved_or_owner(caller, id, owner) {
                return Err(Error::NotApproved);
            }

            // Check if from is the actual owner
            if owner != from {
                return Err(Error::NotOwner);
            }

            // Clear approval
            self.token_approvals.remove(id);

            // Update token counts
            let from_count = self.owned_tokens_count.get(from).unwrap_or(0);
            self.owned_tokens_count.insert(from, &(from_count - 1));

            let to_count = self.owned_tokens_count.get(to).unwrap_or(0);
            self.owned_tokens_count.insert(to, &(to_count + 1));

            // Transfer ownership
            self.token_owner.insert(id, &to);

            // Emit event
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                id,
            });

            Ok(())
        }

        /// Transfer token (caller must be owner)
        #[ink(message)]
        pub fn transfer(&mut self, destination: AccountId, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            self.transfer_from(caller, destination, id)
        }

        /// Approve another address to transfer a specific token
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;

            if !(owner == caller || self.is_approved_for_all(owner, caller)) {
                return Err(Error::NotAllowed);
            }

            self.token_approvals.insert(id, &to);

            self.env().emit_event(Approval {
                from: caller,
                to,
                id,
            });

            Ok(())
        }

        /// Set approval for all tokens
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, to: AccountId, approved: bool) -> Result<()> {
            let caller = self.env().caller();

            if to == caller {
                return Err(Error::NotAllowed);
            }

            if approved {
                self.operator_approvals.insert((caller, to), &());
            } else {
                self.operator_approvals.remove((caller, to));
            }

            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator: to,
                approved,
            });

            Ok(())
        }

        /// Burn a token
        #[ink(message)]
        pub fn burn(&mut self, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;

            // Only owner or contract owner can burn
            if owner != caller && self.owner != caller {
                return Err(Error::NotOwner);
            }

            // Clear approval
            self.token_approvals.remove(id);

            // Update token count
            let count = self.owned_tokens_count.get(owner).unwrap_or(0);
            self.owned_tokens_count.insert(owner, &(count - 1));

            // Remove token
            self.token_owner.remove(id);
            self.token_uris.remove(id);

            // Update total supply
            self.total_supply -= 1;

            // Emit event
            self.env().emit_event(Transfer {
                from: Some(owner),
                to: None,
                id,
            });

            Ok(())
        }

        /// Get token URI
        #[ink(message)]
        pub fn token_uri(&self, id: TokenId) -> Option<String> {
            if !self.token_owner.contains(id) {
                return None;
            }
            self.token_uris.get(id)
        }

        /// Set base URI (owner only)
        #[ink(message)]
        pub fn set_base_uri(&mut self, new_base_uri: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.base_uri = new_base_uri.clone();

            self.env().emit_event(BaseURIChanged {
                new_base_uri,
            });

            Ok(())
        }

        /// Get tokens owned by an address
        #[ink(message)]
        pub fn tokens_of_owner(&self, owner: AccountId) -> Vec<TokenId> {
            let mut tokens = Vec::new();
            
            for token_id in 1..self.next_token_id {
                if let Some(token_owner) = self.token_owner.get(token_id) {
                    if token_owner == owner {
                        tokens.push(token_id);
                    }
                }
            }
            
            tokens
        }

        /// Check if a token exists
        #[ink(message)]
        pub fn exists(&self, id: TokenId) -> bool {
            self.token_owner.contains(id)
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Transfer ownership of the contract
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.owner = new_owner;
            Ok(())
        }

        /// Internal helper: Check if account is approved or owner
        fn approved_or_owner(&self, account: AccountId, id: TokenId, owner: AccountId) -> bool {
            account == owner
                || self.token_approvals.get(id) == Some(account)
                || self.is_approved_for_all(owner, account)
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            assert_eq!(contract.name(), "Test NFT");
            assert_eq!(contract.symbol(), "TNFT");
            assert_eq!(contract.total_supply(), 0);
        }

        #[ink::test]
        fn mint_works() {
            let mut contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.mint(accounts.bob, "token1".to_string()), Ok(1));
            assert_eq!(contract.owner_of(1), Some(accounts.bob));
            assert_eq!(contract.balance_of(accounts.bob), 1);
            assert_eq!(contract.total_supply(), 1);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Mint token
            assert_eq!(contract.mint(accounts.alice, "token1".to_string()), Ok(1));

            // Transfer
            assert_eq!(contract.transfer_from(accounts.alice, accounts.bob, 1), Ok(()));
            assert_eq!(contract.owner_of(1), Some(accounts.bob));
            assert_eq!(contract.balance_of(accounts.alice), 0);
            assert_eq!(contract.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn approve_works() {
            let mut contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Mint token
            assert_eq!(contract.mint(accounts.alice, "token1".to_string()), Ok(1));

            // Approve
            assert_eq!(contract.approve(accounts.bob, 1), Ok(()));
            assert_eq!(contract.get_approved(1), Some(accounts.bob));

            // Approved transfer
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.transfer_from(accounts.alice, accounts.charlie, 1), Ok(()));
            assert_eq!(contract.owner_of(1), Some(accounts.charlie));
        }

        #[ink::test]
        fn batch_mint_works() {
            let mut contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let uris = vec!["token1".to_string(), "token2".to_string()];
            let token_ids = contract.batch_mint(accounts.bob, uris).unwrap();

            assert_eq!(token_ids, vec![1, 2]);
            assert_eq!(contract.balance_of(accounts.bob), 2);
            assert_eq!(contract.total_supply(), 2);
        }

        #[ink::test]
        fn burn_works() {
            let mut contract = Erc721::new(
                "Test NFT".to_string(),
                "TNFT".to_string(),
                "https://example.com/".to_string(),
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Mint and burn
            assert_eq!(contract.mint(accounts.alice, "token1".to_string()), Ok(1));
            assert_eq!(contract.burn(1), Ok(()));
            assert_eq!(contract.owner_of(1), None);
            assert_eq!(contract.total_supply(), 0);
        }
    }
}
```

## Key Migration Points

### 1. Token ID Management
**Solidity:**
- Uses `uint256` for token IDs
- Manual tracking with `nextTokenId`
- Automatic incrementation

**ink!:**
- Uses `u32` for token IDs (more efficient)
- Custom `TokenId` type alias
- Manual tracking with `next_token_id`

### 2. Storage Structure
**Solidity:**
```solidity
mapping(uint256 => address) public ownerOf;
mapping(address => uint256) public balanceOf;
mapping(uint256 => address) public getApproved;
mapping(address => mapping(address => bool)) public isApprovedForAll;
```

**ink!:**
```rust
token_owner: Mapping<TokenId, AccountId>,
owned_tokens_count: Mapping<AccountId, u32>,
token_approvals: Mapping<TokenId, AccountId>,
operator_approvals: Mapping<(AccountId, AccountId), ()>,
```

### 3. Approval Mechanisms
**Solidity:**
```solidity
function approve(address to, uint256 tokenId) public {
    address tokenOwner = ownerOf[tokenId];
    require(msg.sender == tokenOwner || isApprovedForAll[tokenOwner][msg.sender]);
    getApproved[tokenId] = to;
    emit Approval(tokenOwner, to, tokenId);
}
```

**ink!:**
```rust
#[ink(message)]
pub fn approve(&mut self, to: AccountId, id: TokenId) -> Result<()> {
    let caller = self.env().caller();
    let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;
    
    if !(owner == caller || self.is_approved_for_all(owner, caller)) {
        return Err(Error::NotAllowed);
    }
    
    self.token_approvals.insert(id, &to);
    Ok(())
}
```

### 4. Transfer Logic
**Solidity:**
```solidity
function transferFrom(address from, address to, uint256 tokenId) public {
    require(to != address(0));
    require(ownerOf[tokenId] == from);
    require(msg.sender == from || getApproved[tokenId] == msg.sender || isApprovedForAll[from][msg.sender]);
    
    getApproved[tokenId] = address(0);
    ownerOf[tokenId] = to;
    balanceOf[from]--;
    balanceOf[to]++;
    
    emit Transfer(from, to, tokenId);
}
```

**ink!:**
```rust
#[ink(message)]
pub fn transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId) -> Result<()> {
    let caller = self.env().caller();
    let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;
    
    if !self.approved_or_owner(caller, id, owner) {
        return Err(Error::NotApproved);
    }
    
    if owner != from {
        return Err(Error::NotOwner);
    }
    
    self.token_approvals.remove(id);
    self.token_owner.insert(id, &to);
    
    // Update balances...
    Ok(())
}
```

### 5. Metadata Handling
**Solidity:**
```solidity
mapping(uint256 => string) public tokenURI;

function getTokenURI(uint256 tokenId) public view returns (string memory) {
    require(ownerOf[tokenId] != address(0));
    return tokenURI[tokenId];
}
```

**ink!:**
```rust
token_uris: Mapping<TokenId, String>,

#[ink(message)]
pub fn token_uri(&self, id: TokenId) -> Option<String> {
    if !self.token_owner.contains(id) {
        return None;
    }
    self.token_uris.get(id)
}
```

## Migration Steps

### Step 1: Set Up Token Structure
1. Define `TokenId` type (use `u32` instead of `uint256`)
2. Create storage struct with all mappings
3. Add metadata fields (name, symbol, base_uri)

### Step 2: Implement Core ERC721 Functions
1. Convert `ownerOf` to `owner_of` with `Option<AccountId>` return
2. Implement `balance_of` using mapping lookup
3. Add approval mechanisms with proper error handling

### Step 3: Handle Token Operations
1. Implement minting with proper event emission
2. Add transfer logic with authorization checks
3. Implement burning with cleanup

### Step 4: Add Advanced Features
1. Batch operations for efficiency
2. Token enumeration functions
3. Metadata management

### Step 5: Error Handling and Events
1. Define comprehensive error enum
2. Emit appropriate events for all operations
3. Add proper authorization checks

## Common Patterns

### Authorization Checks
```rust
fn approved_or_owner(&self, account: AccountId, id: TokenId, owner: AccountId) -> bool {
    account == owner
        || self.token_approvals.get(id) == Some(account)
        || self.is_approved_for_all(owner, account)
}
```

### Safe Token Operations
```rust
// Always check token exists
let owner = self.token_owner.get(id).ok_or(Error::TokenNotFound)?;

// Update counts safely
let count = self.owned_tokens_count.get(owner).unwrap_or(0);
self.owned_tokens_count.insert(owner, &(count + 1));
```

### Batch Operations
```rust
#[ink(message)]
pub fn batch_mint(&mut self, to: AccountId, uris: Vec<String>) -> Result<Vec<TokenId>> {
    let mut token_ids = Vec::new();
    
    for uri in uris {
        let token_id = self.mint_single(to, uri)?;
        token_ids.push(token_id);
    }
    
    Ok(token_ids)
}
```

## Best Practices

### 1. Use Appropriate Data Types
- `u32` for token IDs (sufficient for most use cases)
- `AccountId` for addresses
- `Option<T>` for nullable values

### 2. Error Handling
- Return `Result<T, Error>` for all operations
- Use descriptive error variants
- Check token existence before operations

### 3. Event Emission
- Use `#[ink(topic)]` for searchable fields
- Emit events for all state changes
- Follow ERC721 event standards

### 4. Storage Efficiency
- Use tuple keys for nested mappings
- Consider gas costs for enumeration functions
- Clean up storage when burning tokens

### 5. Security Considerations
- Always validate ownership and approvals
- Check for zero addresses
- Prevent unauthorized operations

This migration demonstrates how Solidity's ERC721 implementation translates to ink!'s more type-safe and efficient approach, with better error handling and more flexible storage patterns.