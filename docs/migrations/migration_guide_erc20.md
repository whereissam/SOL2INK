# ERC20 Implementation: Solidity vs ink!

## Overview
A comprehensive ERC20 token implementation demonstrating standard token functionality including transfers, approvals, and allowances. This example shows how to implement a complete fungible token contract with proper error handling, events, and all standard ERC20 methods in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleERC20
/// @notice A simple ERC20 token implementation similar to ink! ERC20 example
/// @dev This contract implements basic ERC20 functionality without external dependencies
contract SimpleERC20 {
    // State variables
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    // Mappings
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    // Custom errors
    error InsufficientBalance(address account, uint256 requested, uint256 available);
    error InsufficientAllowance(address spender, uint256 requested, uint256 available);

    /// @notice Constructor to initialize the token
    /// @param _name Token name
    /// @param _symbol Token symbol
    /// @param _decimals Number of decimals
    /// @param _totalSupply Total supply of tokens
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

    /// @notice Transfer tokens from caller to recipient
    /// @param to Recipient address
    /// @param value Amount to transfer
    /// @return success True if transfer succeeded
    function transfer(address to, uint256 value) public returns (bool success) {
        if (balanceOf[msg.sender] < value) {
            revert InsufficientBalance(msg.sender, value, balanceOf[msg.sender]);
        }
        
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    /// @notice Approve spender to spend tokens on behalf of caller
    /// @param spender Address allowed to spend tokens
    /// @param value Amount to approve
    /// @return success True if approval succeeded
    function approve(address spender, uint256 value) public returns (bool success) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    /// @notice Transfer tokens from one address to another using allowance
    /// @param from Address to transfer from
    /// @param to Address to transfer to
    /// @param value Amount to transfer
    /// @return success True if transfer succeeded
    function transferFrom(address from, address to, uint256 value) public returns (bool success) {
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

    /// @notice Increase the allowance granted to spender
    /// @param spender Address to increase allowance for
    /// @param addedValue Amount to add to allowance
    /// @return success True if increase succeeded
    function increaseAllowance(address spender, uint256 addedValue) public returns (bool success) {
        allowance[msg.sender][spender] += addedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }

    /// @notice Decrease the allowance granted to spender
    /// @param spender Address to decrease allowance for
    /// @param subtractedValue Amount to subtract from allowance
    /// @return success True if decrease succeeded
    function decreaseAllowance(address spender, uint256 subtractedValue) public returns (bool success) {
        uint256 currentAllowance = allowance[msg.sender][spender];
        if (currentAllowance < subtractedValue) {
            revert InsufficientAllowance(spender, subtractedValue, currentAllowance);
        }
        
        allowance[msg.sender][spender] = currentAllowance - subtractedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }
}
```

## ink! Implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Token name
        name: Option<String>,
        /// Token symbol
        symbol: Option<String>,
        /// Token decimals
        decimals: Option<u8>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
        /// Returned if arithmetic overflow occurs.
        Overflow,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(
            total_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimals: Option<u8>,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
                name,
                symbol,
                decimals,
            }
        }

        /// Creates a new ERC-20 contract with just initial supply.
        #[ink(constructor)]
        pub fn new_with_supply(total_supply: Balance) -> Self {
            Self::new(total_supply, None, None, None)
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the token name.
        #[ink(message)]
        pub fn token_name(&self) -> Option<String> {
            self.name.clone()
        }

        /// Returns the token symbol.
        #[ink(message)]
        pub fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }

        /// Returns the token decimals.
        #[ink(message)]
        pub fn token_decimals(&self) -> Option<u8> {
            self.decimals
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `balance_of` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `allowance` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with
        /// `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            // We checked that allowance >= value
            self.allowances.insert((&from, &caller), &(allowance - value));
            Ok(())
        }

        /// Increase the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `Overflow` error if the new allowance would overflow.
        #[ink(message)]
        pub fn increase_allowance(&mut self, spender: AccountId, delta: Balance) -> Result<()> {
            let owner = self.env().caller();
            let allowance = self.allowance_impl(&owner, &spender);
            let new_allowance = allowance.checked_add(delta).ok_or(Error::Overflow)?;
            self.allowances.insert((&owner, &spender), &new_allowance);
            self.env().emit_event(Approval {
                owner,
                spender,
                value: new_allowance,
            });
            Ok(())
        }

        /// Decrease the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if the current allowance is less than `delta`.
        #[ink(message)]
        pub fn decrease_allowance(&mut self, spender: AccountId, delta: Balance) -> Result<()> {
            let owner = self.env().caller();
            let allowance = self.allowance_impl(&owner, &spender);
            if allowance < delta {
                return Err(Error::InsufficientAllowance);
            }
            let new_allowance = allowance - delta;
            self.allowances.insert((&owner, &spender), &new_allowance);
            self.env().emit_event(Approval {
                owner,
                spender,
                value: new_allowance,
            });
            Ok(())
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance.checked_add(value).ok_or(Error::Overflow)?));
            
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
            let erc20 = Erc20::new_with_supply(100);
            assert_eq!(erc20.total_supply(), 100);
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.balance_of(accounts.alice), 100);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(erc20.balance_of(accounts.alice), 90);
            assert_eq!(erc20.balance_of(accounts.bob), 10);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(erc20.transfer(accounts.alice, 10), Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn approve_works() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 10);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(erc20.transfer_from(accounts.alice, accounts.charlie, 10), Ok(()));
            
            assert_eq!(erc20.balance_of(accounts.alice), 90);
            assert_eq!(erc20.balance_of(accounts.charlie), 10);
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 0);
        }

        #[ink::test]
        fn increase_allowance_works() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));
            assert_eq!(erc20.increase_allowance(accounts.bob, 5), Ok(()));
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 15);
        }

        #[ink::test]
        fn decrease_allowance_works() {
            let mut erc20 = Erc20::new_with_supply(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));
            assert_eq!(erc20.decrease_allowance(accounts.bob, 5), Ok(()));
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 5);
        }
    }
}
```

## Key Migration Points

### 1. Constructor and Metadata
**Solidity:**
```solidity
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
}
```

**ink!:**
```rust
#[ink(constructor)]
pub fn new(
    total_supply: Balance,
    name: Option<String>,
    symbol: Option<String>,
    decimals: Option<u8>,
) -> Self {
    let mut balances = Mapping::default();
    let caller = Self::env().caller();
    balances.insert(caller, &total_supply);
    Self { total_supply, balances, allowances: Default::default(), name, symbol, decimals }
}
```

### 2. Storage Structure
**Solidity:**
- Individual state variables
- Nested mappings for allowances
- Public variables with automatic getters

**ink!:**
- All state in `#[ink(storage)]` struct
- Tuple keys for nested mappings
- Explicit getter functions

