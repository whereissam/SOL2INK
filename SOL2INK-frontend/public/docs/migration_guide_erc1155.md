# ERC1155 Implementation: Solidity vs ink!

## Overview
A comprehensive ERC1155 multi-token implementation demonstrating fungible and non-fungible token management in a single contract. This example shows how to implement batch transfers, operator approvals, and safe transfer mechanisms with proper token receiver handling in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleERC1155
/// @notice A simple ERC1155 multi-token implementation similar to ink! ERC1155 example
/// @dev This contract implements basic ERC1155 functionality for both fungible and non-fungible tokens
contract SimpleERC1155 {
    // Token ID counter for creating new tokens
    uint256 private _tokenIdCounter;
    
    // Mapping from token ID to account balances
    mapping(uint256 => mapping(address => uint256)) private _balances;
    
    // Mapping from account to operator approvals
    mapping(address => mapping(address => bool)) private _operatorApprovals;
    
    // Events
    event TransferSingle(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256 id,
        uint256 value
    );
    
    event TransferBatch(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256[] ids,
        uint256[] values
    );
    
    event ApprovalForAll(
        address indexed account,
        address indexed operator,
        bool approved
    );
    
    event URI(string value, uint256 indexed id);
    
    // Custom errors
    error ZeroAddressTransfer();
    error NotApproved();
    error InsufficientBalance(address account, uint256 id, uint256 requested, uint256 available);
    error SelfApproval();
    error BatchTransferMismatch();
    error UnexistentToken(uint256 id);
    error ERC1155ReceiverRejected();
    error ERC1155ReceiverNotImplemented();
    
    /// @notice Create a new token type with the given initial supply
    /// @param value Initial supply of the new token
    /// @return tokenId The ID of the newly created token
    function create(uint256 value) public returns (uint256 tokenId) {
        tokenId = _tokenIdCounter++;
        _balances[tokenId][msg.sender] = value;
        
        emit TransferSingle(msg.sender, address(0), msg.sender, tokenId, value);
        return tokenId;
    }
    
    /// @notice Mint additional tokens of an existing type
    /// @param id The token ID to mint
    /// @param value The amount to mint
    function mint(uint256 id, uint256 value) public {
        if (!exists(id)) {
            revert UnexistentToken(id);
        }
        
        _balances[id][msg.sender] += value;
        emit TransferSingle(msg.sender, address(0), msg.sender, id, value);
    }
    
    /// @notice Get the balance of an account for a specific token
    /// @param account The account to query
    /// @param id The token ID to query
    /// @return The balance of the account for the token
    function balanceOf(address account, uint256 id) public view returns (uint256) {
        return _balances[id][account];
    }
    
    /// @notice Get the balance of multiple accounts for multiple tokens
    /// @param accounts Array of account addresses
    /// @param ids Array of token IDs
    /// @return Array of balances
    function balanceOfBatch(
        address[] calldata accounts,
        uint256[] calldata ids
    ) public view returns (uint256[] memory) {
        require(accounts.length == ids.length, "Accounts and IDs length mismatch");
        
        uint256[] memory batchBalances = new uint256[](accounts.length);
        
        for (uint256 i = 0; i < accounts.length; i++) {
            batchBalances[i] = balanceOf(accounts[i], ids[i]);
        }
        
        return batchBalances;
    }
    
    /// @notice Set or unset the approval of an operator for the caller
    /// @param operator Address to set approval for
    /// @param approved True to approve, false to revoke
    function setApprovalForAll(address operator, bool approved) public {
        if (operator == msg.sender) {
            revert SelfApproval();
        }
        
        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }
    
    /// @notice Check if an operator is approved for all tokens of an owner
    /// @param account The owner address
    /// @param operator The operator address
    /// @return True if the operator is approved
    function isApprovedForAll(address account, address operator) public view returns (bool) {
        return _operatorApprovals[account][operator];
    }
    
    /// @notice Safely transfer a token from one account to another
    /// @param from The sender address
    /// @param to The recipient address
    /// @param id The token ID to transfer
    /// @param value The amount to transfer
    /// @param data Additional data with no specified format
    function safeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes calldata data
    ) public {
        if (to == address(0)) {
            revert ZeroAddressTransfer();
        }
        
        if (from != msg.sender && !isApprovedForAll(from, msg.sender)) {
            revert NotApproved();
        }
        
        uint256 fromBalance = _balances[id][from];
        if (fromBalance < value) {
            revert InsufficientBalance(from, id, value, fromBalance);
        }
        
        _balances[id][from] = fromBalance - value;
        _balances[id][to] += value;
        
        emit TransferSingle(msg.sender, from, to, id, value);
        
        _doSafeTransferAcceptanceCheck(msg.sender, from, to, id, value, data);
    }
    
    /// @notice Safely transfer multiple tokens from one account to another
    /// @param from The sender address
    /// @param to The recipient address
    /// @param ids Array of token IDs to transfer
    /// @param values Array of amounts to transfer
    /// @param data Additional data with no specified format
    function safeBatchTransferFrom(
        address from,
        address to,
        uint256[] calldata ids,
        uint256[] calldata values,
        bytes calldata data
    ) public {
        if (to == address(0)) {
            revert ZeroAddressTransfer();
        }
        
        if (ids.length != values.length) {
            revert BatchTransferMismatch();
        }
        
        if (from != msg.sender && !isApprovedForAll(from, msg.sender)) {
            revert NotApproved();
        }
        
        for (uint256 i = 0; i < ids.length; i++) {
            uint256 id = ids[i];
            uint256 value = values[i];
            
            uint256 fromBalance = _balances[id][from];
            if (fromBalance < value) {
                revert InsufficientBalance(from, id, value, fromBalance);
            }
            
            _balances[id][from] = fromBalance - value;
            _balances[id][to] += value;
        }
        
        emit TransferBatch(msg.sender, from, to, ids, values);
        
        _doSafeBatchTransferAcceptanceCheck(msg.sender, from, to, ids, values, data);
    }
    
    /// @notice Check if a token exists
    /// @param id The token ID to check
    /// @return True if the token exists
    function exists(uint256 id) public view returns (bool) {
        return id < _tokenIdCounter;
    }
    
    /// @notice Get the current token ID counter
    /// @return The next token ID that will be created
    function getTokenIdCounter() public view returns (uint256) {
        return _tokenIdCounter;
    }
    
    /// @notice Get the URI for a token
    /// @param id The token ID
    /// @return The URI string
    function uri(uint256 id) public view returns (string memory) {
        if (!exists(id)) {
            revert UnexistentToken(id);
        }
        return string(abi.encodePacked("https://example.com/token/", _toString(id)));
    }
    
    /// @notice Check if this contract implements an interface
    /// @param interfaceId The interface identifier
    /// @return True if the interface is supported
    function supportsInterface(bytes4 interfaceId) public pure returns (bool) {
        return
            interfaceId == type(IERC1155).interfaceId ||
            interfaceId == type(IERC1155MetadataURI).interfaceId ||
            interfaceId == 0x01ffc9a7; // ERC165 interface ID
    }
    
    /// @dev Internal function to handle safe transfer acceptance check
    function _doSafeTransferAcceptanceCheck(
        address operator,
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) private {
        if (to.code.length > 0) {
            try IERC1155Receiver(to).onERC1155Received(operator, from, id, value, data) returns (bytes4 response) {
                if (response != IERC1155Receiver.onERC1155Received.selector) {
                    revert ERC1155ReceiverRejected();
                }
            } catch {
                revert ERC1155ReceiverNotImplemented();
            }
        }
    }
    
    /// @dev Internal function to handle safe batch transfer acceptance check
    function _doSafeBatchTransferAcceptanceCheck(
        address operator,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) private {
        if (to.code.length > 0) {
            try IERC1155Receiver(to).onERC1155BatchReceived(operator, from, ids, values, data) returns (bytes4 response) {
                if (response != IERC1155Receiver.onERC1155BatchReceived.selector) {
                    revert ERC1155ReceiverRejected();
                }
            } catch {
                revert ERC1155ReceiverNotImplemented();
            }
        }
    }
    
    /// @dev Convert a uint256 to its ASCII string decimal representation
    function _toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};

