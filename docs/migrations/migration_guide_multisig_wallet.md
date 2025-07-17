# MultiSig Wallet Implementation: Solidity vs ink!

## Overview
A comprehensive multi-signature wallet that requires multiple owner confirmations to execute transactions. This example demonstrates advanced governance patterns, transaction management, owner administration, and secure multi-party decision making in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title MultiSigWallet
/// @notice A multisignature wallet requiring multiple signatures for transactions
/// @dev Demonstrates multi-party approval and secure transaction execution
contract MultiSigWallet {
    // Transaction struct
    struct Transaction {
        address to;
        uint256 value;
        bytes data;
        bool executed;
        uint256 confirmations;
        mapping(address => bool) confirmed;
    }
    
    // State variables
    address[] public owners;
    mapping(address => bool) public isOwner;
    uint256 public required;
    uint256 public transactionCount;
    mapping(uint256 => Transaction) public transactions;
    
    // Events
    event Deposit(address indexed sender, uint256 amount);
    event Submission(uint256 indexed transactionId, address indexed sender, address indexed destination, uint256 value, bytes data);
    event Confirmation(uint256 indexed transactionId, address indexed sender);
    event Revocation(uint256 indexed transactionId, address indexed sender);
    event Execution(uint256 indexed transactionId);
    event ExecutionFailure(uint256 indexed transactionId);
    event OwnerAddition(address indexed owner);
    event OwnerRemoval(address indexed owner);
    event RequirementChange(uint256 required);
    
    // Custom errors
    error NotOwner(address caller);
    error TransactionDoesNotExist(uint256 transactionId);
    error TransactionAlreadyExecuted(uint256 transactionId);
    error TransactionAlreadyConfirmed(uint256 transactionId);
    error TransactionNotConfirmed(uint256 transactionId);
    error InsufficientConfirmations(uint256 transactionId);
    error OwnerAlreadyExists(address owner);
    error OwnerDoesNotExist(address owner);
    error InvalidRequirement(uint256 required, uint256 ownerCount);
    
    // Modifiers
    modifier onlyOwner() {
        if (!isOwner[msg.sender]) {
            revert NotOwner(msg.sender);
        }
        _;
    }
    
    modifier transactionExists(uint256 transactionId) {
        if (transactions[transactionId].to == address(0)) {
            revert TransactionDoesNotExist(transactionId);
        }
        _;
    }
    
    modifier notExecuted(uint256 transactionId) {
        if (transactions[transactionId].executed) {
            revert TransactionAlreadyExecuted(transactionId);
        }
        _;
    }
    
    modifier notConfirmed(uint256 transactionId) {
        if (transactions[transactionId].confirmed[msg.sender]) {
            revert TransactionAlreadyConfirmed(transactionId);
        }
        _;
    }
    
    modifier confirmed(uint256 transactionId) {
        if (!transactions[transactionId].confirmed[msg.sender]) {
            revert TransactionNotConfirmed(transactionId);
        }
        _;
    }
    
    modifier validRequirement(uint256 ownerCount, uint256 _required) {
        if (_required > ownerCount || _required == 0 || ownerCount == 0) {
            revert InvalidRequirement(_required, ownerCount);
        }
        _;
    }
    
    /// @notice Constructor sets initial owners and required confirmations
    /// @param _owners List of initial owners
    /// @param _required Number of required confirmations
    constructor(address[] memory _owners, uint256 _required) validRequirement(_owners.length, _required) {
        for (uint256 i = 0; i < _owners.length; i++) {
            require(_owners[i] != address(0), "Owner cannot be zero address");
            require(!isOwner[_owners[i]], "Owner already exists");
            
            isOwner[_owners[i]] = true;
            owners.push(_owners[i]);
        }
        
        required = _required;
    }
    
    /// @notice Fallback function to receive Ether
    receive() external payable {
        emit Deposit(msg.sender, msg.value);
    }
    
    /// @notice Submit a transaction for approval
    /// @param destination Transaction target address
    /// @param value Transaction value
    /// @param data Transaction data
    /// @return transactionId The ID of the submitted transaction
    function submitTransaction(address destination, uint256 value, bytes memory data) public onlyOwner returns (uint256) {
        uint256 transactionId = transactionCount++;
        
        Transaction storage txn = transactions[transactionId];
        txn.to = destination;
        txn.value = value;
        txn.data = data;
        txn.executed = false;
        txn.confirmations = 0;
        
        emit Submission(transactionId, msg.sender, destination, value, data);
        
        // Automatically confirm the transaction from the submitter
        confirmTransaction(transactionId);
        
        return transactionId;
    }
    
    /// @notice Confirm a transaction
    /// @param transactionId The transaction ID to confirm
    function confirmTransaction(uint256 transactionId) public 
        onlyOwner 
        transactionExists(transactionId) 
        notConfirmed(transactionId) 
    {
        Transaction storage txn = transactions[transactionId];
        txn.confirmed[msg.sender] = true;
        txn.confirmations++;
        
        emit Confirmation(transactionId, msg.sender);
        
        // Try to execute if we have enough confirmations
        if (txn.confirmations >= required) {
            executeTransaction(transactionId);
        }
    }
    
    /// @notice Revoke confirmation for a transaction
    /// @param transactionId The transaction ID to revoke confirmation for
    function revokeConfirmation(uint256 transactionId) public 
        onlyOwner 
        transactionExists(transactionId) 
        confirmed(transactionId) 
        notExecuted(transactionId) 
    {
        Transaction storage txn = transactions[transactionId];
        txn.confirmed[msg.sender] = false;
        txn.confirmations--;
        
        emit Revocation(transactionId, msg.sender);
    }
    
    /// @notice Execute a confirmed transaction
    /// @param transactionId The transaction ID to execute
    function executeTransaction(uint256 transactionId) public 
        onlyOwner 
        transactionExists(transactionId) 
        notExecuted(transactionId) 
    {
        Transaction storage txn = transactions[transactionId];
        
        if (txn.confirmations < required) {
            revert InsufficientConfirmations(transactionId);
        }
        
        txn.executed = true;
        
        (bool success, ) = txn.to.call{value: txn.value}(txn.data);
        
        if (success) {
            emit Execution(transactionId);
        } else {
            emit ExecutionFailure(transactionId);
            txn.executed = false;
        }
    }
    
    /// @notice Add a new owner (requires multisig approval)
    /// @param owner Address of new owner
    function addOwner(address owner) public onlyOwner {
        require(owner != address(0), "Owner cannot be zero address");
        if (isOwner[owner]) {
            revert OwnerAlreadyExists(owner);
        }
        
        isOwner[owner] = true;
        owners.push(owner);
        
        emit OwnerAddition(owner);
    }
    
    /// @notice Remove an owner (requires multisig approval)
    /// @param owner Address of owner to remove
    function removeOwner(address owner) public onlyOwner {
        if (!isOwner[owner]) {
            revert OwnerDoesNotExist(owner);
        }
        
        isOwner[owner] = false;
        
        // Remove from owners array
        for (uint256 i = 0; i < owners.length; i++) {
            if (owners[i] == owner) {
                owners[i] = owners[owners.length - 1];
                owners.pop();
                break;
            }
        }
        
        // Adjust required confirmations if necessary
        if (required > owners.length) {
            required = owners.length;
            emit RequirementChange(required);
        }
        
        emit OwnerRemoval(owner);
    }
    
    /// @notice Change the required number of confirmations (requires multisig approval)
    /// @param _required New required number of confirmations
    function changeRequirement(uint256 _required) public onlyOwner validRequirement(owners.length, _required) {
        required = _required;
        emit RequirementChange(_required);
    }
    
    /// @notice Get list of owners
    /// @return Array of owner addresses
    function getOwners() public view returns (address[] memory) {
        return owners;
    }
    
    /// @notice Get transaction details
    /// @param transactionId The transaction ID
    /// @return to Transaction destination
    /// @return value Transaction value
    /// @return data Transaction data
    /// @return executed Whether transaction was executed
    /// @return confirmations Number of confirmations
    function getTransaction(uint256 transactionId) public view 
        transactionExists(transactionId) 
        returns (address to, uint256 value, bytes memory data, bool executed, uint256 confirmations) 
    {
        Transaction storage txn = transactions[transactionId];
        return (txn.to, txn.value, txn.data, txn.executed, txn.confirmations);
    }
    
    /// @notice Get confirmation status for a transaction
    /// @param transactionId The transaction ID
    /// @param owner The owner address
    /// @return True if owner has confirmed the transaction
    function getConfirmation(uint256 transactionId, address owner) public view 
        transactionExists(transactionId) 
        returns (bool) 
    {
        return transactions[transactionId].confirmed[owner];
    }
    
    /// @notice Check if transaction is confirmed by required number of owners
    /// @param transactionId The transaction ID
    /// @return True if transaction is confirmed
    function isConfirmed(uint256 transactionId) public view 
        transactionExists(transactionId) 
        returns (bool) 
    {
        return transactions[transactionId].confirmations >= required;
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod multisig {
    use ink::{
        env::{
            call::{build_call, ExecutionInput},
            CallFlags,
        },
        prelude::vec::Vec,
        storage::Mapping,
    };

    /// Maximum number of owners for gas efficiency
    const MAX_OWNERS: u32 = 50;
    
    /// Transaction ID type
    pub type TransactionId = u32;

    /// Confirmation status for transactions
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ConfirmationStatus {
        /// Transaction is confirmed and ready for execution
        Confirmed,
        /// Number of confirmations still needed
        ConfirmationsNeeded(u32),
    }

    /// Transaction data structure
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Transaction {
        /// Target contract address
        pub callee: AccountId,
        /// Function selector (first 4 bytes of function signature)
        pub selector: [u8; 4],
        /// Encoded function parameters
        pub input: Vec<u8>,
        /// Amount of native token to transfer
        pub transferred_value: Balance,
        /// Gas limit for execution
        pub ref_time_limit: u64,
        /// Allow re-entrancy flag
        pub allow_reentry: bool,
    }

    /// Transaction storage tracking
    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Transactions {
        /// List of all transaction IDs
        transactions: Vec<TransactionId>,
        /// Next available transaction ID
        next_id: TransactionId,
    }

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct Multisig {
        /// List of wallet owners
        owners: Vec<AccountId>,
        /// Quick lookup for owner status
        is_owner: Mapping<AccountId, ()>,
        /// Required number of confirmations
        requirement: u32,
        /// Transaction confirmations (transaction_id, owner) -> ()
        confirmations: Mapping<(TransactionId, AccountId), ()>,
        /// Count of confirmations per transaction
        confirmation_count: Mapping<TransactionId, u32>,
        /// Transaction storage
        transactions: Mapping<TransactionId, Transaction>,
        /// Transaction list for cleanup
        transaction_list: Transactions,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        sender: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Submission {
        #[ink(topic)]
        transaction: TransactionId,
        #[ink(topic)]
        sender: AccountId,
    }

    #[ink(event)]
    pub struct Confirmation {
        #[ink(topic)]
        transaction: TransactionId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        status: ConfirmationStatus,
    }

    #[ink(event)]
    pub struct Revocation {
        #[ink(topic)]
        transaction: TransactionId,
        #[ink(topic)]
        from: AccountId,
    }

    #[ink(event)]
    pub struct Execution {
        #[ink(topic)]
        transaction: TransactionId,
        #[ink(topic)]
        result: Result<Option<Vec<u8>>, Error>,
    }

    #[ink(event)]
    pub struct Cancellation {
        #[ink(topic)]
        transaction: TransactionId,
    }

    #[ink(event)]
    pub struct OwnerAddition {
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct OwnerRemoval {
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct RequirementChange {
        new_requirement: u32,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        NotFromWallet,
        TransactionNotFound,
        TransactionAlreadyExecuted,
        TransactionAlreadyConfirmed,
        TransactionNotConfirmed,
        InsufficientConfirmations,
        OwnerAlreadyExists,
        OwnerDoesNotExist,
        InvalidRequirement,
        TransactionFailed,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Multisig {
        /// Constructor that initializes the multisig wallet
        #[ink(constructor)]
        pub fn new(requirement: u32, mut owners: Vec<AccountId>) -> Self {
            // Remove duplicates and sort
            owners.sort_unstable();
            owners.dedup();
            
            // Validate requirement
            Self::ensure_requirement_is_valid(owners.len() as u32, requirement);
            
            let mut contract = Self {
                owners: Vec::new(),
                is_owner: Mapping::default(),
                requirement,
                confirmations: Mapping::default(),
                confirmation_count: Mapping::default(),
                transactions: Mapping::default(),
                transaction_list: Transactions::default(),
            };
            
            // Set up owners
            for owner in &owners {
                contract.is_owner.insert(owner, &());
            }
            contract.owners = owners;
            
            contract
        }

        /// Receive function to accept native token deposits
        #[ink(message, payable)]
        pub fn receive(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            
            self.env().emit_event(Deposit {
                sender: caller,
                amount,
            });
        }

        /// Submit a new transaction for approval
        #[ink(message)]
        pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(TransactionId, ConfirmationStatus)> {
            self.ensure_caller_is_owner()?;
            
            let trans_id = self.transaction_list.next_id;
            self.transaction_list.next_id = trans_id.checked_add(1).ok_or(Error::TransactionFailed)?;
            
            self.transactions.insert(trans_id, &transaction);
            self.transaction_list.transactions.push(trans_id);
            
            self.env().emit_event(Submission {
                transaction: trans_id,
                sender: self.env().caller(),
            });
            
            // Auto-confirm from submitter
            let status = self.confirm_by_caller(self.env().caller(), trans_id);
            
            Ok((trans_id, status))
        }

        /// Confirm a transaction
        #[ink(message)]
        pub fn confirm_transaction(&mut self, trans_id: TransactionId) -> Result<ConfirmationStatus> {
            self.ensure_caller_is_owner()?;
            self.ensure_transaction_exists(trans_id)?;
            
            Ok(self.confirm_by_caller(self.env().caller(), trans_id))
        }

        /// Revoke confirmation for a transaction
        #[ink(message)]
        pub fn revoke_confirmation(&mut self, trans_id: TransactionId) -> Result<()> {
            self.ensure_caller_is_owner()?;
            let caller = self.env().caller();
            
            if self.confirmations.contains((trans_id, caller)) {
                self.confirmations.remove((trans_id, caller));
                
                let mut count = self.confirmation_count.get(trans_id).unwrap_or(0);
                count = count.saturating_sub(1);
                self.confirmation_count.insert(trans_id, &count);
                
                self.env().emit_event(Revocation {
                    transaction: trans_id,
                    from: caller,
                });
            }
            
            Ok(())
        }

        /// Execute a confirmed transaction
        #[ink(message, payable)]
        pub fn invoke_transaction(&mut self, trans_id: TransactionId) -> Result<()> {
            self.ensure_confirmed(trans_id)?;
            let transaction = self.take_transaction(trans_id)?;
            
            // Verify transferred value matches
            if self.env().transferred_value() != transaction.transferred_value {
                return Err(Error::TransactionFailed);
            }
            
            let call_flags = if transaction.allow_reentry {
                CallFlags::ALLOW_REENTRY
            } else {
                CallFlags::empty()
            };

            // Execute the transaction
            let result = build_call::<Environment>()
                .call(transaction.callee)
                .ref_time_limit(transaction.ref_time_limit)
                .transferred_value(transaction.transferred_value)
                .call_flags(call_flags)
                .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(&transaction.input))
                .returns::<()>()
                .try_invoke();

            let execution_result = match result {
                Ok(Ok(_)) => Ok(()),
                _ => Err(Error::TransactionFailed),
            };

            self.env().emit_event(Execution {
                transaction: trans_id,
                result: execution_result.map(|_| None),
            });

            execution_result
        }

        /// Cancel a transaction (only callable by wallet itself)
        #[ink(message)]
        pub fn cancel_transaction(&mut self, trans_id: TransactionId) -> Result<()> {
            self.ensure_from_wallet()?;
            
            if self.take_transaction(trans_id).is_ok() {
                self.env().emit_event(Cancellation {
                    transaction: trans_id,
                });
            }
            
            Ok(())
        }

        /// Add a new owner (only callable by wallet itself)
        #[ink(message)]
        pub fn add_owner(&mut self, new_owner: AccountId) -> Result<()> {
            self.ensure_from_wallet()?;
            self.ensure_no_owner(&new_owner)?;
            
            let new_owner_count = (self.owners.len() as u32).checked_add(1).ok_or(Error::InvalidRequirement)?;
            Self::ensure_requirement_is_valid(new_owner_count, self.requirement);
            
            self.is_owner.insert(new_owner, &());
            self.owners.push(new_owner);
            
            self.env().emit_event(OwnerAddition { owner: new_owner });
            
            Ok(())
        }

        /// Remove an owner (only callable by wallet itself)
        #[ink(message)]
        pub fn remove_owner(&mut self, owner: AccountId) -> Result<()> {
            self.ensure_from_wallet()?;
            self.ensure_owner(&owner)?;
            
            let new_owner_count = (self.owners.len() as u32).saturating_sub(1);
            let new_requirement = u32::min(new_owner_count, self.requirement);
            Self::ensure_requirement_is_valid(new_owner_count, new_requirement);
            
            // Remove from owners list
            if let Some(pos) = self.owners.iter().position(|&x| x == owner) {
                self.owners.remove(pos);
            }
            
            self.is_owner.remove(owner);
            self.requirement = new_requirement;
            self.clean_owner_confirmations(&owner);
            
            self.env().emit_event(OwnerRemoval { owner });
            
            Ok(())
        }

        /// Replace an owner (only callable by wallet itself)
        #[ink(message)]
        pub fn replace_owner(&mut self, old_owner: AccountId, new_owner: AccountId) -> Result<()> {
            self.ensure_from_wallet()?;
            self.ensure_owner(&old_owner)?;
            self.ensure_no_owner(&new_owner)?;
            
            // Replace in owners list
            if let Some(pos) = self.owners.iter().position(|&x| x == old_owner) {
                self.owners[pos] = new_owner;
            }
            
            self.is_owner.remove(old_owner);
            self.is_owner.insert(new_owner, &());
            self.clean_owner_confirmations(&old_owner);
            
            self.env().emit_event(OwnerRemoval { owner: old_owner });
            self.env().emit_event(OwnerAddition { owner: new_owner });
            
            Ok(())
        }

        /// Change the requirement (only callable by wallet itself)
        #[ink(message)]
        pub fn change_requirement(&mut self, new_requirement: u32) -> Result<()> {
            self.ensure_from_wallet()?;
            Self::ensure_requirement_is_valid(self.owners.len() as u32, new_requirement);
            
            self.requirement = new_requirement;
            
            self.env().emit_event(RequirementChange { new_requirement });
            
            Ok(())
        }

        /// Get list of owners
        #[ink(message)]
        pub fn get_owners(&self) -> Vec<AccountId> {
            self.owners.clone()
        }

        /// Get transaction details
        #[ink(message)]
        pub fn get_transaction(&self, trans_id: TransactionId) -> Option<Transaction> {
            self.transactions.get(trans_id)
        }

        /// Get confirmation count for a transaction
        #[ink(message)]
        pub fn get_confirmation_count(&self, trans_id: TransactionId) -> u32 {
            self.confirmation_count.get(trans_id).unwrap_or(0)
        }

        /// Check if owner has confirmed a transaction
        #[ink(message)]
        pub fn get_confirmation(&self, trans_id: TransactionId, owner: AccountId) -> bool {
            self.confirmations.contains((trans_id, owner))
        }

        /// Check if transaction is confirmed
        #[ink(message)]
        pub fn is_confirmed(&self, trans_id: TransactionId) -> bool {
            self.confirmation_count.get(trans_id).unwrap_or(0) >= self.requirement
        }

        /// Get current requirement
        #[ink(message)]
        pub fn get_requirement(&self) -> u32 {
            self.requirement
        }

        /// Get owner count
        #[ink(message)]
        pub fn get_owner_count(&self) -> u32 {
            self.owners.len() as u32
        }

        /// Check if address is an owner
        #[ink(message)]
        pub fn is_owner(&self, account: AccountId) -> bool {
            self.is_owner.contains(account)
        }

        /// Internal: Set transaction as confirmed by caller
        fn confirm_by_caller(&mut self, confirmer: AccountId, transaction: TransactionId) -> ConfirmationStatus {
            let mut count = self.confirmation_count.get(transaction).unwrap_or(0);
            let key = (transaction, confirmer);
            let new_confirmation = !self.confirmations.contains(key);
            
            if new_confirmation {
                count = count.checked_add(1).unwrap();
                self.confirmations.insert(key, &());
                self.confirmation_count.insert(transaction, &count);
            }
            
            let status = if count >= self.requirement {
                ConfirmationStatus::Confirmed
            } else {
                ConfirmationStatus::ConfirmationsNeeded(self.requirement - count)
            };
            
            if new_confirmation {
                self.env().emit_event(Confirmation {
                    transaction,
                    from: confirmer,
                    status,
                });
            }
            
            status
        }

        /// Internal: Remove transaction and cleanup
        fn take_transaction(&mut self, trans_id: TransactionId) -> Result<Transaction> {
            let transaction = self.transactions.get(trans_id).ok_or(Error::TransactionNotFound)?;
            
            self.transactions.remove(trans_id);
            
            // Remove from transaction list
            if let Some(pos) = self.transaction_list.transactions.iter().position(|&x| x == trans_id) {
                self.transaction_list.transactions.remove(pos);
            }
            
            // Clean up confirmations
            for owner in &self.owners {
                self.confirmations.remove((trans_id, *owner));
            }
            self.confirmation_count.remove(trans_id);
            
            Ok(transaction)
        }

        /// Internal: Clean confirmations for removed owner
        fn clean_owner_confirmations(&mut self, owner: &AccountId) {
            for &trans_id in &self.transaction_list.transactions {
                let key = (trans_id, *owner);
                if self.confirmations.contains(key) {
                    self.confirmations.remove(key);
                    let mut count = self.confirmation_count.get(trans_id).unwrap_or(0);
                    count = count.saturating_sub(1);
                    self.confirmation_count.insert(trans_id, &count);
                }
            }
        }

        /// Validation: Ensure caller is owner
        fn ensure_caller_is_owner(&self) -> Result<()> {
            if !self.is_owner.contains(self.env().caller()) {
                return Err(Error::NotOwner);
            }
            Ok(())
        }

        /// Validation: Ensure call is from wallet itself
        fn ensure_from_wallet(&self) -> Result<()> {
            if self.env().caller() != self.env().account_id() {
                return Err(Error::NotFromWallet);
            }
            Ok(())
        }

        /// Validation: Ensure transaction exists
        fn ensure_transaction_exists(&self, trans_id: TransactionId) -> Result<()> {
            if !self.transactions.contains(trans_id) {
                return Err(Error::TransactionNotFound);
            }
            Ok(())
        }

        /// Validation: Ensure transaction is confirmed
        fn ensure_confirmed(&self, trans_id: TransactionId) -> Result<()> {
            if self.confirmation_count.get(trans_id).unwrap_or(0) < self.requirement {
                return Err(Error::InsufficientConfirmations);
            }
            Ok(())
        }

        /// Validation: Ensure account is owner
        fn ensure_owner(&self, owner: &AccountId) -> Result<()> {
            if !self.is_owner.contains(*owner) {
                return Err(Error::OwnerDoesNotExist);
            }
            Ok(())
        }

        /// Validation: Ensure account is not owner
        fn ensure_no_owner(&self, owner: &AccountId) -> Result<()> {
            if self.is_owner.contains(*owner) {
                return Err(Error::OwnerAlreadyExists);
            }
            Ok(())
        }

        /// Validation: Ensure requirement is valid
        fn ensure_requirement_is_valid(owners: u32, requirement: u32) {
            assert!(0 < requirement && requirement <= owners && owners <= MAX_OWNERS);
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let owners = vec![accounts.alice, accounts.bob, accounts.charlie];
            let contract = Multisig::new(2, owners.clone());
            
            assert_eq!(contract.get_owner_count(), 3);
            assert_eq!(contract.get_requirement(), 2);
            assert!(contract.is_owner(accounts.alice));
            assert!(contract.is_owner(accounts.bob));
            assert!(contract.is_owner(accounts.charlie));
        }

        #[ink::test]
        fn submit_transaction_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let owners = vec![accounts.alice, accounts.bob];
            let mut contract = Multisig::new(2, owners);
            
            let transaction = Transaction {
                callee: accounts.charlie,
                selector: [0u8; 4],
                input: Vec::new(),
                transferred_value: 0,
                ref_time_limit: 1000000,
                allow_reentry: false,
            };
            
            let result = contract.submit_transaction(transaction);
            assert!(result.is_ok());
            
            let (trans_id, status) = result.unwrap();
            assert_eq!(trans_id, 0);
            assert_eq!(status, ConfirmationStatus::ConfirmationsNeeded(1));
        }

        #[ink::test]
        fn confirmation_workflow_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let owners = vec![accounts.alice, accounts.bob];
            let mut contract = Multisig::new(2, owners);
            
            let transaction = Transaction {
                callee: accounts.charlie,
                selector: [0u8; 4],
                input: Vec::new(),
                transferred_value: 0,
                ref_time_limit: 1000000,
                allow_reentry: false,
            };
            
            // Submit transaction (auto-confirms from alice)
            let (trans_id, _) = contract.submit_transaction(transaction).unwrap();
            assert_eq!(contract.get_confirmation_count(trans_id), 1);
            assert!(!contract.is_confirmed(trans_id));
            
            // Confirm from bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let status = contract.confirm_transaction(trans_id).unwrap();
            assert_eq!(status, ConfirmationStatus::Confirmed);
            assert_eq!(contract.get_confirmation_count(trans_id), 2);
            assert!(contract.is_confirmed(trans_id));
        }

        #[ink::test]
        fn revoke_confirmation_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let owners = vec![accounts.alice, accounts.bob];
            let mut contract = Multisig::new(2, owners);
            
            let transaction = Transaction {
                callee: accounts.charlie,
                selector: [0u8; 4],
                input: Vec::new(),
                transferred_value: 0,
                ref_time_limit: 1000000,
                allow_reentry: false,
            };
            
            let (trans_id, _) = contract.submit_transaction(transaction).unwrap();
            assert_eq!(contract.get_confirmation_count(trans_id), 1);
            
            // Revoke confirmation
            assert!(contract.revoke_confirmation(trans_id).is_ok());
            assert_eq!(contract.get_confirmation_count(trans_id), 0);
        }
    }
}
```

## Key Migration Points

### 1. Transaction Structure
**Solidity:**
```solidity
struct Transaction {
    address to;
    uint256 value;
    bytes data;
    bool executed;
    uint256 confirmations;
    mapping(address => bool) confirmed;
}
```

**ink!:**
```rust
pub struct Transaction {
    pub callee: AccountId,
    pub selector: [u8; 4],
    pub input: Vec<u8>,
    pub transferred_value: Balance,
    pub ref_time_limit: u64,
    pub allow_reentry: bool,
}
```

### 2. Modifier to Function Conversion
**Solidity:**
```solidity
modifier onlyOwner() {
    if (!isOwner[msg.sender]) {
        revert NotOwner(msg.sender);
    }
    _;
}

function submitTransaction(...) public onlyOwner {
    // function body
}
```

**ink!:**
```rust
#[ink(message)]
pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(TransactionId, ConfirmationStatus)> {
    self.ensure_caller_is_owner()?;
    // function body
}

fn ensure_caller_is_owner(&self) -> Result<()> {
    if !self.is_owner.contains(self.env().caller()) {
        return Err(Error::NotOwner);
    }
    Ok(())
}
```

### 3. Transaction Execution
**Solidity:**
```solidity
(bool success, ) = txn.to.call{value: txn.value}(txn.data);
```

**ink!:**
```rust
let result = build_call::<Environment>()
    .call(transaction.callee)
    .ref_time_limit(transaction.ref_time_limit)
    .transferred_value(transaction.transferred_value)
    .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(&transaction.input))
    .try_invoke();
```

### 4. Owner Management
**Solidity:**
```solidity
mapping(address => bool) public isOwner;
address[] public owners;
```

**ink!:**
```rust
owners: Vec<AccountId>,
is_owner: Mapping<AccountId, ()>,
```

### 5. Confirmation Tracking
**Solidity:**
```solidity
mapping(uint256 => Transaction) public transactions;
// Inside Transaction struct:
mapping(address => bool) confirmed;
```

**ink!:**
```rust
confirmations: Mapping<(TransactionId, AccountId), ()>,
confirmation_count: Mapping<TransactionId, u32>,
transactions: Mapping<TransactionId, Transaction>,
```

## Migration Steps

### Step 1: Define Data Structures
1. Create `Transaction` struct without nested mappings
2. Use separate mappings for confirmations
3. Define proper error and result types

### Step 2: Convert Access Control
1. Replace modifiers with explicit validation functions
2. Use `Result<T, Error>` return types
3. Handle wallet-only operations properly

### Step 3: Implement Transaction Management
1. Convert transaction submission and confirmation
2. Handle transaction execution with cross-contract calls
3. Implement proper cleanup for cancelled transactions

### Step 4: Owner Management
1. Convert owner addition/removal logic
2. Handle requirement validation
3. Clean up confirmations when owners are removed

### Step 5: Cross-Contract Calls
1. Use `build_call` for transaction execution
2. Handle gas limits and re-entrancy flags
3. Properly encode function calls

## Common Patterns

### Validation Functions
```rust
fn ensure_caller_is_owner(&self) -> Result<()> {
    if !self.is_owner.contains(self.env().caller()) {
        return Err(Error::NotOwner);
    }
    Ok(())
}

fn ensure_from_wallet(&self) -> Result<()> {
    if self.env().caller() != self.env().account_id() {
        return Err(Error::NotFromWallet);
    }
    Ok(())
}
```

### Transaction Execution
```rust
let result = build_call::<Environment>()
    .call(transaction.callee)
    .ref_time_limit(transaction.ref_time_limit)
    .transferred_value(transaction.transferred_value)
    .call_flags(call_flags)
    .exec_input(ExecutionInput::new(transaction.selector.into()))
    .returns::<()>()
    .try_invoke();
```

### Confirmation Management
```rust
fn confirm_by_caller(&mut self, confirmer: AccountId, transaction: TransactionId) -> ConfirmationStatus {
    let key = (transaction, confirmer);
    if !self.confirmations.contains(key) {
        self.confirmations.insert(key, &());
        let count = self.confirmation_count.get(transaction).unwrap_or(0) + 1;
        self.confirmation_count.insert(transaction, &count);
    }
    // Return status...
}
```

## Best Practices

### 1. Access Control
- Use explicit validation functions instead of modifiers
- Separate wallet-only operations from owner operations
- Always return `Result<T, Error>` for validation

### 2. Transaction Management
- Use tuple keys for complex mappings
- Clean up storage when transactions are executed/cancelled
- Track transaction lists for proper cleanup

### 3. Cross-Contract Calls
- Always set appropriate gas limits
- Handle re-entrancy carefully
- Use proper call flags for security

### 4. Owner Management
- Validate requirements after owner changes
- Clean up confirmations when owners are removed
- Use efficient array operations for owner lists

### 5. Event Emission
- Emit comprehensive events for all state changes
- Use indexed topics for searchable fields
- Include result information in execution events

This migration demonstrates how Solidity's modifier-based multisig wallet translates to ink!'s more explicit and type-safe approach, with better error handling and more flexible cross-contract interaction patterns.