# From Solidity to ink! - A Complete Migration Tutorial

## 🎯 Introduction

Welcome, Solidity developers! This tutorial will guide you through migrating from Ethereum's Solidity to Polkadot's ink! smart contract framework. We'll use real contract examples to show you the differences, similarities, and best practices.

## 📚 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Key Differences Overview](#key-differences-overview)
3. [Tutorial 1: Simple Storage Contract](#tutorial-1-simple-storage-contract)
4. [Tutorial 2: ERC20 Token Contract](#tutorial-2-erc20-token-contract)
5. [Tutorial 3: Advanced Features](#tutorial-3-advanced-features)
6. [Migration Checklist](#migration-checklist)
7. [Common Pitfalls](#common-pitfalls)
8. [Resources](#resources)

---

## Prerequisites

### What You Need to Know
- ✅ Solidity smart contract development
- ✅ Basic Rust syntax (we'll explain ink!-specific parts)
- ✅ Understanding of blockchain concepts

### Development Environment
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ink! CLI
cargo install --force --locked cargo-contract

# Add WebAssembly target
rustup target add wasm32-unknown-unknown
```

---

## Key Differences Overview

| Aspect | Solidity | ink! |
|--------|----------|------|
| **Language** | Solidity | Rust |
| **Compilation** | EVM bytecode | WebAssembly (WASM) |
| **Memory Management** | Automatic | Rust ownership system |
| **Error Handling** | `require()`, `revert()` | `Result<T, E>` |
| **Storage** | `mapping`, state variables | `Mapping<K, V>`, storage struct |
| **Events** | `emit Event(...)` | `self.env().emit_event(...)` |
| **Access Control** | `msg.sender` | `self.env().caller()` |
| **Deployment** | Gas-based | Weight-based |

---

## Tutorial 1: Simple Storage Contract

Let's start with a basic storage contract to understand the fundamental differences.

### 🔵 Solidity Version

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract SimpleStorage {
    uint256 private storedValue;
    address private owner;
    
    event ValueChanged(uint256 indexed oldValue, uint256 indexed newValue);
    
    constructor() {
        owner = msg.sender;
        storedValue = 0;
    }
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this");
        _;
    }
    
    function setValue(uint256 value) public onlyOwner {
        uint256 oldValue = storedValue;
        storedValue = value;
        emit ValueChanged(oldValue, value);
    }
    
    function getValue() public view returns (uint256) {
        return storedValue;
    }
    
    function getOwner() public view returns (address) {
        return owner;
    }
}
```

### 🟠 ink! Version

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct SimpleStorage {
        stored_value: u128,
        owner: AccountId,
    }

    /// Events that our contract can emit
    #[ink(event)]
    pub struct ValueChanged {
        #[ink(topic)]
        old_value: u128,
        #[ink(topic)]
        new_value: u128,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl SimpleStorage {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                stored_value: 0,
                owner: Self::env().caller(),
            }
        }

        /// Set a new value (only owner can call)
        #[ink(message)]
        pub fn set_value(&mut self, value: u128) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let old_value = self.stored_value;
            self.stored_value = value;
            
            // Emit the event
            self.env().emit_event(ValueChanged {
                old_value,
                new_value: value,
            });

            Ok(())
        }

        /// Get the current value
        #[ink(message)]
        pub fn get_value(&self) -> u128 {
            self.stored_value
        }

        /// Get the owner address
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = SimpleStorage::new();
            assert_eq!(contract.get_value(), 0);
        }

        #[ink::test]
        fn set_value_works() {
            let mut contract = SimpleStorage::new();
            assert_eq!(contract.set_value(42), Ok(()));
            assert_eq!(contract.get_value(), 42);
        }

        #[ink::test]
        fn set_value_fails_for_non_owner() {
            let mut contract = SimpleStorage::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Change the caller to someone else
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            assert_eq!(contract.set_value(42), Err(Error::NotOwner));
        }
    }
}
```

### 🔄 Key Migration Points

1. **Storage Definition**
   - Solidity: State variables declared at contract level
   - ink!: All state in a single `#[ink(storage)]` struct

2. **Access Control**
   - Solidity: `modifier onlyOwner()` with `require()`
   - ink!: Explicit checks returning `Result<T, Error>`

3. **Events**
   - Solidity: `emit Event(params)`
   - ink!: `self.env().emit_event(Event { params })`

4. **Error Handling**
   - Solidity: `require()` statements that revert
   - ink!: `Result<T, E>` pattern with custom error types

---

## Tutorial 2: ERC20 Token Contract

Now let's tackle a more complex example with the ERC20 token standard.

### 🔵 Solidity ERC20

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract SimpleERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address account, uint256 requested, uint256 available);
    error InsufficientAllowance(address spender, uint256 requested, uint256 available);

    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _totalSupply
    ) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _totalSupply;
        balanceOf[msg.sender] = _totalSupply;
        emit Transfer(address(0), msg.sender, _totalSupply);
    }

    function transfer(address to, uint256 value) public returns (bool) {
        if (balanceOf[msg.sender] < value) {
            revert InsufficientBalance(msg.sender, value, balanceOf[msg.sender]);
        }
        
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function approve(address spender, uint256 value) public returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) public returns (bool) {
        if (balanceOf[from] < value) {
            revert InsufficientBalance(from, value, balanceOf[from]);
        }
        if (allowance[from][msg.sender] < value) {
            revert InsufficientAllowance(msg.sender, value, allowance[from][msg.sender]);
        }
        
        balanceOf[from] -= value;
        balanceOf[to] += value;
        allowance[from][msg.sender] -= value;
        emit Transfer(from, to, value);
        return true;
    }
}
```

### 🟠 ink! ERC20

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// The ERC-20 storage struct
    #[ink(storage)]
    pub struct Erc20 {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// ERC-20 events
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// ERC-20 errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the token
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            decimals: u8,
            total_supply: Balance,
        ) -> Self {
            let mut instance = Self {
                name,
                symbol,
                decimals,
                total_supply,
                balances: Mapping::new(),
                allowances: Mapping::new(),
            };

            let caller = instance.env().caller();
            instance.balances.insert(caller, &total_supply);
            
            instance.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            instance
        }

        /// Returns the token name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Returns the token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Returns the token decimals
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }

        /// Returns the total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of an account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        /// Returns the allowance from owner to spender
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        /// Transfers tokens from caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Approve another account to spend tokens on behalf of caller
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            
            Ok(())
        }

        /// Transfer tokens from one account to another using allowance
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            
            self.transfer_from_to(&from, &to, value)?;
            
            // Update allowance
            self.allowances.insert((from, caller), &(allowance - value));
            
            Ok(())
        }

        /// Internal transfer function
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = Erc20::new(
                "Test Token".to_string(),
                "TEST".to_string(),
                18,
                1000,
            );
            assert_eq!(contract.total_supply(), 1000);
            assert_eq!(contract.name(), "Test Token");
            assert_eq!(contract.symbol(), "TEST");
            assert_eq!(contract.decimals(), 18);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(
                "Test Token".to_string(),
                "TEST".to_string(),
                18,
                1000,
            );
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut contract = Erc20::new(
                "Test Token".to_string(),
                "TEST".to_string(),
                18,
                1000,
            );
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Try to transfer more than we have
            assert_eq!(
                contract.transfer(accounts.bob, 2000),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn approve_works() {
            let mut contract = Erc20::new(
                "Test Token".to_string(),
                "TEST".to_string(),
                18,
                1000,
            );
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.approve(accounts.bob, 100), Ok(()));
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 100);
        }
    }
}
```