### 3. Events
**Solidity:**
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
emit Transfer(from, to, value);
```

**ink!:**
```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,
}

self.env().emit_event(Transfer { from: Some(from), to: Some(to), value });
```

### 4. Error Handling
**Solidity:**
```solidity
error InsufficientBalance(address account, uint256 requested, uint256 available);
revert InsufficientBalance(msg.sender, value, balanceOf[msg.sender]);
```

**ink!:**
```rust
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    InsufficientBalance,
    InsufficientAllowance,
}

if from_balance < value {
    return Err(Error::InsufficientBalance);
}
```

### 5. Allowance Management
**Solidity:**
```solidity
mapping(address => mapping(address => uint256)) public allowance;
allowance[owner][spender] = value;
```

**ink!:**
```rust
allowances: Mapping<(AccountId, AccountId), Balance>,
self.allowances.insert((&owner, &spender), &value);
```

## Migration Steps

### Step 1: Convert Storage Structure
1. Move all state variables into `#[ink(storage)]` struct
2. Replace nested mappings with tuple keys
3. Add optional metadata fields (name, symbol, decimals)
4. Use `Balance` type instead of `uint256`

### Step 2: Convert Constructor
1. Replace `constructor` with `#[ink(constructor)]`
2. Use `Self::env().caller()` instead of `msg.sender`
3. Initialize mappings with `Mapping::default()`
4. Handle optional metadata parameters

### Step 3: Convert Core Functions
1. Add `#[ink(message)]` to all public functions
2. Replace `bool` return types with `Result<()>`
3. Use explicit error handling instead of `revert`
4. Handle arithmetic operations with overflow checks

### Step 4: Convert Events
1. Create event structs with `#[ink(event)]`
2. Use `Option<AccountId>` for mint/burn events
3. Use `self.env().emit_event()` to emit events
4. Add `#[ink(topic)]` for indexed fields

### Step 5: Handle Allowances
1. Use tuple keys for allowance mapping
2. Implement helper functions for efficiency
3. Update allowances atomically with transfers
4. Add increase/decrease allowance functions

### Step 6: Add Comprehensive Tests
1. Test all ERC20 functions
2. Test error conditions
3. Test edge cases (zero transfers, etc.)
4. Test event emissions

## Common Patterns

### Balance Checks
**Solidity:**
```solidity
if (balanceOf[msg.sender] < value) {
    revert InsufficientBalance(msg.sender, value, balanceOf[msg.sender]);
}
```

**ink!:**
```rust
let from_balance = self.balance_of_impl(from);
if from_balance < value {
    return Err(Error::InsufficientBalance);
}
```

### Allowance Updates
**Solidity:**
```solidity
allowance[from][msg.sender] -= value;
```

**ink!:**
```rust
let allowance = self.allowance_impl(&from, &caller);
self.allowances.insert((&from, &caller), &(allowance - value));
```

### Safe Arithmetic
**Solidity:**
```solidity
// Solidity 0.8+ has built-in overflow protection
balanceOf[to] += value;
```

**ink!:**
```rust
let to_balance = self.balance_of_impl(to);
let new_balance = to_balance.checked_add(value).ok_or(Error::Overflow)?;
self.balances.insert(to, &new_balance);
```

## Best Practices

### 1. Use Efficient Storage Access
- Cache frequently accessed mapping values
- Use reference-based helper functions
- Minimize storage reads/writes

### 2. Proper Error Handling
- Use descriptive error types
- Return `Result<T, Error>` for fallible operations
- Handle arithmetic overflow explicitly

### 3. Event Management
- Use `Option<AccountId>` for mint/burn events
- Emit events after successful state changes
- Use topics sparingly for gas efficiency

### 4. Testing
- Test all standard ERC20 functions
- Test error conditions thoroughly
- Test allowance edge cases
- Verify event emissions

This migration demonstrates how Solidity's ERC20 implementation translates to ink! with better type safety, explicit error handling, and more efficient storage patterns while maintaining full compatibility with the ERC20 standard.