/// A type representing the unique IDs of tokens managed by this contract.
pub type TokenId = u128;

type Balance = <ink::env::DefaultEnvironment as ink::env::Environment>::Balance;

// This is the return value that we expect if a smart contract supports receiving ERC-1155 tokens.
const ON_ERC_1155_RECEIVED_SELECTOR: [u8; 4] = [0xF2, 0x3A, 0x6E, 0x61];

// This is the return value that we expect if a smart contract supports batch receiving ERC-1155 tokens.
const ON_ERC_1155_BATCH_RECEIVED_SELECTOR: [u8; 4] = [0xBC, 0x19, 0x7C, 0x81];

// The ERC-1155 error types.
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    /// This token ID has not yet been created by the contract.
    UnexistentToken,
    /// The caller tried to sending tokens to the zero-address (`0x00`).
    ZeroAddressTransfer,
    /// The caller is not approved to transfer tokens on behalf of the account.
    NotApproved,
    /// The account does not have enough funds to complete the transfer.
    InsufficientBalance,
    /// An account does not need to approve themselves to transfer tokens.
    SelfApproval,
    /// The number of tokens being transferred does not match the specified number of transfers.
    BatchTransferMismatch,
}

// The ERC-1155 result types.
pub type Result<T> = core::result::Result<T, Error>;

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
macro_rules! ensure {
    ( $condition:expr, $error:expr $(,)? ) => {{
        if !$condition {
            return ::core::result::Result::Err(::core::convert::Into::into($error))
        }
    }};
}

