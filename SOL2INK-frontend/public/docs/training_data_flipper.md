# Flipper Implementation: Solidity vs ink! - Training Data

## Overview
A simple boolean toggle contract that demonstrates basic state management and user interactions. The Solidity version includes advanced features like ownership, per-user values, and batch operations, while the ink! version focuses on simplicity and core functionality.

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

    /// @notice Set a user's personal value directly (owner only)
    /// @param user The user address
    /// @param newValue The new boolean value
    function setUserValue(address user, bool newValue) public onlyOwner {
        userValues[user] = newValue;
        emit Flipped(user, newValue);
    }

    /// @notice Reset all user values to false (owner only)
    function resetAll() public onlyOwner {
        // Note: This is a simplified reset. In a real implementation,
        // you'd need to track users to reset their values properly.
        value = false;
        emit GlobalFlipped(msg.sender, false);
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

    /// @notice Batch flip for multiple users (owner only)
    /// @param users Array of user addresses
    function batchFlip(address[] memory users) public onlyOwner {
        for (uint256 i = 0; i < users.length; i++) {
            userValues[users[i]] = !userValues[users[i]];
            flipCounts[users[i]]++;
            totalFlips++;
            emit Flipped(users[i], userValues[users[i]]);
        }
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::new_default();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new(false);
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            // when
            let flip = call_builder.flip();
            let _flip_res = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), true));

            Ok(())
        }

        #[ink_e2e::test]
        async fn default_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new_default();

            // when
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            Ok(())
        }
    }
}
```

## Migration Notes: Solidity Flipper to ink! Flipper

### Key Differences:

1. **Storage Structure**: 
   - Solidity: Multiple state variables (`bool public value`, `address public owner`, mappings)
   - ink!: Single struct with `#[ink(storage)]` attribute containing fields

2. **Constructor Pattern**:
   - Solidity: `constructor(bool _initialValue)` function
   - ink!: `#[ink(constructor)]` attribute on methods, can have multiple constructors

3. **Function Visibility**:
   - Solidity: `public`, `view`, `pure` modifiers
   - ink!: `#[ink(message)]` for public functions, `&self` for read-only, `&mut self` for state-changing

4. **Access Control**:
   - Solidity: `modifier onlyOwner()` and `require()` statements
   - ink!: Custom error types and explicit checks within functions

5. **Events**:
   - Solidity: `event` declarations and `emit` statements
   - ink!: `#[ink(event)]` structs and `self.env().emit_event()`

6. **Error Handling**:
   - Solidity: `require()` statements with string messages
   - ink!: `Result<T, E>` return types with custom error enums

### Migration Steps:

1. **Convert Storage Structure**:
   ```solidity
   // Solidity
   bool public value;
   address public owner;
   mapping(address => bool) public userValues;
   ```
   
   ```rust
   // ink!
   #[ink(storage)]
   pub struct Flipper {
       value: bool,
       owner: AccountId,
       user_values: Mapping<AccountId, bool>,
   }
   ```

2. **Convert Constructor**:
   ```solidity
   // Solidity
   constructor(bool _initialValue) {
       value = _initialValue;
       owner = msg.sender;
   }
   ```
   
   ```rust
   // ink!
   #[ink(constructor)]
   pub fn new(init_value: bool) -> Self {
       Self { 
           value: init_value,
           owner: Self::env().caller(),
           user_values: Mapping::default(),
       }
   }
   ```

3. **Convert Functions**:
   ```solidity
   // Solidity
   function flip() public {
       value = !value;
   }
   
   function getValue() public view returns (bool) {
       return value;
   }
   ```
   
   ```rust
   // ink!
   #[ink(message)]
   pub fn flip(&mut self) {
       self.value = !self.value;
   }
   
   #[ink(message)]
   pub fn get(&self) -> bool {
       self.value
   }
   ```

