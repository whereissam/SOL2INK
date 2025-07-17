# Simple Storage Implementation: Solidity vs ink!

## Overview
A comprehensive storage contract demonstrating mapping operations, nested mappings, arrays, and advanced storage patterns. This example shows how to handle complex data structures, batch operations, and storage management in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleStorage
/// @notice A basic storage contract similar to ink! mapping example
/// @dev Demonstrates basic storage patterns with mappings and arrays
contract SimpleStorage {
    // State variables
    mapping(address => uint256) public balances;
    mapping(address => mapping(string => uint256)) public userNamedValues;
    address[] public users;
    uint256 public totalSupply;

    // Events
    event BalanceSet(address indexed user, uint256 oldBalance, uint256 newBalance);
    event ValueSet(address indexed user, string indexed name, uint256 value);
    event UserAdded(address indexed user);

    /// @notice Set balance for a specific account
    /// @param account The account to set balance for
    /// @param balance The new balance
    function setBalance(address account, uint256 balance) public {
        uint256 oldBalance = balances[account];
        
        // If this is a new user, add to users array
        if (oldBalance == 0 && balance > 0) {
            users.push(account);
            emit UserAdded(account);
        }
        
        // Update total supply
        totalSupply = totalSupply - oldBalance + balance;
        
        // Set new balance
        balances[account] = balance;
        
        emit BalanceSet(account, oldBalance, balance);
    }

    /// @notice Get balance for a specific account
    /// @param account The account to get balance for
    /// @return The account balance
    function getBalance(address account) public view returns (uint256) {
        return balances[account];
    }

    /// @notice Get the total supply
    /// @return The total supply
    function getTotalSupply() public view returns (uint256) {
        return totalSupply;
    }

    /// @notice Set a named value for a user
    /// @param name The name of the value
    /// @param value The value to set
    function setNamedValue(string memory name, uint256 value) public {
        userNamedValues[msg.sender][name] = value;
        emit ValueSet(msg.sender, name, value);
    }

    /// @notice Get a named value for a user
    /// @param user The user address
    /// @param name The name of the value
    /// @return The stored value
    function getNamedValue(address user, string memory name) public view returns (uint256) {
        return userNamedValues[user][name];
    }

    /// @notice Get all users who have non-zero balances
    /// @return Array of user addresses
    function getUsers() public view returns (address[] memory) {
        return users;
    }

    /// @notice Get the number of users
    /// @return The number of users
    function getUserCount() public view returns (uint256) {
        return users.length;
    }

    /// @notice Transfer balance from one account to another
    /// @param from The account to transfer from
    /// @param to The account to transfer to
    /// @param amount The amount to transfer
    function transfer(address from, address to, uint256 amount) public {
        require(balances[from] >= amount, "Insufficient balance");
        
        balances[from] -= amount;
        balances[to] += amount;
        
        // If 'to' is a new user, add to users array
        if (balances[to] == amount) {
            users.push(to);
            emit UserAdded(to);
        }
        
        emit BalanceSet(from, balances[from] + amount, balances[from]);
        emit BalanceSet(to, balances[to] - amount, balances[to]);
    }

    /// @notice Batch set balances for multiple accounts
    /// @param accounts Array of accounts
    /// @param newBalances Array of new balances
    function batchSetBalances(address[] memory accounts, uint256[] memory newBalances) public {
        require(accounts.length == newBalances.length, "Arrays length mismatch");
        
        for (uint256 i = 0; i < accounts.length; i++) {
            setBalance(accounts[i], newBalances[i]);
        }
    }

    /// @notice Clear all data (emergency function)
    function clearAll() public {
        // Clear all user balances
        for (uint256 i = 0; i < users.length; i++) {
            balances[users[i]] = 0;
        }
        
        // Clear users array
        delete users;
        
        // Reset total supply
        totalSupply = 0;
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    use ink::{
        prelude::{
            string::String,
            vec::Vec,
        },
        storage::Mapping,
    };

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct SimpleStorage {
        /// Mapping from account to balance
        balances: Mapping<AccountId, Balance>,
        /// Nested mapping: user -> name -> value
        user_named_values: Mapping<(AccountId, String), u128>,
        /// List of users with non-zero balances
        users: Vec<AccountId>,
        /// Total supply across all accounts
        total_supply: Balance,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct BalanceSet {
        #[ink(topic)]
        user: AccountId,
        old_balance: Balance,
        new_balance: Balance,
    }

    #[ink(event)]
    pub struct ValueSet {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        name: String,
        value: u128,
    }

    #[ink(event)]
    pub struct UserAdded {
        #[ink(topic)]
        user: AccountId,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
        ArrayLengthMismatch,
        ArithmeticOverflow,
        ValueTooLarge,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl SimpleStorage {
        /// Constructor that initializes an empty storage
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
                user_named_values: Mapping::default(),
                users: Vec::new(),
                total_supply: 0,
            }
        }

        /// Set balance for a specific account
        #[ink(message)]
        pub fn set_balance(&mut self, account: AccountId, balance: Balance) -> Result<()> {
            let old_balance = self.balances.get(account).unwrap_or(0);
            
            // If this is a new user with a positive balance, add to users list
            if old_balance == 0 && balance > 0 && !self.users.contains(&account) {
                self.users.push(account);
                self.env().emit_event(UserAdded { user: account });
            }
            
            // Update total supply
            self.total_supply = self.total_supply
                .checked_sub(old_balance)
                .and_then(|v| v.checked_add(balance))
                .ok_or(Error::ArithmeticOverflow)?;
            
            // Set new balance
            self.balances.insert(account, &balance);
            
            self.env().emit_event(BalanceSet {
                user: account,
                old_balance,
                new_balance: balance,
            });
            
            Ok(())
        }

        /// Get balance for a specific account
        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        /// Get the total supply
        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Set a named value for the caller
        #[ink(message)]
        pub fn set_named_value(&mut self, name: String, value: u128) -> Result<()> {
            let caller = self.env().caller();
            
            // Use tuple key for nested mapping
            self.user_named_values.insert((caller, name.clone()), &value);
            
            self.env().emit_event(ValueSet {
                user: caller,
                name,
                value,
            });
            
            Ok(())
        }

        /// Get a named value for a user
        #[ink(message)]
        pub fn get_named_value(&self, user: AccountId, name: String) -> u128 {
            self.user_named_values.get((user, name)).unwrap_or(0)
        }

        /// Get all users who have non-zero balances
        #[ink(message)]
        pub fn get_users(&self) -> Vec<AccountId> {
            self.users.clone()
        }

        /// Get the number of users
        #[ink(message)]
        pub fn get_user_count(&self) -> u32 {
            self.users.len() as u32
        }

        /// Transfer balance from one account to another
        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<()> {
            let from_balance = self.balances.get(from).unwrap_or(0);
            
            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }
            
            let to_balance = self.balances.get(to).unwrap_or(0);
            
            // Update balances
            self.balances.insert(from, &(from_balance - amount));
            self.balances.insert(to, &(to_balance + amount));
            
            // If 'to' is a new user, add to users list
            if to_balance == 0 && !self.users.contains(&to) {
                self.users.push(to);
                self.env().emit_event(UserAdded { user: to });
            }
            
            // Emit balance change events
            self.env().emit_event(BalanceSet {
                user: from,
                old_balance: from_balance,
                new_balance: from_balance - amount,
            });
            
            self.env().emit_event(BalanceSet {
                user: to,
                old_balance: to_balance,
                new_balance: to_balance + amount,
            });
            
            Ok(())
        }

        /// Batch set balances for multiple accounts
        #[ink(message)]
        pub fn batch_set_balances(&mut self, accounts: Vec<AccountId>, balances: Vec<Balance>) -> Result<()> {
            if accounts.len() != balances.len() {
                return Err(Error::ArrayLengthMismatch);
            }
            
            for (account, balance) in accounts.into_iter().zip(balances.into_iter()) {
                self.set_balance(account, balance)?;
            }
            
            Ok(())
        }

        /// Check if a balance exists for an account
        #[ink(message)]
        pub fn contains_balance(&self, account: AccountId) -> bool {
            self.balances.contains(account)
        }

        /// Remove a balance entry (sets to zero and removes from users list)
        #[ink(message)]
        pub fn remove_balance(&mut self, account: AccountId) -> Result<()> {
            let old_balance = self.balances.get(account).unwrap_or(0);
            
            if old_balance > 0 {
                // Update total supply
                self.total_supply = self.total_supply
                    .checked_sub(old_balance)
                    .ok_or(Error::ArithmeticOverflow)?;
                
                // Remove from mapping
                self.balances.remove(account);
                
                // Remove from users list
                if let Some(pos) = self.users.iter().position(|&x| x == account) {
                    self.users.remove(pos);
                }
                
                self.env().emit_event(BalanceSet {
                    user: account,
                    old_balance,
                    new_balance: 0,
                });
            }
            
            Ok(())
        }

        /// Take a balance (returns the balance and removes it from storage)
        #[ink(message)]
        pub fn take_balance(&mut self, account: AccountId) -> Result<Balance> {
            let balance = self.balances.take(account).unwrap_or(0);
            
            if balance > 0 {
                // Update total supply
                self.total_supply = self.total_supply
                    .checked_sub(balance)
                    .ok_or(Error::ArithmeticOverflow)?;
                
                // Remove from users list
                if let Some(pos) = self.users.iter().position(|&x| x == account) {
                    self.users.remove(pos);
                }
                
                self.env().emit_event(BalanceSet {
                    user: account,
                    old_balance: balance,
                    new_balance: 0,
                });
            }
            
            Ok(balance)
        }

        /// Clear all data (emergency function)
        #[ink(message)]
        pub fn clear_all(&mut self) -> Result<()> {
            // Clear all user balances
            for &user in &self.users {
                self.balances.remove(user);
            }
            
            // Clear users array
            self.users.clear();
            
            // Reset total supply
            self.total_supply = 0;
            
            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = SimpleStorage::new();
            assert_eq!(contract.get_total_supply(), 0);
            assert_eq!(contract.get_user_count(), 0);
        }

        #[ink::test]
        fn set_balance_works() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.set_balance(accounts.alice, 100), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 100);
            assert_eq!(contract.get_total_supply(), 100);
            assert_eq!(contract.get_user_count(), 1);
        }

        #[ink::test]
        fn named_values_work() {
            let mut contract = SimpleStorage::new();
            
            assert_eq!(contract.set_named_value("score".to_string(), 42), Ok(()));
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.get_named_value(accounts.alice, "score".to_string()), 42);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Set initial balance
            assert_eq!(contract.set_balance(accounts.alice, 100), Ok(()));
            
            // Transfer
            assert_eq!(contract.transfer(accounts.alice, accounts.bob, 30), Ok(()));
            
            assert_eq!(contract.get_balance(accounts.alice), 70);
            assert_eq!(contract.get_balance(accounts.bob), 30);
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.set_balance(accounts.alice, 50), Ok(()));
            assert_eq!(
                contract.transfer(accounts.alice, accounts.bob, 100),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn batch_operations_work() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            let users = vec![accounts.alice, accounts.bob];
            let balances = vec![100, 200];
            
            assert_eq!(contract.batch_set_balances(users, balances), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 100);
            assert_eq!(contract.get_balance(accounts.bob), 200);
            assert_eq!(contract.get_total_supply(), 300);
        }

        #[ink::test]
        fn contains_and_remove_work() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.set_balance(accounts.alice, 100), Ok(()));
            assert!(contract.contains_balance(accounts.alice));
            
            assert_eq!(contract.remove_balance(accounts.alice), Ok(()));
            assert!(!contract.contains_balance(accounts.alice));
            assert_eq!(contract.get_balance(accounts.alice), 0);
        }

        #[ink::test]
        fn take_balance_works() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.set_balance(accounts.alice, 100), Ok(()));
            assert_eq!(contract.take_balance(accounts.alice), Ok(100));
            assert_eq!(contract.get_balance(accounts.alice), 0);
            assert_eq!(contract.get_total_supply(), 0);
        }
    }
}
```

## Key Migration Points

### 1. Storage Structure Differences
**Solidity:**
- Individual state variables at contract level
- Automatic public getters for `public` variables
- Dynamic arrays directly in storage

**ink!:**
- All state in a single `#[ink(storage)]` struct
- Explicit getter functions required
- `Vec<T>` for dynamic arrays