/// The interface for an ERC-1155 compliant contract.
#[ink::trait_definition]
pub trait Erc1155 {
    /// Transfer a `value` amount of `token_id` tokens to the `to` account from the `from` account.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: TokenId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<()>;

    /// Perform a batch transfer of `token_ids` to the `to` account from the `from` account.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_ids: Vec<TokenId>,
        values: Vec<Balance>,
        data: Vec<u8>,
    ) -> Result<()>;

    /// Query the balance of a specific token for the provided account.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId, token_id: TokenId) -> Balance;

    /// Query the balances for a set of tokens for a set of accounts.
    #[ink(message)]
    fn balance_of_batch(
        &self,
        owners: Vec<AccountId>,
        token_ids: Vec<TokenId>,
    ) -> Vec<Balance>;

    /// Enable or disable a third party, known as an `operator`, to control all tokens on behalf of the caller.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<()>;

    /// Query if the given `operator` is allowed to control all of `owner`'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;
}

/// The interface for an ERC-1155 Token Receiver contract.
#[ink::trait_definition]
pub trait Erc1155TokenReceiver {
    /// Handle the receipt of a single ERC-1155 token.
    #[ink(message, selector = 0xF23A6E61)]
    fn on_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        token_id: TokenId,
        value: Balance,
        data: Vec<u8>,
    ) -> Vec<u8>;

    /// Handle the receipt of multiple ERC-1155 tokens.
    #[ink(message, selector = 0xBC197C81)]
    fn on_batch_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        token_ids: Vec<TokenId>,
        values: Vec<Balance>,
        data: Vec<u8>,
    ) -> Vec<u8>;
}

#[ink::contract]
mod erc1155 {
    use super::*;
    use ink::storage::Mapping;

    type Owner = AccountId;
    type Operator = AccountId;