4. **Convert Modifiers to Error Handling**:
   ```solidity
   // Solidity
   modifier onlyOwner() {
       require(msg.sender == owner, "Only owner can call this function");
       _;
   }
   
   function setValue(bool newValue) public onlyOwner {
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
   
   pub type Result<T> = core::result::Result<T, Error>;
   
   #[ink(message)]
   pub fn set_value(&mut self, new_value: bool) -> Result<()> {
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
   event Flipped(address indexed user, bool newValue);
   
   emit Flipped(msg.sender, value);
   ```
   
   ```rust
   // ink!
   #[ink(event)]
   pub struct Flipped {
       #[ink(topic)]
       user: AccountId,
       new_value: bool,
   }
   
   self.env().emit_event(Flipped {
       user: self.env().caller(),
       new_value: self.value,
   });
   ```

### Common Patterns:

- **Solidity**: `msg.sender` → **ink!**: `self.env().caller()`
- **Solidity**: `require(condition, "message")` → **ink!**: `ensure!(condition, Error::CustomError)` or explicit if statements
- **Solidity**: `public` functions → **ink!**: `#[ink(message)]` functions
- **Solidity**: `view` functions → **ink!**: functions with `&self` parameter
- **Solidity**: `mapping(address => type)` → **ink!**: `Mapping<AccountId, type>`

## Usage Examples

### Solidity Usage:
```solidity
// Deploy contract
Flipper flipper = new Flipper(false);

// Basic interactions
flipper.flip();
bool currentValue = flipper.getValue();

// Owner-only operations
flipper.setValue(true);
flipper.transferOwnership(newOwner);

// User-specific operations
flipper.flipPersonal();
bool userValue = flipper.getUserValue(user);
```

### ink! Usage:
```rust
// In your ink! contract tests
#[ink::test]
fn test_flipper() {
    let mut flipper = Flipper::new(false);
    assert!(!flipper.get());
    
    flipper.flip();
    assert!(flipper.get());
    
    flipper.flip();
    assert!(!flipper.get());
}

// E2E test usage
#[ink_e2e::test]
async fn test_flipper_e2e<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let mut constructor = FlipperRef::new(false);
    let contract = client
        .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    
    let mut call_builder = contract.call_builder::<Flipper>();
    
    let flip = call_builder.flip();
    let _flip_res = client
        .call(&ink_e2e::alice(), &flip)
        .submit()
        .await
        .expect("flip failed");
    
    let get = call_builder.get();
    let get_res = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    assert!(matches!(get_res.return_value(), true));
    
    Ok(())
}
```

## Key Takeaways

1. **Simplicity**: ink! promotes simpler, more focused contracts with explicit error handling
2. **Safety**: Rust's type system provides compile-time guarantees and prevents common smart contract vulnerabilities
3. **Testing**: ink! provides both unit testing with `#[ink::test]` and E2E testing with `#[ink_e2e::test]`
4. **Efficiency**: ink! contracts are typically more gas-efficient due to Rust's zero-cost abstractions
5. **Modularity**: ink! encourages composable contract design with clear separation of concerns

## Common Questions

**Q: How do I migrate from Solidity to ink!?**
A: Follow the migration steps above, focusing on storage layout, constructor patterns, function annotations, and error handling. Start with simple contracts like Flipper before moving to complex ones.

**Q: Are there any performance differences?**
A: ink! contracts are generally more gas-efficient due to Rust's zero-cost abstractions and better memory management. The WebAssembly runtime also provides consistent performance across different platforms.

**Q: How do I handle ownership in ink!?**
A: Implement ownership patterns explicitly using storage fields and custom error types. Consider using the `openbrush` library which provides standard ownership implementations.

**Q: Can I use existing Solidity libraries in ink!?**
A: No, you need to use ink!-specific libraries like `openbrush` or implement equivalent functionality in Rust. This often results in more secure and efficient code.

**Q: How do I test ink! contracts?**
A: Use `#[ink::test]` for unit tests and `#[ink_e2e::test]` for end-to-end tests. The testing framework provides powerful tools for simulating real contract interactions.