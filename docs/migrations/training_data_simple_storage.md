# SimpleStorage Implementation: Solidity vs ink! - Training Data

## Overview
A storage contract that demonstrates mapping functionality and data management. The Solidity version includes complex features like nested mappings, arrays, and batch operations, while the ink! version focuses on demonstrating various Mapping API methods and error handling.

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
//! A smart contract which demonstrates functionality of `Mapping` functions.

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod mapping {
    use ink::{
        prelude::{
            string::String,
            vec::Vec,
        },
        storage::Mapping,
    };

    #[derive(Debug, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ContractError {
        ValueTooLarge,
    }

    /// A contract for testing `Mapping` functionality.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Mappings {
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping from owner to aliases.
        names: Mapping<AccountId, Vec<String>>,
    }

    impl Mappings {
        /// Demonstrates the usage of `Mappings::default()`
        ///
        /// Creates an empty mapping between accounts and balances.
        #[ink(constructor)]
        pub fn new() -> Self {
            let balances = Mapping::default();
            let names = Mapping::default();
            Self { balances, names }
        }

        /// Demonstrates the usage of `Mapping::get()`.
        ///
        /// Returns the balance of a account, or `None` if the account is not in the
        /// `Mapping`.
        #[ink(message)]
        pub fn get_balance(&self) -> Option<Balance> {
            let caller = Self::env().caller();
            self.balances.get(caller)
        }

        /// Demonstrates the usage of `Mappings::insert()`.
        ///
        /// Assigns the value to a given account.
        ///
        /// Returns the size of the pre-existing balance at the specified key if any.
        /// Returns `None` if the account was not previously in the `Mapping`.
        #[ink(message)]
        pub fn insert_balance(&mut self, value: Balance) -> Option<u32> {
            let caller = Self::env().caller();
            self.balances.insert(caller, &value)
        }

        /// Demonstrates the usage of `Mappings::size()`.
        ///
        /// Returns the size of the pre-existing balance at the specified key if any.
        /// Returns `None` if the account was not previously in the `Mapping`.
        #[ink(message)]
        pub fn size_balance(&mut self) -> Option<u32> {
            let caller = Self::env().caller();
            self.balances.size(caller)
        }

        /// Demonstrates the usage of `Mapping::contains()`.
        ///
        /// Returns `true` if the account has any balance assigned to it.
        #[ink(message)]
        pub fn contains_balance(&self) -> bool {
            let caller = Self::env().caller();
            self.balances.contains(caller)
        }

        /// Demonstrates the usage of `Mappings::remove()`.
        ///
        /// Removes the balance entry for a given account.
        #[ink(message)]
        pub fn remove_balance(&mut self) {
            let caller = Self::env().caller();
            self.balances.remove(caller);
        }

        /// Demonstrates the usage of `Mappings::take()`.
        ///
        /// Returns the balance of a given account removing it from storage.
        ///
        /// Returns `None` if the account is not in the `Mapping`.
        #[ink(message)]
        pub fn take_balance(&mut self) -> Option<Balance> {
            let caller = Self::env().caller();
            self.balances.take(caller)
        }

        /// Demonstrates the usage of `Mappings::try_take()` and `Mappings::try_insert()`.
        ///
        /// Adds a name of a given account.
        ///
        /// Returns `Ok(None)` if the account is not in the `Mapping`.
        /// Returns `Ok(Some(_))` if the account was already in the `Mapping`
        /// Returns `Err(_)` if the mapping value couldn't be encoded.
        #[ink(message)]
        pub fn try_insert_name(&mut self, name: String) -> Result<(), ContractError> {
            let caller = Self::env().caller();
            let mut names = match self.names.try_take(caller) {
                None => Vec::new(),
                Some(value) => value.map_err(|_| ContractError::ValueTooLarge)?,
            };

            names.push(name);

            self.names
                .try_insert(caller, &names)
                .map_err(|_| ContractError::ValueTooLarge)?;

            Ok(())
        }

        /// Demonstrates the usage of `Mappings::try_get()`.
        ///
        /// Returns the name of a given account.
        ///
        /// Returns `Ok(None)` if the account is not in the `Mapping`.
        /// Returns `Ok(Some(_))` if the account was already in the `Mapping`
        /// Returns `Err(_)` if the mapping value couldn't be encoded.
        #[ink(message)]
        pub fn try_get_names(&mut self) -> Option<Result<Vec<String>, ContractError>> {
            let caller = Self::env().caller();
            self.names
                .try_get(caller)
                .map(|result| result.map_err(|_| ContractError::ValueTooLarge))
        }
    }
}
```

## Migration Notes: Solidity SimpleStorage to ink! Mapping

### Key Differences:

1. **Storage Structure**: 
   - Solidity: Multiple state variables with different types and public visibility
   - ink!: Single struct with `#[ink(storage)]` and `Mapping` types