    /// Indicate that a token transfer has occured.
    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        operator: Option<AccountId>,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        token_id: TokenId,
        value: Balance,
    }

    /// Indicate that a batch token transfer has occured.
    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        operator: Option<AccountId>,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        token_ids: Vec<TokenId>,
        values: Vec<Balance>,
    }

    /// Indicate that an approval event has happened.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    /// Indicate that a token's URI has been updated.
    #[ink(event)]
    pub struct Uri {
        value: ink::prelude::string::String,
        #[ink(topic)]
        token_id: TokenId,
    }

    /// An ERC-1155 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {
        /// Tracks the balances of accounts across the different tokens that they might be holding.
        balances: Mapping<(AccountId, TokenId), Balance>,
        /// Which accounts (called operators) have been approved to spend funds on behalf of an owner.
        approvals: Mapping<(Owner, Operator), ()>,
        /// A unique identifier for the tokens which have been minted (and are therefore supported) by this contract.
        token_id_nonce: TokenId,
    }

    impl Contract {
        /// Initialize a default instance of this ERC-1155 implementation.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Create the initial supply for a token.
        ///
        /// The initial supply will be provided to the caller (a.k.a the minter), and the
        /// `token_id` will be assigned by the smart contract.
        #[ink(message)]
        pub fn create(&mut self, value: Balance) -> TokenId {
            let caller = self.env().caller();

            self.token_id_nonce = self.token_id_nonce.checked_add(1).unwrap();
            self.balances.insert((caller, self.token_id_nonce), &value);

            // Emit transfer event but with mint semantics
            self.env().emit_event(TransferSingle {
                operator: Some(caller),
                from: None,
                to: if value == 0 { None } else { Some(caller) },
                token_id: self.token_id_nonce,
                value,
            });

            self.token_id_nonce
        }

        /// Mint a `value` amount of `token_id` tokens.
        ///
        /// It is assumed that the token has already been `create`-ed. The newly minted
        /// supply will be assigned to the caller (a.k.a the minter).
        #[ink(message)]
        pub fn mint(&mut self, token_id: TokenId, value: Balance) -> Result<()> {
            ensure!(token_id <= self.token_id_nonce, Error::UnexistentToken);

            let caller = self.env().caller();
            let current_balance = self.balances.get((caller, token_id)).unwrap_or(0);
            let new_balance = current_balance.checked_add(value).unwrap();
            self.balances.insert((caller, token_id), &new_balance);

            // Emit transfer event but with mint semantics
            self.env().emit_event(TransferSingle {
                operator: Some(caller),
                from: None,
                to: Some(caller),
                token_id,
                value,
            });

            Ok(())
        }

        /// Check if a token exists
        #[ink(message)]
        pub fn exists(&self, token_id: TokenId) -> bool {
            token_id > 0 && token_id <= self.token_id_nonce
        }

        /// Get the current token ID counter
        #[ink(message)]
        pub fn get_token_id_counter(&self) -> TokenId {
            self.token_id_nonce
        }

        /// Get the URI for a token
        #[ink(message)]
        pub fn uri(&self, token_id: TokenId) -> Option<ink::prelude::string::String> {
            if !self.exists(token_id) {
                return None;
            }
            Some(ink::prelude::format!("https://example.com/token/{}", token_id))
        }

        // Helper function for performing single token transfers.
        fn perform_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
        ) {
            let mut sender_balance = self
                .balances
                .get((from, token_id))
                .expect("Caller should have ensured that `from` holds `token_id`.");
            
            sender_balance = sender_balance.checked_sub(value).unwrap();
            self.balances.insert((from, token_id), &sender_balance);

            let mut recipient_balance = self.balances.get((to, token_id)).unwrap_or(0);
            recipient_balance = recipient_balance.checked_add(value).unwrap();
            self.balances.insert((to, token_id), &recipient_balance);

            let caller = self.env().caller();
            self.env().emit_event(TransferSingle {
                operator: Some(caller),
                from: Some(from),
                to: Some(to),
                token_id,
                value,
            });
        }

        // Check if the address at `to` is a smart contract which accepts ERC-1155 token transfers.
        #[cfg_attr(test, allow(unused_variables))]
        fn transfer_acceptance_check(
            &mut self,
            caller: AccountId,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) {
            // This is disabled during tests due to the use of `invoke_contract()` not being supported
            #[cfg(not(test))]
            {
                use ink::env::call::{build_call, ExecutionInput, Selector};

                let result = build_call::<Environment>()
                    .call(to)
                    .ref_time_limit(5000)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ON_ERC_1155_RECEIVED_SELECTOR))
                            .push_arg(caller)
                            .push_arg(from)
                            .push_arg(token_id)
                            .push_arg(value)
                            .push_arg(data),
                    )
                    .returns::<Vec<u8>>()
                    .params()
                    .try_invoke();

                match result {
                    Ok(v) => {
                        assert_eq!(
                            v.clone().expect("Call should be valid"),
                            &ON_ERC_1155_RECEIVED_SELECTOR[..],
                            "The recipient contract does not accept token transfers."
                        );
                    }
                    Err(e) => {
                        use ink::env::ReturnErrorCode;
                        match e {
                            ink::env::Error::ReturnError(
                                ReturnErrorCode::CodeNotFound | ReturnErrorCode::NotCallable,
                            ) => {
                                // Recipient is not a smart contract, so there's nothing more to do
                            }
                            _ => {
                                panic!("Got error while trying to call recipient contract: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    impl super::Erc1155 for Contract {
        #[ink(message)]
        fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != from {
                ensure!(self.is_approved_for_all(from, caller), Error::NotApproved);
            }

            ensure!(to != zero_address(), Error::ZeroAddressTransfer);

            let balance = self.balance_of(from, token_id);
            ensure!(balance >= value, Error::InsufficientBalance);

            self.perform_transfer(from, to, token_id, value);
            self.transfer_acceptance_check(caller, from, to, token_id, value, data);

            Ok(())
        }

        #[ink(message)]
        fn safe_batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_ids: Vec<TokenId>,
            values: Vec<Balance>,
            data: Vec<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != from {
                ensure!(self.is_approved_for_all(from, caller), Error::NotApproved);
            }

            ensure!(to != zero_address(), Error::ZeroAddressTransfer);
            ensure!(!token_ids.is_empty(), Error::BatchTransferMismatch);
            ensure!(token_ids.len() == values.len(), Error::BatchTransferMismatch);

            let transfers = token_ids.iter().zip(values.iter());
            for (&id, &v) in transfers.clone() {
                let balance = self.balance_of(from, id);
                ensure!(balance >= v, Error::InsufficientBalance);
            }

            for (&id, &v) in transfers {
                self.perform_transfer(from, to, id, v);
            }

            // Emit batch transfer event
            self.env().emit_event(TransferBatch {
                operator: Some(caller),
                from: Some(from),
                to: Some(to),
                token_ids,
                values,
            });

            Ok(())
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId, token_id: TokenId) -> Balance {
            self.balances.get((owner, token_id)).unwrap_or(0)
        }

        #[ink(message)]
        fn balance_of_batch(
            &self,
            owners: Vec<AccountId>,
            token_ids: Vec<TokenId>,
        ) -> Vec<Balance> {
            let mut output = Vec::new();
            for o in &owners {
                for t in &token_ids {
                    let amount = self.balance_of(*o, *t);
                    output.push(amount);
                }
            }
            output
        }

        #[ink(message)]
        fn set_approval_for_all(
            &mut self,
            operator: AccountId,
            approved: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            ensure!(operator != caller, Error::SelfApproval);

            if approved {
                self.approvals.insert((&caller, &operator), &());
            } else {
                self.approvals.remove((&caller, &operator));
            }

            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator,
                approved,
            });

            Ok(())
        }

        #[ink(message)]
        fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.approvals.contains((&owner, &operator))
        }
    }

    impl super::Erc1155TokenReceiver for Contract {
        #[ink(message, selector = 0xF23A6E61)]
        fn on_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _token_id: TokenId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Vec<u8> {
            // This smart contract does not accept token transfers
            unimplemented!("This smart contract does not accept token transfers.")
        }

        #[ink(message, selector = 0xBC197C81)]
        fn on_batch_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _token_ids: Vec<TokenId>,
            _values: Vec<Balance>,
            _data: Vec<u8>,
        ) -> Vec<u8> {
            // This smart contract does not accept batch token transfers
            unimplemented!("This smart contract does not accept batch token transfers.")
        }
    }

    /// Helper for referencing the zero address (`0x00`).
    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::Erc1155;

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        fn charlie() -> AccountId {
            default_accounts().charlie
        }

        #[ink::test]
        fn constructor_works() {
            let contract = Contract::new();
            assert_eq!(contract.get_token_id_counter(), 0);
        }

        #[ink::test]
        fn create_token_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id = contract.create(100);
            assert_eq!(token_id, 1);
            assert_eq!(contract.balance_of(alice(), 1), 100);
            assert_eq!(contract.get_token_id_counter(), 1);
        }

        #[ink::test]
        fn mint_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id = contract.create(100);
            assert_eq!(contract.mint(token_id, 50), Ok(()));
            assert_eq!(contract.balance_of(alice(), token_id), 150);
        }

        #[ink::test]
        fn mint_nonexistent_token_fails() {
            let mut contract = Contract::new();
            assert_eq!(contract.mint(999, 50), Err(Error::UnexistentToken));
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id = contract.create(100);
            assert_eq!(contract.safe_transfer_from(alice(), bob(), token_id, 30, vec![]), Ok(()));
            assert_eq!(contract.balance_of(alice(), token_id), 70);
            assert_eq!(contract.balance_of(bob(), token_id), 30);
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id = contract.create(100);
            assert_eq!(
                contract.safe_transfer_from(alice(), bob(), token_id, 150, vec![]),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn batch_transfer_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id1 = contract.create(100);
            let token_id2 = contract.create(200);
            
            assert_eq!(
                contract.safe_batch_transfer_from(
                    alice(), 
                    bob(), 
                    vec![token_id1, token_id2], 
                    vec![30, 50], 
                    vec![]
                ),
                Ok(())
            );
            
            assert_eq!(contract.balance_of(alice(), token_id1), 70);
            assert_eq!(contract.balance_of(alice(), token_id2), 150);
            assert_eq!(contract.balance_of(bob(), token_id1), 30);
            assert_eq!(contract.balance_of(bob(), token_id2), 50);
        }

        #[ink::test]
        fn batch_transfer_length_mismatch_fails() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id1 = contract.create(100);
            let token_id2 = contract.create(200);
            
            assert_eq!(
                contract.safe_batch_transfer_from(
                    alice(), 
                    bob(), 
                    vec![token_id1, token_id2], 
                    vec![30], 
                    vec![]
                ),
                Err(Error::BatchTransferMismatch)
            );
        }

        #[ink::test]
        fn approval_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id = contract.create(100);
            assert_eq!(contract.set_approval_for_all(bob(), true), Ok(()));
            assert!(contract.is_approved_for_all(alice(), bob()));
            
            set_sender(bob());
            assert_eq!(contract.safe_transfer_from(alice(), charlie(), token_id, 30, vec![]), Ok(()));
            assert_eq!(contract.balance_of(alice(), token_id), 70);
            assert_eq!(contract.balance_of(charlie(), token_id), 30);
        }

        #[ink::test]
        fn balance_of_batch_works() {
            let mut contract = Contract::new();
            set_sender(alice());
            
            let token_id1 = contract.create(100);
            let token_id2 = contract.create(200);
            
            let balances = contract.balance_of_batch(vec![alice()], vec![token_id1, token_id2]);
            assert_eq!(balances, vec![100, 200]);
            
            let balances = contract.balance_of_batch(vec![alice(), bob()], vec![token_id1]);
            assert_eq!(balances, vec![100, 0]);
        }
    }
}
```

## Key Migration Points

### 1. Storage Structure
**Solidity:**
```solidity
mapping(uint256 => mapping(address => uint256)) private _balances;
mapping(address => mapping(address => bool)) private _operatorApprovals;
uint256 private _tokenIdCounter;
```

**ink!:**
```rust
#[ink(storage)]
pub struct Contract {
    balances: Mapping<(AccountId, TokenId), Balance>,
    approvals: Mapping<(Owner, Operator), ()>,
    token_id_nonce: TokenId,
}
```

### 2. Event Definitions
**Solidity:**
```solidity
event TransferSingle(
    address indexed operator,
    address indexed from,
    address indexed to,
    uint256 id,
    uint256 value
);
```

**ink!:**
```rust
#[ink(event)]
pub struct TransferSingle {
    #[ink(topic)]
    operator: Option<AccountId>,
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    token_id: TokenId,
    value: Balance,
}
```

### 3. Batch Operations
**Solidity:**
```solidity
function safeBatchTransferFrom(
    address from,
    address to,
    uint256[] calldata ids,
    uint256[] calldata values,
    bytes calldata data
) public {
    // Multiple balance checks and transfers in a loop
}
```

**ink!:**
```rust
#[ink(message)]
fn safe_batch_transfer_from(
    &mut self,
    from: AccountId,
    to: AccountId,
    token_ids: Vec<TokenId>,
    values: Vec<Balance>,
    data: Vec<u8>,
) -> Result<()> {
    // Check all balances first, then perform transfers
}
```

### 4. Token Receiver Pattern
**Solidity:**
```solidity
function _doSafeTransferAcceptanceCheck(
    address operator,
    address from,
    address to,
    uint256 id,
    uint256 value,
    bytes memory data
) private {
    if (to.code.length > 0) {
        try IERC1155Receiver(to).onERC1155Received(operator, from, id, value, data) returns (bytes4 response) {
            if (response != IERC1155Receiver.onERC1155Received.selector) {
                revert ERC1155ReceiverRejected();
            }
        } catch {
            revert ERC1155ReceiverNotImplemented();
        }
    }
}
```

**ink!:**
```rust
#[ink::trait_definition]
pub trait Erc1155TokenReceiver {
    #[ink(message, selector = 0xF23A6E61)]
    fn on_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        token_id: TokenId,
        value: Balance,
        data: Vec<u8>,
    ) -> Vec<u8>;
}

