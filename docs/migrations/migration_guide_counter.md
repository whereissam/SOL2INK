# Counter/Incrementer Implementation: Solidity vs ink!

## Overview
A simple counter contract that demonstrates basic arithmetic operations, state management, access control, and event emission. This example shows how to handle numeric state with user tracking and administrative functions.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title Counter
/// @notice A simple counter contract similar to ink! incrementer example
/// @dev Demonstrates basic state management and access control
contract Counter {
    // State variables
    uint256 public number;
    address public owner;
    mapping(address => uint256) public userCounters;
    uint256 public totalIncrements;

    // Events
    event Incremented(address indexed user, uint256 newValue);
    event Decremented(address indexed user, uint256 newValue);
    event NumberSet(address indexed user, uint256 oldValue, uint256 newValue);
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    /// @notice Constructor sets initial number and owner
    /// @param _initialNumber Initial value for the counter
    constructor(uint256 _initialNumber) {
        number = _initialNumber;
        owner = msg.sender;
        emit NumberSet(msg.sender, 0, _initialNumber);
    }

    /// @notice Increment the global counter
    function increment() public {
        number++;
        userCounters[msg.sender]++;
        totalIncrements++;
        emit Incremented(msg.sender, number);
    }

    /// @notice Decrement the global counter
    function decrement() public {
        require(number > 0, "Counter cannot go below zero");
        number--;
        emit Decremented(msg.sender, number);
    }

    /// @notice Set the counter to a specific value (owner only)
    /// @param newNumber The new value for the counter
    function setNumber(uint256 newNumber) public onlyOwner {
        uint256 oldNumber = number;
        number = newNumber;
        emit NumberSet(msg.sender, oldNumber, newNumber);
    }

    /// @notice Get the current counter value
    /// @return The current counter value
    function getNumber() public view returns (uint256) {
        return number;
    }

    /// @notice Get how many times a user has incremented
    /// @param user The user address
    /// @return The number of times the user has incremented
    function getUserIncrements(address user) public view returns (uint256) {
        return userCounters[user];
    }

    /// @notice Get the total number of increments by all users
    /// @return The total number of increments
    function getTotalIncrements() public view returns (uint256) {
        return totalIncrements;
    }

    /// @notice Increment by a specific amount
    /// @param amount The amount to increment by
    function incrementBy(uint256 amount) public {
        number += amount;
        userCounters[msg.sender] += amount;
        totalIncrements += amount;
        emit Incremented(msg.sender, number);
    }

    /// @notice Reset the counter to zero (owner only)
    function reset() public onlyOwner {
        uint256 oldNumber = number;
        number = 0;
        emit NumberSet(msg.sender, oldNumber, 0);
    }

    /// @notice Transfer ownership to a new address
    /// @param newOwner The address of the new owner
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnerChanged(oldOwner, newOwner);
    }

    /// @notice Check if an address is the owner
    /// @param account The address to check
    /// @return True if the address is the owner
    function isOwner(address account) public view returns (bool) {
        return account == owner;
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct Incrementer {
        /// Current counter value
        value: i32,
        /// Contract owner
        owner: AccountId,
        /// Track increments per user
        user_counters: Mapping<AccountId, u32>,
        /// Total number of increments across all users
        total_increments: u32,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct Incremented {
        #[ink(topic)]
        user: AccountId,
        new_value: i32,
    }

    #[ink(event)]
    pub struct Decremented {
        #[ink(topic)]
        user: AccountId,
        new_value: i32,
    }

    #[ink(event)]
    pub struct NumberSet {
        #[ink(topic)]
        user: AccountId,
        old_value: i32,
        new_value: i32,
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
        CounterUnderflow,
        ArithmeticOverflow,
        ZeroAddress,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Incrementer {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                value: init_value,
                owner: caller,
                user_counters: Mapping::default(),
                total_increments: 0,
            };
            
            // Emit initial event
            instance.env().emit_event(NumberSet {
                user: caller,
                old_value: 0,
                new_value: init_value,
            });
            
            instance
        }

        /// Creates a new incrementer initialized to 0
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(0)
        }

        /// Increment the counter by 1
        #[ink(message)]
        pub fn inc(&mut self) -> Result<()> {
            self.inc_by(1)
        }

        /// Increment the counter by a specific amount
        #[ink(message)]
        pub fn inc_by(&mut self, by: i32) -> Result<()> {
            let caller = self.env().caller();
            
            // Check for overflow
            self.value = self.value
                .checked_add(by)
                .ok_or(Error::ArithmeticOverflow)?;
            
            // Update user counter
            let user_count = self.user_counters.get(caller).unwrap_or(0);
            self.user_counters.insert(caller, &(user_count + by.abs() as u32));
            
            // Update total increments
            self.total_increments = self.total_increments
                .checked_add(by.abs() as u32)
                .ok_or(Error::ArithmeticOverflow)?;
            
            // Emit event
            self.env().emit_event(Incremented {
                user: caller,
                new_value: self.value,
            });
            
            Ok(())
        }

        /// Decrement the counter by 1
        #[ink(message)]
        pub fn dec(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            // Check for underflow (assuming we want to prevent negative values)
            if self.value <= 0 {
                return Err(Error::CounterUnderflow);
            }
            
            self.value -= 1;
            
            // Emit event
            self.env().emit_event(Decremented {
                user: caller,
                new_value: self.value,
            });
            
            Ok(())
        }

        /// Set the counter to a specific value (owner only)
        #[ink(message)]
        pub fn set(&mut self, new_value: i32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            let old_value = self.value;
            self.value = new_value;
            
            self.env().emit_event(NumberSet {
                user: caller,
                old_value,
                new_value,
            });
            
            Ok(())
        }

        /// Get the current counter value
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// Get how many times a user has incremented
        #[ink(message)]
        pub fn get_user_increments(&self, user: AccountId) -> u32 {
            self.user_counters.get(user).unwrap_or(0)
        }

        /// Get the total number of increments by all users
        #[ink(message)]
        pub fn get_total_increments(&self) -> u32 {
            self.total_increments
        }

        /// Reset the counter to zero (owner only)
        #[ink(message)]
        pub fn reset(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            let old_value = self.value;
            self.value = 0;
            
            self.env().emit_event(NumberSet {
                user: caller,
                old_value,
                new_value: 0,
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
            let contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
        }

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::new_default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn increment_works() {
            let mut contract = Incrementer::new(5);
            assert_eq!(contract.inc(), Ok(()));
            assert_eq!(contract.get(), 6);
        }

        #[ink::test]
        fn increment_by_works() {
            let mut contract = Incrementer::new(5);
            assert_eq!(contract.inc_by(10), Ok(()));
            assert_eq!(contract.get(), 15);
            assert_eq!(contract.get_total_increments(), 10);
        }

        #[ink::test]
        fn decrement_works() {
            let mut contract = Incrementer::new(5);
            assert_eq!(contract.dec(), Ok(()));
            assert_eq!(contract.get(), 4);
        }

        #[ink::test]
        fn decrement_underflow_fails() {
            let mut contract = Incrementer::new(0);
            assert_eq!(contract.dec(), Err(Error::CounterUnderflow));
        }

        #[ink::test]
        fn set_works() {
            let mut contract = Incrementer::new(5);
            assert_eq!(contract.set(100), Ok(()));
            assert_eq!(contract.get(), 100);
        }

        #[ink::test]
        fn set_fails_for_non_owner() {
            let mut contract = Incrementer::new(5);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Change the caller to someone else
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            assert_eq!(contract.set(100), Err(Error::NotOwner));
        }

        #[ink::test]
        fn reset_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.reset(), Ok(()));
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = Incrementer::new(5);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.transfer_ownership(accounts.bob), Ok(()));
            assert_eq!(contract.get_owner(), accounts.bob);
        }

        #[ink::test]
        fn user_tracking_works() {
            let mut contract = Incrementer::new(0);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Alice increments
            assert_eq!(contract.inc_by(5), Ok(()));
            assert_eq!(contract.get_user_increments(accounts.alice), 5);
            
            // Switch to Bob and increment
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.inc_by(3), Ok(()));
            assert_eq!(contract.get_user_increments(accounts.bob), 3);
            assert_eq!(contract.get_total_increments(), 8);
        }
    }
}
```

## Key Migration Points

### 1. Data Types and Arithmetic
**Solidity:**
- Uses `uint256` (unsigned integers)
- Automatic overflow protection in ^0.8.0
- Simple arithmetic operations

**ink!:**
- Uses `i32` (can be negative)
- Explicit overflow checking with `checked_add()`
- More control over arithmetic safety

### 2. Error Handling for Arithmetic
**Solidity:**
```solidity
require(number > 0, "Counter cannot go below zero");
number--;
```

**ink!:**
```rust
if self.value <= 0 {
    return Err(Error::CounterUnderflow);
}
self.value -= 1;