2. **Data Types**:
   - Solidity: `mapping(address => uint256)`, `address[]`, `uint256`
   - ink!: `Mapping<AccountId, Balance>`, `Vec<String>`, `Balance`

3. **Error Handling**:
   - Solidity: `require()` statements with revert messages
   - ink!: `Result<T, E>` types with custom error enums

4. **Function Patterns**:
   - Solidity: Direct mapping access `balances[account]`
   - ink!: Method calls `self.balances.get(account)`

5. **Visibility & Access**:
   - Solidity: `public` state variables create automatic getters
   - ink!: Explicit `#[ink(message)]` functions for public access

6. **Events**:
   - Solidity: `event` declarations with `emit` statements
   - ink!: `#[ink(event)]` structs with `self.env().emit_event()`

### Migration Steps:

1. **Convert Storage Structure**:
   ```solidity
   // Solidity
   mapping(address => uint256) public balances;
   mapping(address => mapping(string => uint256)) public userNamedValues;
   address[] public users;
   uint256 public totalSupply;
   ```
   
   ```rust
   // ink!
   #[ink(storage)]
   pub struct Storage {
       balances: Mapping<AccountId, Balance>,
       user_named_values: Mapping<(AccountId, String), Balance>,
       users: Vec<AccountId>,
       total_supply: Balance,
   }
   ```

2. **Convert Basic Operations**:
   ```solidity
   // Solidity
   function setBalance(address account, uint256 balance) public {
       balances[account] = balance;
   }
   
   function getBalance(address account) public view returns (uint256) {
       return balances[account];
   }
   ```
   
   ```rust
   // ink!
   #[ink(message)]
   pub fn set_balance(&mut self, account: AccountId, balance: Balance) {
       self.balances.insert(account, &balance);
   }
   
   #[ink(message)]
   pub fn get_balance(&self, account: AccountId) -> Option<Balance> {
       self.balances.get(account)
   }
   ```

3. **Convert Error Handling**:
   ```solidity
   // Solidity
   function transfer(address from, address to, uint256 amount) public {
       require(balances[from] >= amount, "Insufficient balance");
       balances[from] -= amount;
       balances[to] += amount;
   }
   ```
   
   ```rust
   // ink!
   #[derive(Debug, PartialEq, Eq)]
   #[ink::scale_derive(Encode, Decode, TypeInfo)]
   pub enum Error {
       InsufficientBalance,
   }
   
   #[ink(message)]
   pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), Error> {
       let from_balance = self.balances.get(from).unwrap_or(0);
       if from_balance < amount {
           return Err(Error::InsufficientBalance);
       }
       
       self.balances.insert(from, &(from_balance - amount));
       let to_balance = self.balances.get(to).unwrap_or(0);
       self.balances.insert(to, &(to_balance + amount));
       
       Ok(())
   }
   ```

4. **Convert Nested Mappings**:
   ```solidity
   // Solidity
   mapping(address => mapping(string => uint256)) public userNamedValues;
   
   function setNamedValue(string memory name, uint256 value) public {
       userNamedValues[msg.sender][name] = value;
   }
   ```
   
   ```rust
   // ink!
   use ink::prelude::string::String;
   
   // In storage struct
   user_named_values: Mapping<(AccountId, String), Balance>,
   
   #[ink(message)]
   pub fn set_named_value(&mut self, name: String, value: Balance) {
       let caller = self.env().caller();
       self.user_named_values.insert((caller, name), &value);
   }
   ```

5. **Convert Array Operations**:
   ```solidity
   // Solidity
   address[] public users;
   
   function getUsers() public view returns (address[] memory) {
       return users;
   }
   
   function getUserCount() public view returns (uint256) {
       return users.length;
   }
   ```
   
   ```rust
   // ink!
   use ink::prelude::vec::Vec;
   
   // In storage struct
   users: Vec<AccountId>,
   
   #[ink(message)]
   pub fn get_users(&self) -> Vec<AccountId> {
       self.users.clone()
   }
   
   #[ink(message)]
   pub fn get_user_count(&self) -> u32 {
       self.users.len() as u32
   }
   ```

6. **Convert Batch Operations**:
   ```solidity
   // Solidity
   function batchSetBalances(address[] memory accounts, uint256[] memory newBalances) public {
       require(accounts.length == newBalances.length, "Arrays length mismatch");
       
       for (uint256 i = 0; i < accounts.length; i++) {
           setBalance(accounts[i], newBalances[i]);
       }
   }
   ```
   
   ```rust
   // ink!
   #[ink(message)]
   pub fn batch_set_balances(&mut self, accounts: Vec<AccountId>, new_balances: Vec<Balance>) -> Result<(), Error> {
       if accounts.len() != new_balances.len() {
           return Err(Error::ArrayLengthMismatch);
       }
       
       for (account, balance) in accounts.iter().zip(new_balances.iter()) {
           self.balances.insert(*account, balance);
       }
       
       Ok(())
   }
   ```

