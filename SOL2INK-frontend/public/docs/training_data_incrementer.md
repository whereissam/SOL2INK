# Incrementer Implementation: Solidity vs ink! - Training Data

## Overview
A simple counter contract that demonstrates arithmetic operations and overflow protection. The Solidity version includes advanced features like ownership, per-user values, and batch operations, while the ink! version focuses on simplicity and core increment functionality.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title Incrementer
/// @notice A simple incrementing counter contract similar to ink! incrementer example
/// @dev Demonstrates basic arithmetic operations and overflow protection
contract Incrementer {
    // State variables
    int32 public value;
    address public owner;
    mapping(address => int32) public userValues;
    mapping(address => uint256) public incrementCounts;
    uint256 public totalIncrements;

    // Events
    event ValueIncremented(address indexed user, int32 oldValue, int32 newValue, int32 by);
    event UserValueIncremented(address indexed user, int32 oldValue, int32 newValue, int32 by);
    event ValueSet(address indexed user, int32 newValue);
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    // Custom errors
    error Overflow();
    error Underflow();
    error NotOwner();

    // Modifiers
    modifier onlyOwner() {
        if (msg.sender != owner) {
            revert NotOwner();
        }
        _;
    }

    /// @notice Constructor sets initial value and owner
    /// @param _initialValue Initial integer value
    constructor(int32 _initialValue) {
        value = _initialValue;
        owner = msg.sender;
        userValues[msg.sender] = _initialValue;
        emit ValueSet(msg.sender, _initialValue);
    }

    /// @notice Increment the global value by a given amount
    /// @param by The amount to increment by (can be negative)
    function inc(int32 by) public {
        int32 oldValue = value;
        
        // Check for overflow/underflow
        if (by > 0 && oldValue > type(int32).max - by) {
            revert Overflow();
        }
        if (by < 0 && oldValue < type(int32).min - by) {
            revert Underflow();
        }
        
        value = oldValue + by;
        incrementCounts[msg.sender]++;
        totalIncrements++;
        
        emit ValueIncremented(msg.sender, oldValue, value, by);
    }

    /// @notice Increment the user's personal value by a given amount
    /// @param by The amount to increment by (can be negative)
    function incPersonal(int32 by) public {
        int32 oldValue = userValues[msg.sender];
        
        // Check for overflow/underflow
        if (by > 0 && oldValue > type(int32).max - by) {
            revert Overflow();
        }
        if (by < 0 && oldValue < type(int32).min - by) {
            revert Underflow();
        }
        
        userValues[msg.sender] = oldValue + by;
        incrementCounts[msg.sender]++;
        totalIncrements++;
        
        emit UserValueIncremented(msg.sender, oldValue, userValues[msg.sender], by);
    }

    /// @notice Get the current global value
    /// @return The current integer value
    function get() public view returns (int32) {
        return value;
    }

    /// @notice Get a user's personal value
    /// @param user The user address
    /// @return The user's integer value
    function getUserValue(address user) public view returns (int32) {
        return userValues[user];
    }

    /// @notice Get how many times a user has incremented
    /// @param user The user address
    /// @return The number of increments by the user
    function getUserIncrementCount(address user) public view returns (uint256) {
        return incrementCounts[user];
    }

    /// @notice Get the total number of increments by all users
    /// @return The total number of increments
    function getTotalIncrements() public view returns (uint256) {
        return totalIncrements;
    }

    /// @notice Set the global value directly (owner only)
    /// @param newValue The new integer value
    function setValue(int32 newValue) public onlyOwner {
        value = newValue;
        emit ValueSet(msg.sender, newValue);
    }

    /// @notice Set a user's personal value directly (owner only)
    /// @param user The user address
    /// @param newValue The new integer value
    function setUserValue(address user, int32 newValue) public onlyOwner {
        userValues[user] = newValue;
        emit ValueSet(user, newValue);
    }

    /// @notice Reset the global value to zero (owner only)
    function reset() public onlyOwner {
        value = 0;
        emit ValueSet(msg.sender, 0);
    }

    /// @notice Reset all user values to zero (owner only)
    function resetAll() public onlyOwner {
        // Note: This is a simplified reset. In a real implementation,
        // you'd need to track users to reset their values properly.
        value = 0;
        emit ValueSet(msg.sender, 0);
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

    /// @notice Batch increment for multiple users (owner only)
    /// @param users Array of user addresses
    /// @param amounts Array of amounts to increment by
    function batchIncrement(address[] memory users, int32[] memory amounts) public onlyOwner {
        require(users.length == amounts.length, "Arrays length mismatch");
        
        for (uint256 i = 0; i < users.length; i++) {
            int32 oldValue = userValues[users[i]];
            int32 by = amounts[i];
            
            // Check for overflow/underflow
            if (by > 0 && oldValue > type(int32).max - by) {
                revert Overflow();
            }
            if (by < 0 && oldValue < type(int32).min - by) {
                revert Underflow();
            }
            
            userValues[users[i]] = oldValue + by;
            incrementCounts[users[i]]++;
            totalIncrements++;
            
            emit UserValueIncremented(users[i], oldValue, userValues[users[i]], by);
        }
    }

    /// @notice Safely increment with bounds checking
    /// @param by The amount to increment by
    /// @param minValue The minimum allowed value
    /// @param maxValue The maximum allowed value
    function safeInc(int32 by, int32 minValue, int32 maxValue) public {
        int32 oldValue = value;
        int32 newValue = oldValue + by;
        
        // Check bounds
        require(newValue >= minValue && newValue <= maxValue, "Value out of bounds");
        
        // Check for overflow/underflow
        if (by > 0 && oldValue > type(int32).max - by) {
            revert Overflow();
        }
        if (by < 0 && oldValue < type(int32).min - by) {
            revert Underflow();
        }
        
        value = newValue;
        incrementCounts[msg.sender]++;
        totalIncrements++;
        
        emit ValueIncremented(msg.sender, oldValue, value, by);
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::incrementer::{
    Incrementer,
    IncrementerRef,
};

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value = self.value.checked_add(by).unwrap();
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::new_default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
    }
}
```

## Migration Notes: Solidity Incrementer to ink! Incrementer

### Key Differences:

1. **Overflow Protection**: 
   - Solidity: Manual overflow/underflow checks with custom errors
   - ink!: Built-in `checked_add()` with automatic panic on overflow

2. **Integer Types**:
   - Solidity: `int32` with explicit type declaration
   - ink!: `i32` (Rust's 32-bit signed integer)

3. **Error Handling**:
   - Solidity: Custom errors (`error Overflow()`) with `revert`
   - ink!: Panic on overflow with `unwrap()` or `Result<T, E>` for graceful handling

4. **Constructor Pattern**:
   - Solidity: Single constructor with parameters
   - ink!: Multiple constructors with `#[ink(constructor)]` attribute

5. **Function Modifiers**:
   - Solidity: `modifier onlyOwner()` for access control
   - ink!: Explicit checks within functions or custom access control patterns

6. **State Management**:
   - Solidity: Multiple state variables for complex features
   - ink!: Simple struct focusing on core functionality

### Migration Steps:

1. **Convert Storage Structure**:
   ```solidity
   // Solidity
   int32 public value;
   address public owner;
   mapping(address => int32) public userValues;
   mapping(address => uint256) public incrementCounts;
   uint256 public totalIncrements;
   ```
   
   ```rust
   // ink!
   #[ink(storage)]
   pub struct Incrementer {
       value: i32,
       owner: AccountId,
       user_values: Mapping<AccountId, i32>,
       increment_counts: Mapping<AccountId, u32>,
       total_increments: u32,
   }
   ```

2. **Convert Constructor**:
   ```solidity
   // Solidity
   constructor(int32 _initialValue) {
       value = _initialValue;
       owner = msg.sender;
       userValues[msg.sender] = _initialValue;
       emit ValueSet(msg.sender, _initialValue);
   }
   ```
   
   ```rust
   // ink!
   #[ink(constructor)]
   pub fn new(init_value: i32) -> Self {
       Self { 
           value: init_value,
           owner: Self::env().caller(),
           user_values: Mapping::default(),
           increment_counts: Mapping::default(),
           total_increments: 0,
       }
   }
   ```

3. **Convert Overflow Protection**:
   ```solidity
   // Solidity
   function inc(int32 by) public {
       int32 oldValue = value;
       
       // Check for overflow/underflow
       if (by > 0 && oldValue > type(int32).max - by) {
           revert Overflow();
       }
       if (by < 0 && oldValue < type(int32).min - by) {
           revert Underflow();
       }
       
       value = oldValue + by;
   }
   ```
   
   ```rust
   // ink! - Option 1: Panic on overflow (simple)
   #[ink(message)]
   pub fn inc(&mut self, by: i32) {
       self.value = self.value.checked_add(by).unwrap();
   }
   
   // ink! - Option 2: Graceful error handling
   #[derive(Debug, PartialEq, Eq)]
   #[ink::scale_derive(Encode, Decode, TypeInfo)]
   pub enum Error {
       Overflow,
       Underflow,
   }
   
   #[ink(message)]
   pub fn inc(&mut self, by: i32) -> Result<(), Error> {
       self.value = self.value.checked_add(by).ok_or(Error::Overflow)?;
       Ok(())
   }
   ```

4. **Convert Access Control**:
   ```solidity
   // Solidity
   modifier onlyOwner() {
       if (msg.sender != owner) {
           revert NotOwner();
       }
       _;
   }
   
   function setValue(int32 newValue) public onlyOwner {
       value = newValue;
   }
   ```
   
   ```rust
   // ink!
   #[derive(Debug, PartialEq, Eq)]
   #[ink::scale_derive(Encode, Decode, TypeInfo)]
   pub enum Error {
       NotOwner,
   }
   
   #[ink(message)]
   pub fn set_value(&mut self, new_value: i32) -> Result<(), Error> {
       if self.env().caller() != self.owner {
           return Err(Error::NotOwner);
       }
       self.value = new_value;
       Ok(())
   }
   ```

5. **Convert Events**:
   ```solidity
   // Solidity
   event ValueIncremented(address indexed user, int32 oldValue, int32 newValue, int32 by);
   
   emit ValueIncremented(msg.sender, oldValue, value, by);
   ```
   
   ```rust
   // ink!
   #[ink(event)]
   pub struct ValueIncremented {
       #[ink(topic)]
       user: AccountId,
       old_value: i32,
       new_value: i32,
       by: i32,
   }
   
   self.env().emit_event(ValueIncremented {
       user: self.env().caller(),
       old_value,
       new_value: self.value,
       by,
   });
   ```

6. **Convert Batch Operations**:
   ```solidity
   // Solidity
   function batchIncrement(address[] memory users, int32[] memory amounts) public onlyOwner {
       require(users.length == amounts.length, "Arrays length mismatch");
       
       for (uint256 i = 0; i < users.length; i++) {
           // Process each user
       }
   }
   ```
   
   ```rust
   // ink!
   use ink::prelude::vec::Vec;
   
   #[ink(message)]
   pub fn batch_increment(&mut self, users: Vec<AccountId>, amounts: Vec<i32>) -> Result<(), Error> {
       if users.len() != amounts.len() {
           return Err(Error::ArrayLengthMismatch);
       }
       
       for (user, amount) in users.iter().zip(amounts.iter()) {
           // Process each user
       }
       
       Ok(())
   }
   ```

### Common Patterns:

- **Solidity**: `int32 public value` → **ink!**: `value: i32` (private by default)
- **Solidity**: `require(condition, "message")` → **ink!**: `if !condition { return Err(Error::Custom) }`
- **Solidity**: `type(int32).max` → **ink!**: `i32::MAX`
- **Solidity**: `revert CustomError()` → **ink!**: `Err(Error::Custom)`
- **Solidity**: `msg.sender` → **ink!**: `self.env().caller()`
- **Solidity**: Manual overflow checks → **ink!**: `checked_add()`, `checked_sub()`, etc.

### Advanced ink! Features:

1. **Checked Arithmetic**:
   ```rust
   // Safe increment with overflow protection
   #[ink(message)]
   pub fn safe_inc(&mut self, by: i32) -> Result<(), Error> {
       match by {
           0 => Ok(()),
           positive if positive > 0 => {
               self.value = self.value.checked_add(by).ok_or(Error::Overflow)?;
               Ok(())
           }
           negative => {
               self.value = self.value.checked_sub(negative.abs()).ok_or(Error::Underflow)?;
               Ok(())
           }
       }
   }
   ```

2. **Saturating Arithmetic**:
   ```rust
   // Saturating increment (clamps to min/max instead of panicking)
   #[ink(message)]
   pub fn saturating_inc(&mut self, by: i32) {
       self.value = self.value.saturating_add(by);
   }
   ```

3. **Wrapping Arithmetic**:
   ```rust
   // Wrapping increment (allows overflow/underflow to wrap around)
   #[ink(message)]
   pub fn wrapping_inc(&mut self, by: i32) {
       self.value = self.value.wrapping_add(by);
   }
   ```

## Usage Examples

### Solidity Usage:
```solidity
// Deploy contract
Incrementer counter = new Incrementer(0);

// Basic increment
counter.inc(5);
int32 value = counter.get(); // Returns 5

// Negative increment (decrement)
counter.inc(-2);
value = counter.get(); // Returns 3

// Personal increment
counter.incPersonal(10);
int32 personalValue = counter.getUserValue(msg.sender); // Returns 10

// Owner-only operations
counter.setValue(100);
counter.reset();

// Batch operations
address[] memory users = new address[](2);
users[0] = user1;
users[1] = user2;
int32[] memory amounts = new int32[](2);
amounts[0] = 10;
amounts[1] = 20;
counter.batchIncrement(users, amounts);
```

### ink! Usage:
```rust
// In unit tests
#[ink::test]
fn test_incrementer() {
    let mut contract = Incrementer::new(0);
    assert_eq!(contract.get(), 0);
    
    contract.inc(5);
    assert_eq!(contract.get(), 5);
    
    contract.inc(-2);
    assert_eq!(contract.get(), 3);
}

// In E2E tests
#[ink_e2e::test]
async fn test_incrementer_e2e<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let mut constructor = IncrementerRef::new(0);
    let contract = client
        .instantiate("incrementer", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    
    let mut call_builder = contract.call_builder::<Incrementer>();
    
    // Test increment
    let inc = call_builder.inc(5);
    client.call(&ink_e2e::alice(), &inc).submit().await?;
    
    // Test get
    let get = call_builder.get();
    let value = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    assert_eq!(value.return_value(), 5);
    
    Ok(())
}
```

### Error Handling Examples:

```rust
// Graceful error handling
#[ink(message)]
pub fn safe_inc(&mut self, by: i32) -> Result<(), Error> {
    match self.value.checked_add(by) {
        Some(new_value) => {
            self.value = new_value;
            Ok(())
        }
        None => Err(Error::Overflow),
    }
}

// Using the function
let result = contract.safe_inc(i32::MAX); // Returns Err(Error::Overflow)
```

## Key Takeaways

1. **Overflow Safety**: ink! provides multiple arithmetic options: checked, saturating, and wrapping
2. **Simplicity**: ink! promotes simple, focused contracts with clear error handling
3. **Type Safety**: Rust's type system prevents many common arithmetic errors at compile time
4. **Testing**: Built-in support for both unit tests and E2E tests
5. **Error Handling**: Explicit `Result<T, E>` types for better error management

## Common Questions

**Q: How do I handle overflow in ink!?**
A: Use `checked_add()` for explicit error handling, `saturating_add()` for clamping, or `wrapping_add()` for wrap-around behavior.

**Q: Can I use floating-point numbers in ink!?**
A: ink! contracts should avoid floating-point arithmetic. Use fixed-point arithmetic or integer representations instead.

**Q: How do I implement access control in ink!?**
A: Store an owner field and check `self.env().caller()` in functions. Consider using the `openbrush` library for standard access control patterns.

**Q: What's the difference between `unwrap()` and `Result` handling?**
A: `unwrap()` panics on error (terminates contract execution), while `Result` allows graceful error handling and returning errors to callers.

**Q: How do I test arithmetic operations?**
A: Use `#[ink::test]` for unit tests to verify arithmetic behavior, including edge cases like maximum and minimum values.