// Or with checked arithmetic
self.value = self.value
    .checked_add(by)
    .ok_or(Error::ArithmeticOverflow)?;
```

### 3. State Updates with Events
**Solidity:**
```solidity
function increment() public {
    number++;
    userCounters[msg.sender]++;
    totalIncrements++;
    emit Incremented(msg.sender, number);
}
```

**ink!:**
```rust
#[ink(message)]
pub fn inc(&mut self) -> Result<()> {
    self.value = self.value.checked_add(1).ok_or(Error::ArithmeticOverflow)?;
    
    let caller = self.env().caller();
    let user_count = self.user_counters.get(caller).unwrap_or(0);
    self.user_counters.insert(caller, &(user_count + 1));
    
    self.env().emit_event(Incremented {
        user: caller,
        new_value: self.value,
    });
    
    Ok(())
}
```

### 4. Access Control Patterns
**Solidity:**
```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Only owner can call this function");
    _;
}

function setNumber(uint256 newNumber) public onlyOwner {
    // function body
}
```

**ink!:**
```rust
#[ink(message)]
pub fn set(&mut self, new_value: i32) -> Result<()> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    // function body
    Ok(())
}
```

## Migration Steps

### Step 1: Choose Appropriate Data Types
1. **Consider signedness**: `uint256` vs `i32`/`u32`
2. **Choose bit width**: `u32` often sufficient instead of `uint256`
3. **Plan for arithmetic safety**: Use checked operations

### Step 2: Convert Storage Structure
1. Move all state variables into `#[ink(storage)]` struct
2. Convert `mapping(address => uint256)` to `Mapping<AccountId, u32>`
3. Initialize mappings with `Mapping::default()`

