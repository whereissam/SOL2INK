# Flipper Implementation: Solidity vs ink!

## Overview
A simple boolean state contract that can be flipped between true and false. This example demonstrates basic state management, events, access control, and user-specific data in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title Flipper
/// @notice A simple boolean toggle contract similar to ink! flipper example
/// @dev Demonstrates basic boolean state management and user-specific toggles
contract Flipper {
    // State variables
    bool public value;
    address public owner;
    mapping(address => bool) public userValues;
    mapping(address => uint256) public flipCounts;
    uint256 public totalFlips;

    // Events
    event Flipped(address indexed user, bool newValue);
    event GlobalFlipped(address indexed user, bool newValue);
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    /// @notice Constructor sets initial value and owner
    /// @param _initialValue Initial boolean value
    constructor(bool _initialValue) {
        value = _initialValue;
        owner = msg.sender;
        userValues[msg.sender] = _initialValue;
        emit GlobalFlipped(msg.sender, _initialValue);
    }

    /// @notice Flip the global boolean value
    function flip() public {
        value = !value;
        flipCounts[msg.sender]++;
        totalFlips++;
        emit GlobalFlipped(msg.sender, value);
    }

    /// @notice Flip the user's personal boolean value
    function flipPersonal() public {
        userValues[msg.sender] = !userValues[msg.sender];
        flipCounts[msg.sender]++;
        totalFlips++;
        emit Flipped(msg.sender, userValues[msg.sender]);
    }

    /// @notice Get the current global value
    /// @return The current boolean value
    function getValue() public view returns (bool) {
        return value;
    }

    /// @notice Get a user's personal value
    /// @param user The user address
    /// @return The user's boolean value
    function getUserValue(address user) public view returns (bool) {
        return userValues[user];
    }

    /// @notice Get how many times a user has flipped
    /// @param user The user address
    /// @return The number of flips by the user
    function getUserFlipCount(address user) public view returns (uint256) {
        return flipCounts[user];
    }

    /// @notice Get the total number of flips by all users
    /// @return The total number of flips
    function getTotalFlips() public view returns (uint256) {
        return totalFlips;
    }

    /// @notice Set the global value directly (owner only)
    /// @param newValue The new boolean value
    function setValue(bool newValue) public onlyOwner {
        value = newValue;
        emit GlobalFlipped(msg.sender, newValue);
    }

    /// @notice Transfer ownership to a new address
    /// @param newOwner The address of the new owner
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnerChanged(oldOwner, newOwner);
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod flipper {
    use ink::storage::Mapping;

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct Flipper {
        /// Global boolean value
        value: bool,
        /// Contract owner
        owner: AccountId,
        /// User-specific boolean values
        user_values: Mapping<AccountId, bool>,
        /// Track how many times each user has flipped
        flip_counts: Mapping<AccountId, u32>,
        /// Total number of flips by all users
        total_flips: u32,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        user: AccountId,
        new_value: bool,
    }

    #[ink(event)]
    pub struct GlobalFlipped {
        #[ink(topic)]
        user: AccountId,
        new_value: bool,
    }

    #[ink(event)]
    pub struct OwnerChanged {
        #[ink(topic)]
        old_owner: AccountId,
        #[ink(topic)]
        new_owner: AccountId,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        ZeroAddress,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Flipper {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                value: init_value,
                owner: caller,
                user_values: Mapping::default(),
                flip_counts: Mapping::default(),
                total_flips: 0,
            };
            
            // Set the initial user value for the owner
            instance.user_values.insert(caller, &init_value);
            
            // Emit the initial event
            instance.env().emit_event(GlobalFlipped {
                user: caller,
                new_value: init_value,
            });
            
            instance
        }

        /// Creates a new flipper smart contract initialized to `false`
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(false)
        }

        /// Flip the global boolean value
        #[ink(message)]
        pub fn flip(&mut self) {
            let caller = self.env().caller();
            
            // Flip the global value
            self.value = !self.value;
            
            // Update counters
            let current_count = self.flip_counts.get(caller).unwrap_or(0);
            self.flip_counts.insert(caller, &(current_count + 1));
            self.total_flips += 1;
            
            // Emit event
            self.env().emit_event(GlobalFlipped {
                user: caller,
                new_value: self.value,
            });
        }

        /// Flip the user's personal boolean value
        #[ink(message)]
        pub fn flip_personal(&mut self) {
            let caller = self.env().caller();
            
            // Get current user value and flip it
            let current_value = self.user_values.get(caller).unwrap_or(false);
            let new_value = !current_value;
            self.user_values.insert(caller, &new_value);
            
            // Update counters
            let current_count = self.flip_counts.get(caller).unwrap_or(0);
            self.flip_counts.insert(caller, &(current_count + 1));
            self.total_flips += 1;
            
            // Emit event
            self.env().emit_event(Flipped {
                user: caller,
                new_value,
            });
        }

        /// Get the current global value
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Get a user's personal value
        #[ink(message)]
        pub fn get_user_value(&self, user: AccountId) -> bool {
            self.user_values.get(user).unwrap_or(false)
        }

        /// Get how many times a user has flipped
        #[ink(message)]
        pub fn get_user_flip_count(&self, user: AccountId) -> u32 {
            self.flip_counts.get(user).unwrap_or(0)
        }

        /// Get the total number of flips by all users
        #[ink(message)]
        pub fn get_total_flips(&self) -> u32 {
            self.total_flips
        }

        /// Set the global value directly (owner only)
        #[ink(message)]
        pub fn set_value(&mut self, new_value: bool) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            self.value = new_value;
            
            self.env().emit_event(GlobalFlipped {
                user: caller,
                new_value,
            });
            
            Ok(())
        }

        /// Transfer ownership to a new address
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            let old_owner = self.owner;
            self.owner = new_owner;
            
            self.env().emit_event(OwnerChanged {
                old_owner,
                new_owner,
            });
            
            Ok(())
        }

        /// Get the current owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Check if an address is the owner
        #[ink(message)]
        pub fn is_owner(&self, account: AccountId) -> bool {
            account == self.owner
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let flipper = Flipper::new(true);
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::new_default();
            assert_eq!(flipper.get(), false);
        }

        #[ink::test]
        fn flip_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
            assert_eq!(flipper.get_total_flips(), 1);
        }

        #[ink::test]
        fn flip_personal_works() {
            let mut flipper = Flipper::new_default();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            flipper.flip_personal();
            assert_eq!(flipper.get_user_value(accounts.alice), true);
            assert_eq!(flipper.get_user_flip_count(accounts.alice), 1);
        }

        #[ink::test]
        fn set_value_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.set_value(true), Ok(()));
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn set_value_fails_for_non_owner() {
            let mut flipper = Flipper::new_default();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Change the caller to someone else
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            assert_eq!(flipper.set_value(true), Err(Error::NotOwner));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut flipper = Flipper::new_default();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(flipper.transfer_ownership(accounts.bob), Ok(()));
            assert_eq!(flipper.get_owner(), accounts.bob);
        }
    }
}
```

## Key Migration Points

### 1. Storage Structure
**Solidity:**
- State variables declared at contract level
- `mapping(address => bool) public userValues`
- Public variables auto-generate getters

**ink!:**
- All state in a single `#[ink(storage)]` struct
- `user_values: Mapping<AccountId, bool>`
- Must create explicit getter functions