### 2. Mapping Types and Operations
**Solidity:**
```solidity
mapping(address => uint256) public balances;
mapping(address => mapping(string => uint256)) public userNamedValues;

// Usage
balances[account] = value;  // Always works
uint256 balance = balances[account];  // Returns 0 if not set
```

**ink!:**
```rust
balances: Mapping<AccountId, Balance>,
user_named_values: Mapping<(AccountId, String), u128>,  // Tuple keys for nested

// Usage
self.balances.insert(account, &value);
let balance = self.balances.get(account).unwrap_or(0);  // Handle Option
```

### 3. Advanced Mapping Operations
**Solidity:** (Limited operations)
```solidity
// Only basic get/set operations
balances[user] = value;
uint256 value = balances[user];
```

**ink!:** (Rich API)
```rust
// Check existence
if self.balances.contains(account) { ... }

// Remove entry
self.balances.remove(account);

// Take (get and remove)
let value = self.balances.take(account).unwrap_or(0);

// Size information
let size = self.balances.size(account);
```

### 4. Array Management
**Solidity:**
```solidity
address[] public users;
users.push(newUser);
delete users;  // Clears entire array
```

**ink!:**
```rust
users: Vec<AccountId>,
self.users.push(new_user);
self.users.clear();  // Clears entire vector

// Additional operations
if let Some(pos) = self.users.iter().position(|&x| x == account) {
    self.users.remove(pos);  // Remove specific element
}
```