### Step 3: Handle Arithmetic Operations
1. Replace simple `++` with `checked_add(1)`
2. Handle overflow/underflow explicitly
3. Return `Result<T, Error>` for operations that can fail

### Step 4: Convert Access Control
1. Remove Solidity modifiers
2. Add explicit checks in function bodies
3. Return appropriate error types

### Step 5: Update Event Emission
1. Define event structs with `#[ink(event)]`
2. Use `#[ink(topic)]` for searchable fields
3. Call `self.env().emit_event(Event { ... })`

## Common Patterns

### Safe Arithmetic in ink!
```rust
// Instead of: self.value += amount;
self.value = self.value
    .checked_add(amount)
    .ok_or(Error::ArithmeticOverflow)?;

// For mapping updates
let current = self.mapping.get(key).unwrap_or(0);
self.mapping.insert(key, &(current + amount));
```

### Owner-Only Functions
```rust
#[ink(message)]
pub fn admin_function(&mut self) -> Result<()> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    // Admin logic here
    Ok(())
}
```

### Event Emission with Topics
```rust
#[ink(event)]
pub struct ValueChanged {
    #[ink(topic)]  // Indexed - searchable
    user: AccountId,
    old_value: i32,  // Not indexed - data only
    new_value: i32,  // Not indexed - data only
}
```

## Best Practices

### 1. Arithmetic Safety
- Always use checked arithmetic for user inputs
- Consider using `saturating_add()` for cases where overflow should clamp
- Define clear overflow/underflow behavior

### 2. Error Handling
- Create specific error types for different failure modes
- Return `Result<T, Error>` from all fallible operations
- Test error conditions thoroughly

### 3. Data Type Selection
- Use appropriate bit widths (`u32` vs `u128`)
- Consider whether signed or unsigned integers are needed
- Document any constraints or assumptions

### 4. Storage Efficiency
- Cache frequently accessed mapping values
- Consider gas/storage costs of complex operations
- Use efficient data structures

This migration demonstrates how Solidity's simple arithmetic operations translate to ink!'s more explicit safety-focused approach, and how to handle both overflow protection and access control in a type-safe manner.