// Cross-contract call using build_call
let result = build_call::<Environment>()
    .call(to)
    .exec_input(ExecutionInput::new(Selector::new(ON_ERC_1155_RECEIVED_SELECTOR)))
    .returns::<Vec<u8>>()
    .try_invoke();
```

### 5. Approval Management
**Solidity:**
```solidity
mapping(address => mapping(address => bool)) private _operatorApprovals;
_operatorApprovals[msg.sender][operator] = approved;
```

**ink!:**
```rust
approvals: Mapping<(Owner, Operator), ()>,
if approved {
    self.approvals.insert((&caller, &operator), &());
} else {
    self.approvals.remove((&caller, &operator));
}
```

## Migration Steps

### Step 1: Convert Storage Structure
1. Replace nested mappings with tuple-key mappings
2. Use `Balance` type for token amounts
3. Use `TokenId` type for token identifiers
4. Store approval as unit type `()` instead of boolean

### Step 2: Convert Events
1. Create event structs with `#[ink(event)]`
2. Use `Option<AccountId>` for mint/burn semantics
3. Add `#[ink(topic)]` for indexed fields
4. Support both single and batch transfer events

### Step 3: Implement Core Functions
1. Add `#[ink(message)]` to all public functions
2. Return `Result<()>` instead of void for fallible operations
3. Use explicit error handling with custom error types
4. Handle arithmetic operations with overflow checks