### 🔄 Key Migration Points for ERC20

1. **Mappings**
   - Solidity: `mapping(address => uint256) public balanceOf;`
   - ink!: `balances: Mapping<AccountId, Balance>`

2. **Nested Mappings**
   - Solidity: `mapping(address => mapping(address => uint256)) public allowance;`
   - ink!: `allowances: Mapping<(AccountId, AccountId), Balance>`

3. **Public Variables**
   - Solidity: `public` variables automatically generate getters
   - ink!: Must create explicit getter functions with `#[ink(message)]`

4. **Error Handling**
   - Solidity: `revert InsufficientBalance(account, requested, available);`
   - ink!: `return Err(Error::InsufficientBalance);`

5. **Event Emission**
   - Solidity: `emit Transfer(from, to, value);`
   - ink!: `self.env().emit_event(Transfer { from: Some(from), to: Some(to), value });`

---

## Tutorial 3: Advanced Features

### Cross-Contract Calls

#### 🔵 Solidity Cross-Contract Call

```solidity
interface ITargetContract {
    function getValue() external view returns (uint256);
    function setValue(uint256 _value) external;
}

contract CallerContract {
    ITargetContract public targetContract;
    
    constructor(address _targetContract) {
        targetContract = ITargetContract(_targetContract);
    }
    
    function callTarget(uint256 _value) external {
        targetContract.setValue(_value);
    }
    
    function getFromTarget() external view returns (uint256) {
        return targetContract.getValue();
    }
}
```