### 2. Access Control
**Solidity:**
```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Only owner can call this function");
    _;
}
```

**ink!:**
```rust
if caller != self.owner {
    return Err(Error::NotOwner);
}
```

### 3. Events
**Solidity:**
```solidity
event Flipped(address indexed user, bool newValue);
emit Flipped(msg.sender, newValue);
```

**ink!:**
```rust
#[ink(event)]
pub struct Flipped {
    #[ink(topic)]
    user: AccountId,
    new_value: bool,
}

self.env().emit_event(Flipped { user: caller, new_value });
```

### 4. Error Handling
**Solidity:**
- Uses `require()` statements that revert on failure
- Custom errors with `revert ErrorName(params)`

**ink!:**
- Uses `Result<T, Error>` pattern
- Custom error enums with descriptive variants
- Returns errors instead of reverting

### 5. Data Types
**Solidity:**
- `address` for user accounts
- `uint256` for counters
- `bool` for boolean values

**ink!:**
- `AccountId` for user accounts
- `u32` for counters (can be `u64`, `u128` as needed)
- `bool` for boolean values

## Migration Steps

### Step 1: Set Up the Contract Structure
1. Replace `contract Flipper` with `#[ink::contract] mod flipper`
2. Move all state variables into an `#[ink(storage)]` struct
3. Convert `mapping` types to `Mapping<K, V>`

### Step 2: Convert Constructor
1. Replace `constructor` with `#[ink(constructor)]`
2. Use `Self::env().caller()` instead of `msg.sender`
3. Initialize `Mapping` fields with `Mapping::default()`

### Step 3: Convert Functions
1. Add `#[ink(message)]` to all public functions
2. Replace `public view` with `&self` parameter
3. Replace `public` with `&mut self` parameter
4. Use `Result<T, Error>` for fallible operations

### Step 4: Convert Events
1. Create event structs with `#[ink(event)]`
2. Add `#[ink(topic)]` for indexed fields
3. Use `self.env().emit_event(EventStruct { ... })`

### Step 5: Handle Errors
1. Define custom error enum
2. Replace `require()` with explicit checks and `Result` returns
3. Handle `Mapping` default values with `unwrap_or(default)`

### Step 6: Add Tests
1. Use `#[ink::test]` for unit tests
2. Use `ink::env::test` utilities for testing different accounts
3. Test both success and error cases

## Common Patterns

### Mapping Access Patterns
**Solidity:**
```solidity
// Automatic zero/false for non-existent keys
bool value = userValues[user];  // Returns false if not set
```

**ink!:**
```rust
// Must handle Option type explicitly
let value = self.user_values.get(user).unwrap_or(false);
```

### Owner-Only Functions
**Solidity:**
```solidity
function setValue(bool newValue) public onlyOwner {
    value = newValue;
}
```

**ink!:**
```rust
#[ink(message)]
pub fn set_value(&mut self, new_value: bool) -> Result<()> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    self.value = new_value;
    Ok(())
}
```

## Best Practices

### 1. Use Appropriate Types
- `AccountId` for addresses
- `u32`/`u64` for counters (instead of `uint256`)
- Explicit error types instead of strings

### 2. Handle Storage Efficiently
- Cache frequently accessed `Mapping` values
- Use `unwrap_or()` for default values
- Consider storage costs for large data structures

### 3. Error Handling
- Always return `Result<T, Error>` for fallible operations
- Use descriptive error variants
- Test all error conditions

### 4. Events and Testing
- Use `#[ink(topic)]` sparingly (only for searchable fields)
- Write comprehensive unit tests
- Test edge cases and error conditions

This migration demonstrates how Solidity's modifier-based access control translates to ink!'s explicit error handling, and how mappings require more careful handling of default values in ink!.