### Step 4: Convert Batch Operations
1. Use `Vec<TokenId>` and `Vec<Balance>` for batch parameters
2. Validate array lengths before processing
3. Check all balances before performing any transfers
4. Use iterator patterns for efficient processing

### Step 5: Handle Token Receiver Interface
1. Define trait with explicit selectors
2. Use cross-contract calls for acceptance checks
3. Handle contract vs EOA recipients properly
4. Implement proper error handling for call failures

### Step 6: Add Comprehensive Tests
1. Test token creation and minting
2. Test single and batch transfers
3. Test approval mechanisms
4. Test error conditions and edge cases

## Common Patterns

### Token Creation and Minting
**Solidity:**
```solidity
function create(uint256 value) public returns (uint256 tokenId) {
    tokenId = _tokenIdCounter++;
    _balances[tokenId][msg.sender] = value;
    emit TransferSingle(msg.sender, address(0), msg.sender, tokenId, value);
}
```

**ink!:**
```rust
#[ink(message)]
pub fn create(&mut self, value: Balance) -> TokenId {
    let caller = self.env().caller();
    self.token_id_nonce = self.token_id_nonce.checked_add(1).unwrap();
    self.balances.insert((caller, self.token_id_nonce), &value);
    
    self.env().emit_event(TransferSingle {
        operator: Some(caller),
        from: None,
        to: if value == 0 { None } else { Some(caller) },
        token_id: self.token_id_nonce,
        value,
    });
    
    self.token_id_nonce
}
```