#### 🟠 ink! Cross-Contract Call

```rust
#[ink::contract]
mod caller_contract {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;

    /// Reference to another contract
    #[ink(storage)]
    pub struct CallerContract {
        target_contract: AccountId,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        CallFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl CallerContract {
        #[ink(constructor)]
        pub fn new(target_contract: AccountId) -> Self {
            Self { target_contract }
        }

        /// Call another contract's setValue method
        #[ink(message)]
        pub fn call_target(&mut self, value: u128) -> Result<()> {
            let call = build_call::<DefaultEnvironment>()
                .call(self.target_contract)
                .gas_limit(5000)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("set_value")))
                        .push_arg(value)
                )
                .returns::<()>()
                .try_invoke();

            match call {
                Ok(Ok(_)) => Ok(()),
                _ => Err(Error::CallFailed),
            }
        }

        /// Get value from another contract
        #[ink(message)]
        pub fn get_from_target(&self) -> Result<u128> {
            let call = build_call::<DefaultEnvironment>()
                .call(self.target_contract)
                .gas_limit(5000)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("get_value")))
                )
                .returns::<u128>()
                .try_invoke();

            match call {
                Ok(Ok(value)) => Ok(value),
                _ => Err(Error::CallFailed),
            }
        }
    }
}
```

### Events and Indexing

#### 🔵 Solidity Events

```solidity
event Transfer(
    address indexed from,
    address indexed to,
    uint256 value
);

event Approval(
    address indexed owner,
    address indexed spender,
    uint256 value
);

// Emitting events
emit Transfer(msg.sender, to, value);
```

#### 🟠 ink! Events

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]  // This makes the field indexed
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,  // This is not indexed
}

#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: Balance,
}

// Emitting events
self.env().emit_event(Transfer {
    from: Some(self.env().caller()),
    to: Some(to),
    value,
});
```

---

## Migration Checklist

### ✅ Before You Start

- [ ] Set up Rust development environment
- [ ] Install ink! CLI tools
- [ ] Understand Rust ownership concepts
- [ ] Review your Solidity contract functionality

### ✅ Storage Migration

- [ ] Convert state variables to storage struct
- [ ] Change `mapping` to `Mapping<K, V>`
- [ ] Handle nested mappings with tuple keys
- [ ] Add `#[ink(storage)]` attribute

### ✅ Function Migration

- [ ] Add `#[ink(constructor)]` to constructor
- [ ] Add `#[ink(message)]` to public functions
- [ ] Convert `public view` to `&self` parameters
- [ ] Convert `public` to `&mut self` parameters
- [ ] Replace `msg.sender` with `self.env().caller()`

### ✅ Error Handling

- [ ] Define custom error enum
- [ ] Add error derive attributes
- [ ] Replace `require()` with `ensure!()` or explicit checks
- [ ] Return `Result<T, Error>` from functions

### ✅ Events

- [ ] Convert event definitions to structs
- [ ] Add `#[ink(event)]` attribute
- [ ] Add `#[ink(topic)]` for indexed fields
- [ ] Replace `emit` with `self.env().emit_event()`

### ✅ Testing

- [ ] Write unit tests with `#[ink::test]`
- [ ] Test error conditions
- [ ] Test event emission
- [ ] Add integration tests if needed