### 5. Nested Mapping Patterns
**Solidity:**
```solidity
mapping(address => mapping(string => uint256)) public userNamedValues;
userNamedValues[user][name] = value;
```

**ink!:**
```rust
user_named_values: Mapping<(AccountId, String), u128>,
self.user_named_values.insert((user, name), &value);  // Tuple key
```

## Migration Steps

### Step 1: Convert Storage Structure
1. Move all state variables into `#[ink(storage)]` struct
2. Replace `mapping(K => V)` with `Mapping<K, V>`
3. Use tuple keys `(K1, K2)` for nested mappings
4. Replace `T[]` with `Vec<T>`

### Step 2: Handle Mapping Default Values
1. Replace direct access with `.get().unwrap_or(default)`
2. Use `.contains()` to check existence
3. Handle `Option<T>` return types properly

### Step 3: Update Storage Operations
1. Use `.insert(key, &value)` instead of assignment
2. Use `.remove(key)` for deletion
3. Use `.take(key)` for get-and-remove operations

### Step 4: Convert Array Operations
1. Use `.push()` for appending
2. Use `.clear()` for clearing all elements
3. Use iterator methods for finding/removing specific elements

### Step 5: Add Error Handling
1. Return `Result<T, Error>` for fallible operations
2. Handle arithmetic overflow/underflow
3. Validate array lengths and inputs