### Batch Balance Query
**Solidity:**
```solidity
function balanceOfBatch(
    address[] calldata accounts,
    uint256[] calldata ids
) public view returns (uint256[] memory) {
    uint256[] memory batchBalances = new uint256[](accounts.length);
    for (uint256 i = 0; i < accounts.length; i++) {
        batchBalances[i] = balanceOf(accounts[i], ids[i]);
    }
    return batchBalances;
}
```

**ink!:**
```rust
#[ink(message)]
fn balance_of_batch(
    &self,
    owners: Vec<AccountId>,
    token_ids: Vec<TokenId>,
) -> Vec<Balance> {
    let mut output = Vec::new();
    for o in &owners {
        for t in &token_ids {
            let amount = self.balance_of(*o, *t);
            output.push(amount);
        }
    }
    output
}
```

### Safe Transfer with Acceptance Check
**Solidity:**
```solidity
function safeTransferFrom(
    address from,
    address to,
    uint256 id,
    uint256 value,
    bytes calldata data
) public {
    // Perform transfer
    _balances[id][from] = fromBalance - value;
    _balances[id][to] += value;
    
    emit TransferSingle(msg.sender, from, to, id, value);
    _doSafeTransferAcceptanceCheck(msg.sender, from, to, id, value, data);
}
```