---

## Common Pitfalls

### 1. **Mapping Default Values**

❌ **Solidity**: Mappings return `0` for non-existent keys
```solidity
mapping(address => uint256) public balanceOf;
// balanceOf[nonExistentAddress] returns 0
```

✅ **ink!**: Mappings return `Option<T>`, handle with `unwrap_or()`
```rust
balances: Mapping<AccountId, Balance>,
// self.balances.get(account).unwrap_or(0)
```

### 2. **Event Parameter Indexing**

❌ **Wrong**: All parameters indexed
```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    #[ink(topic)]  // Don't index everything!
    value: Balance,
}
```

✅ **Correct**: Only index searchable fields
```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    value: Balance,  // Data, not indexed
}
```

### 3. **Error Handling Patterns**

❌ **Wrong**: Panicking on errors
```rust
pub fn transfer(&mut self, to: AccountId, value: Balance) {
    assert!(self.balance_of(caller) >= value);  // Don't panic!
}
```

✅ **Correct**: Returning Result types
```rust
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
    if self.balance_of(caller) < value {
        return Err(Error::InsufficientBalance);
    }
    Ok(())
}
```

### 4. **Storage Access Patterns**

❌ **Wrong**: Multiple storage reads
```rust
if self.balances.get(from).unwrap_or(0) < value {
    return Err(Error::InsufficientBalance);
}
let new_balance = self.balances.get(from).unwrap_or(0) - value;
```

✅ **Correct**: Cache storage reads
```rust
let from_balance = self.balances.get(from).unwrap_or(0);
if from_balance < value {
    return Err(Error::InsufficientBalance);
}
let new_balance = from_balance - value;
```

---

## Best Practices

### 1. **Use Appropriate Types**
- `AccountId` for addresses
- `Balance` for token amounts
- `Hash` for hashes
- `BlockNumber` for block numbers

### 2. **Optimize Storage Access**
- Cache frequently accessed storage values
- Use `Option<T>` for optional storage items
- Consider storage vs. memory trade-offs

### 3. **Handle Errors Gracefully**
- Always return `Result<T, Error>` for fallible operations
- Use descriptive error types
- Don't panic in production code

### 4. **Write Comprehensive Tests**
- Test all error conditions
- Test event emission
- Use `ink::env::test` utilities
- Test cross-contract interactions

### 5. **Follow Rust Conventions**
- Use snake_case for functions and variables
- Use PascalCase for types and enums
- Write documentation comments
- Use `cargo clippy` for linting

---

## Resources

### 📚 Official Documentation
- [ink! Documentation](https://use.ink/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Documentation](https://polkadot.network/docs/)

### 🛠️ Development Tools
- [cargo-contract](https://github.com/paritytech/cargo-contract)
- [Contracts UI](https://contracts-ui.substrate.io/)
- [Polkadot.js Apps](https://polkadot.js.org/apps/)

### 🌐 Community Resources
- [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- [Polkadot Discord](https://discord.gg/polkadot)
- [ink! GitHub Repository](https://github.com/paritytech/ink)

### 📖 Example Projects
- [OpenBrush](https://github.com/727-Ventures/openbrush-contracts) - ink! library
- [ink! Examples](https://github.com/paritytech/ink-examples)
- [Awesome ink!](https://github.com/paritytech/awesome-ink)

---

## Conclusion

Migrating from Solidity to ink! opens up new possibilities for smart contract development on Polkadot. While there are syntactic differences, the core concepts remain similar. The main advantages of ink! include:

- **Safety**: Rust's type system prevents many common bugs
- **Performance**: WebAssembly execution is fast and efficient
- **Flexibility**: Access to Rust's rich ecosystem
- **Interoperability**: Seamless integration with Polkadot parachains

Take your time to understand the differences, practice with small contracts, and gradually migrate more complex functionality. The ink! ecosystem is growing rapidly, and your Solidity experience provides a solid foundation for success!

**Happy coding! 🚀**

---

*This tutorial is part of the Solidity to ink! migration training system. For more examples and interactive learning, visit our training platform.*