## Common Patterns

### Safe Mapping Updates
```rust
// Get current value, modify, then set
let current = self.mapping.get(key).unwrap_or(0);
let new_value = current.checked_add(increment).ok_or(Error::Overflow)?;
self.mapping.insert(key, &new_value);
```

### User List Management
```rust
// Add user to list if not already present
if !self.users.contains(&account) {
    self.users.push(account);
}

// Remove user from list
if let Some(pos) = self.users.iter().position(|&x| x == account) {
    self.users.remove(pos);
}
```

### Batch Operations
```rust
// Validate inputs first
if accounts.len() != values.len() {
    return Err(Error::ArrayLengthMismatch);
}

// Process batch
for (account, value) in accounts.into_iter().zip(values.into_iter()) {
    self.process_item(account, value)?;
}
```

## Best Practices

### 1. Storage Efficiency
- Use appropriate data types (`u32` vs `u128`)
- Consider storage costs for large vectors
- Cache frequently accessed values

### 2. Error Handling
- Always handle `Option` types from mappings
- Use checked arithmetic for all calculations
- Validate inputs before processing

### 3. Data Consistency
- Keep derived data (like totals) in sync
- Use events to track all state changes
- Consider atomicity of complex operations

### 4. Advanced Features
- Use `contains()` before expensive operations
- Use `take()` for move semantics
- Leverage tuple keys for complex mappings

This migration demonstrates how Solidity's simple mapping and array operations translate to ink!'s more powerful and type-safe storage API, with better error handling and more flexible data access patterns.