**ink!:**
```rust
#[ink(message)]
fn safe_transfer_from(
    &mut self,
    from: AccountId,
    to: AccountId,
    token_id: TokenId,
    value: Balance,
    data: Vec<u8>,
) -> Result<()> {
    // Validation and balance checks
    ensure!(to != zero_address(), Error::ZeroAddressTransfer);
    ensure!(balance >= value, Error::InsufficientBalance);
    
    self.perform_transfer(from, to, token_id, value);
    self.transfer_acceptance_check(caller, from, to, token_id, value, data);
    
    Ok(())
}
```

## Best Practices

### 1. Efficient Storage Usage
- Use tuple keys for multi-dimensional mappings
- Store approvals as unit type `()` instead of boolean
- Cache frequently accessed values

### 2. Proper Error Handling
- Use `Result<T, Error>` for all fallible operations
- Define descriptive error types
- Handle arithmetic overflow explicitly

### 3. Event Management
- Use `Option<AccountId>` for mint/burn semantics
- Emit events after successful state changes
- Include all relevant data in events

### 4. Cross-Contract Communication
- Use proper selectors for receiver interface
- Handle both contract and EOA recipients
- Implement proper error handling for call failures

### 5. Batch Operations
- Validate all inputs before processing
- Use iterator patterns for efficiency
- Check all preconditions before state changes

This migration demonstrates how Solidity's ERC1155 multi-token implementation translates to ink! with improved type safety, better error handling, and more efficient storage patterns while maintaining full compatibility with the ERC1155 standard.