### Common Patterns:

- **Solidity**: `mapping[key] = value` → **ink!**: `mapping.insert(key, &value)`
- **Solidity**: `mapping[key]` → **ink!**: `mapping.get(key).unwrap_or(default)`
- **Solidity**: `require(condition, "message")` → **ink!**: `if !condition { return Err(Error::Custom) }`
- **Solidity**: `msg.sender` → **ink!**: `self.env().caller()`
- **Solidity**: `address[] memory` → **ink!**: `Vec<AccountId>`
- **Solidity**: `string memory` → **ink!**: `String` (with prelude import)

### Advanced ink! Mapping Features:

1. **Safe Operations**:
   ```rust
   // Check if key exists
   if self.balances.contains(account) {
       // Safe to get value
       let balance = self.balances.get(account).unwrap();
   }
   
   // Get size of stored value
   let size = self.balances.size(account);
   
   // Remove entry
   self.balances.remove(account);
   
   // Take value (get and remove)
   let value = self.balances.take(account);
   ```

2. **Fallible Operations**:
   ```rust
   // Try operations for large values
   match self.large_data.try_get(key) {
       Some(Ok(value)) => {
           // Successfully retrieved
       },
       Some(Err(_)) => {
           // Value too large to decode
           return Err(Error::ValueTooLarge);
       },
       None => {
           // Key not found
       }
   }
   ```

## Usage Examples

### Solidity Usage:
```solidity
// Deploy contract
SimpleStorage storage = new SimpleStorage();

// Set balances
storage.setBalance(user1, 1000);
storage.setBalance(user2, 2000);

// Get balance
uint256 balance = storage.getBalance(user1);

// Set named values
storage.setNamedValue("score", 100);
uint256 score = storage.getNamedValue(user1, "score");

// Transfer
storage.transfer(user1, user2, 500);

// Batch operations
address[] memory accounts = new address[](2);
accounts[0] = user1;
accounts[1] = user2;
uint256[] memory balances = new uint256[](2);
balances[0] = 1500;
balances[1] = 2500;
storage.batchSetBalances(accounts, balances);
```

### ink! Usage:
```rust
// In unit tests
#[ink::test]
fn test_mapping_operations() {
    let mut contract = Mappings::new();
    
    // Insert balance
    contract.insert_balance(1000);
    
    // Get balance
    let balance = contract.get_balance();
    assert_eq!(balance, Some(1000));
    
    // Check if contains
    assert!(contract.contains_balance());
    
    // Remove balance
    contract.remove_balance();
    assert!(!contract.contains_balance());
}

// In E2E tests
#[ink_e2e::test]
async fn test_mapping_e2e<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let mut constructor = MappingsRef::new();
    let contract = client
        .instantiate("mapping", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    
    let mut call_builder = contract.call_builder::<Mappings>();
    
    // Insert balance
    let insert = call_builder.insert_balance(1000);
    client.call(&ink_e2e::alice(), &insert).submit().await?;
    
    // Get balance
    let get = call_builder.get_balance();
    let balance = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    assert_eq!(balance.return_value(), Some(1000));
    
    Ok(())
}
```

## Key Takeaways

1. **Storage Patterns**: ink! uses `Mapping<K, V>` for key-value storage with explicit method calls
2. **Error Handling**: ink! promotes explicit error handling with `Result<T, E>` types
3. **Memory Management**: ink! provides better control over storage operations with size limits
4. **Type Safety**: Rust's type system prevents many common storage-related bugs
5. **Fallible Operations**: ink! provides `try_*` methods for handling large values safely

## Common Questions

**Q: How do I handle nested mappings in ink!?**
A: Use tuple keys: `Mapping<(AccountId, String), Balance>` instead of `mapping(address => mapping(string => uint256))`

**Q: How do I iterate over mappings in ink!?**
A: ink! Mappings don't support iteration by design. Store keys in a separate `Vec` if iteration is needed.

**Q: How do I handle large values in mappings?**
A: Use `try_get()`, `try_insert()`, and `try_take()` methods which return `Result` types for values that might exceed storage limits.

**Q: Can I make mappings public like in Solidity?**
A: No, you need to create explicit getter functions with `#[ink(message)]`. This provides better control over access patterns.

**Q: How do I check if a key exists in a mapping?**
A: Use the `contains()` method: `self.balances.contains(account)` returns `bool`.