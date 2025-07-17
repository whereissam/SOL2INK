# Side-by-Side Contract Comparison: Solidity vs ink!

## 🎯 Purpose

This document provides direct, side-by-side comparisons of identical functionality implemented in both Solidity and ink!. Perfect for developers who want to understand the exact differences between the two platforms.

---

## 📋 Table of Contents

1. [Basic Storage Contract](#basic-storage-contract)
2. [ERC20 Token Contract](#erc20-token-contract)
3. [Flipper Contract](#flipper-contract)
4. [Counter Contract](#counter-contract)
5. [Multi-Signature Wallet](#multi-signature-wallet)
6. [Cross-Contract Calls](#cross-contract-calls)
7. [Quick Reference Guide](#quick-reference-guide)

---

## Basic Storage Contract

### 🔍 Functionality
A simple contract that stores a value and allows only the owner to change it.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract SimpleStorage {
    uint256 private storedValue;
    address private owner;
    
    event ValueChanged(
        uint256 indexed oldValue, 
        uint256 indexed newValue
    );
    
    constructor() {
        owner = msg.sender;
        storedValue = 0;
    }
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
    
    function setValue(uint256 value) 
        public onlyOwner {
        uint256 oldValue = storedValue;
        storedValue = value;
        emit ValueChanged(oldValue, value);
    }
    
    function getValue() 
        public view returns (uint256) {
        return storedValue;
    }
    
    function getOwner() 
        public view returns (address) {
        return owner;
    }
}
```

</td>
<td>

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    #[ink(storage)]
    pub struct SimpleStorage {
        stored_value: u128,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct ValueChanged {
        #[ink(topic)]
        old_value: u128,
        #[ink(topic)]
        new_value: u128,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl SimpleStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                stored_value: 0,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn set_value(&mut self, value: u128) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }

            let old_value = self.stored_value;
            self.stored_value = value;
            
            self.env().emit_event(ValueChanged {
                old_value,
                new_value: value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_value(&self) -> u128 {
            self.stored_value
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **Storage** | State variables in contract | `#[ink(storage)]` struct |
| **Access Control** | `modifier onlyOwner()` | Explicit `if` check with `Result` |
| **Events** | `emit ValueChanged(...)` | `self.env().emit_event(ValueChanged {...})` |
| **Constructor** | `constructor()` | `#[ink(constructor)]` |
| **Public Functions** | `function ... public` | `#[ink(message)]` |
| **Error Handling** | `require()` with revert | `Result<T, Error>` return type |

---

## ERC20 Token Contract

### 🔍 Functionality
Standard fungible token with transfer, approve, and allowance functionality.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
contract SimpleERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) 
        public allowance;

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

    error InsufficientBalance(
        address account, 
        uint256 requested, 
        uint256 available
    );

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

    function transfer(address to, uint256 value) 
        public returns (bool) {
        if (balanceOf[msg.sender] < value) {
            revert InsufficientBalance(
                msg.sender, value, balanceOf[msg.sender]
            );
        }
        
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function approve(address spender, uint256 value) 
        public returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }
}
```

</td>
<td>

```rust
#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Erc20 {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

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

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
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

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            Ok(())
        }

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

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **Simple Mapping** | `mapping(address => uint256)` | `Mapping<AccountId, Balance>` |
| **Nested Mapping** | `mapping(address => mapping(address => uint256))` | `Mapping<(AccountId, AccountId), Balance>` |
| **Public Variables** | `string public name;` (auto-getter) | Manual getter: `pub fn name(&self) -> String` |
| **Default Values** | `balanceOf[addr]` returns `0` if not set | `self.balances.get(addr).unwrap_or(0)` |
| **Error Types** | `error InsufficientBalance(...)` | `enum Error { InsufficientBalance }` |
| **Return Types** | `returns (bool)` | `-> Result<()>` |

---

## Flipper Contract

### 🔍 Functionality
Simple boolean state that can be flipped between true and false.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
contract Flipper {
    bool public value;
    
    event Flipped(bool newValue);
    
    constructor() {
        value = false;
    }
    
    function flip() public {
        value = !value;
        emit Flipped(value);
    }
    
    function get() public view returns (bool) {
        return value;
    }
}
```

</td>
<td>

```rust
#[ink::contract]
mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        new_value: bool,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: false }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **State Storage** | `bool public value;` | `value: bool` in storage struct |
| **State Access** | Direct: `value = !value` | Through self: `self.value = !self.value` |
| **Public Getter** | Automatic with `public` | Manual: `pub fn get(&self) -> bool` |

---

## Counter Contract

### 🔍 Functionality
Counter that can be incremented and decremented with overflow protection.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
contract Counter {
    uint256 public count;
    
    event Incremented(uint256 newValue);
    event Decremented(uint256 newValue);
    
    constructor() {
        count = 0;
    }
    
    function increment() public {
        count += 1;
        emit Incremented(count);
    }
    
    function decrement() public {
        require(count > 0, "Cannot decrement below zero");
        count -= 1;
        emit Decremented(count);
    }
    
    function get() public view returns (uint256) {
        return count;
    }
}
```

</td>
<td>

```rust
#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        count: i32,
    }

    #[ink(event)]
    pub struct Incremented {
        #[ink(topic)]
        new_value: i32,
    }

    #[ink(event)]
    pub struct Decremented {
        #[ink(topic)]
        new_value: i32,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        CannotDecrementBelowZero,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Incrementer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { count: 0 }
        }

        #[ink(message)]
        pub fn inc(&mut self) {
            self.count = self.count.saturating_add(1);
            self.env().emit_event(Incremented {
                new_value: self.count,
            });
        }

        #[ink(message)]
        pub fn dec(&mut self) -> Result<()> {
            if self.count <= 0 {
                return Err(Error::CannotDecrementBelowZero);
            }
            
            self.count = self.count.saturating_sub(1);
            self.env().emit_event(Decremented {
                new_value: self.count,
            });
            
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.count
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **Overflow Protection** | Automatic in Solidity 0.8+ | Manual with `saturating_add()/saturating_sub()` |
| **Error Handling** | `require()` with revert | `Result<T, Error>` |
| **Integer Types** | `uint256` (always positive) | `i32` (can be negative) |

---

## Multi-Signature Wallet

### 🔍 Functionality
Wallet requiring multiple signatures to execute transactions.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
contract MultiSigWallet {
    struct Transaction {
        address to;
        uint256 value;
        bytes data;
        bool executed;
        uint256 confirmations;
    }

    address[] public owners;
    mapping(address => bool) public isOwner;
    uint256 public required;
    Transaction[] public transactions;
    mapping(uint256 => mapping(address => bool)) 
        public confirmations;

    event TransactionSubmitted(uint256 indexed txId);
    event TransactionConfirmed(uint256 indexed txId);
    event TransactionExecuted(uint256 indexed txId);

    constructor(address[] memory _owners, uint256 _required) {
        require(_owners.length >= _required, "Invalid requirements");
        require(_required > 0, "Required must be > 0");
        
        for (uint256 i = 0; i < _owners.length; i++) {
            address owner = _owners[i];
            require(owner != address(0), "Invalid owner");
            require(!isOwner[owner], "Owner not unique");
            
            isOwner[owner] = true;
            owners.push(owner);
        }
        
        required = _required;
    }

    modifier onlyOwner() {
        require(isOwner[msg.sender], "Not owner");
        _;
    }

    function submitTransaction(
        address to,
        uint256 value,
        bytes memory data
    ) public onlyOwner returns (uint256) {
        uint256 txId = transactions.length;
        transactions.push(Transaction({
            to: to,
            value: value,
            data: data,
            executed: false,
            confirmations: 0
        }));
        
        emit TransactionSubmitted(txId);
        return txId;
    }

    function confirmTransaction(uint256 txId) public onlyOwner {
        require(txId < transactions.length, "Invalid transaction");
        require(!confirmations[txId][msg.sender], "Already confirmed");
        
        confirmations[txId][msg.sender] = true;
        transactions[txId].confirmations++;
        
        emit TransactionConfirmed(txId);
        
        if (transactions[txId].confirmations >= required) {
            executeTransaction(txId);
        }
    }

    function executeTransaction(uint256 txId) internal {
        Transaction storage transaction = transactions[txId];
        require(!transaction.executed, "Already executed");
        
        transaction.executed = true;
        
        (bool success, ) = transaction.to.call{
            value: transaction.value
        }(transaction.data);
        
        require(success, "Transaction failed");
        emit TransactionExecuted(txId);
    }
}
```

</td>
<td>

```rust
#[ink::contract]
mod multisig {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Transaction {
        pub to: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
        pub executed: bool,
        pub confirmations: u32,
    }

    #[ink(storage)]
    pub struct MultiSig {
        owners: Vec<AccountId>,
        is_owner: Mapping<AccountId, bool>,
        required: u32,
        transactions: Vec<Transaction>,
        confirmations: Mapping<(u32, AccountId), bool>,
        transaction_count: u32,
    }

    #[ink(event)]
    pub struct TransactionSubmitted {
        #[ink(topic)]
        tx_id: u32,
    }

    #[ink(event)]
    pub struct TransactionConfirmed {
        #[ink(topic)]
        tx_id: u32,
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct TransactionExecuted {
        #[ink(topic)]
        tx_id: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        InvalidTransaction,
        AlreadyConfirmed,
        AlreadyExecuted,
        InsufficientConfirmations,
        TransactionFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl MultiSig {
        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>, required: u32) -> Self {
            let mut is_owner = Mapping::new();
            
            for owner in &owners {
                is_owner.insert(owner, &true);
            }
            
            Self {
                owners,
                is_owner,
                required,
                transactions: Vec::new(),
                confirmations: Mapping::new(),
                transaction_count: 0,
            }
        }

        #[ink(message)]
        pub fn submit_transaction(
            &mut self,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if !self.is_owner.get(caller).unwrap_or(false) {
                return Err(Error::NotOwner);
            }

            let tx_id = self.transaction_count;
            self.transaction_count += 1;
            
            let transaction = Transaction {
                to,
                value,
                data,
                executed: false,
                confirmations: 0,
            };
            
            self.transactions.push(transaction);
            
            self.env().emit_event(TransactionSubmitted { tx_id });
            
            Ok(tx_id)
        }

        #[ink(message)]
        pub fn confirm_transaction(&mut self, tx_id: u32) -> Result<()> {
            let caller = self.env().caller();
            
            if !self.is_owner.get(caller).unwrap_or(false) {
                return Err(Error::NotOwner);
            }
            
            if tx_id >= self.transaction_count {
                return Err(Error::InvalidTransaction);
            }
            
            if self.confirmations.get((tx_id, caller)).unwrap_or(false) {
                return Err(Error::AlreadyConfirmed);
            }
            
            self.confirmations.insert((tx_id, caller), &true);
            
            if let Some(ref mut transaction) = self.transactions.get_mut(tx_id as usize) {
                transaction.confirmations += 1;
                
                self.env().emit_event(TransactionConfirmed {
                    tx_id,
                    owner: caller,
                });
                
                if transaction.confirmations >= self.required {
                    self.execute_transaction(tx_id)?;
                }
            }
            
            Ok(())
        }

        fn execute_transaction(&mut self, tx_id: u32) -> Result<()> {
            if let Some(ref mut transaction) = self.transactions.get_mut(tx_id as usize) {
                if transaction.executed {
                    return Err(Error::AlreadyExecuted);
                }
                
                transaction.executed = true;
                
                // In a real implementation, you would execute the transaction here
                // For this example, we'll just mark it as executed
                
                self.env().emit_event(TransactionExecuted { tx_id });
            }
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_transaction(&self, tx_id: u32) -> Option<Transaction> {
            self.transactions.get(tx_id as usize).cloned()
        }

        #[ink(message)]
        pub fn get_owners(&self) -> Vec<AccountId> {
            self.owners.clone()
        }

        #[ink(message)]
        pub fn get_required(&self) -> u32 {
            self.required
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **Arrays** | `Transaction[] public transactions;` | `transactions: Vec<Transaction>` |
| **Nested Mappings** | `mapping(uint256 => mapping(address => bool))` | `Mapping<(u32, AccountId), bool>` |
| **Struct Storage** | Direct access to array elements | `Vec::get_mut()` for modification |
| **Modifiers** | `modifier onlyOwner()` | Explicit checks in each function |

---

## Cross-Contract Calls

### 🔍 Functionality
Contract that calls functions on another contract.

<table>
<tr>
<th width="50%">🔵 Solidity</th>
<th width="50%">🟠 ink!</th>
</tr>
<tr>
<td>

```solidity
interface ITargetContract {
    function getValue() external view returns (uint256);
    function setValue(uint256 _value) external;
}

contract CallerContract {
    ITargetContract public targetContract;
    
    event CalledTarget(uint256 value);
    
    constructor(address _targetContract) {
        targetContract = ITargetContract(_targetContract);
    }
    
    function callSetValue(uint256 _value) external {
        targetContract.setValue(_value);
        emit CalledTarget(_value);
    }
    
    function callGetValue() external view returns (uint256) {
        return targetContract.getValue();
    }
}
```

</td>
<td>

```rust
#[ink::contract]
mod caller_contract {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;

    #[ink(storage)]
    pub struct CallerContract {
        target_contract: AccountId,
    }

    #[ink(event)]
    pub struct CalledTarget {
        #[ink(topic)]
        value: u128,
    }

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

        #[ink(message)]
        pub fn call_set_value(&mut self, value: u128) -> Result<()> {
            let call_result = build_call::<DefaultEnvironment>()
                .call(self.target_contract)
                .gas_limit(5000)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("set_value")))
                        .push_arg(value)
                )
                .returns::<()>()
                .try_invoke();

            match call_result {
                Ok(Ok(_)) => {
                    self.env().emit_event(CalledTarget { value });
                    Ok(())
                }
                _ => Err(Error::CallFailed),
            }
        }

        #[ink(message)]
        pub fn call_get_value(&self) -> Result<u128> {
            let call_result = build_call::<DefaultEnvironment>()
                .call(self.target_contract)
                .gas_limit(5000)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("get_value")))
                )
                .returns::<u128>()
                .try_invoke();

            match call_result {
                Ok(Ok(value)) => Ok(value),
                _ => Err(Error::CallFailed),
            }
        }
    }
}
```

</td>
</tr>
</table>

### 🔄 Key Differences

| Feature | Solidity | ink! |
|---------|----------|------|
| **Interface Definition** | `interface ITargetContract` | No interface needed |
| **Function Calls** | `targetContract.setValue(_value)` | `build_call()` with selector |
| **Return Values** | Direct: `return targetContract.getValue()` | Pattern matching on `Result` |
| **Error Handling** | Automatic revert on failure | Manual `Result` handling |

---

## Quick Reference Guide

### 🔄 Common Patterns

| Pattern | Solidity | ink! |
|---------|----------|------|
| **Contract Definition** | `contract MyContract {` | `#[ink::contract] mod my_contract {` |
| **Storage** | State variables | `#[ink(storage)] pub struct` |
| **Constructor** | `constructor()` | `#[ink(constructor)] pub fn new()` |
| **Public Function** | `function name() public` | `#[ink(message)] pub fn name()` |
| **View Function** | `function name() public view` | `#[ink(message)] pub fn name(&self)` |
| **Events** | `event MyEvent(uint256 value);` | `#[ink(event)] pub struct MyEvent { value: u128 }` |
| **Emit Event** | `emit MyEvent(value);` | `self.env().emit_event(MyEvent { value });` |
| **Errors** | `error MyError();` | `enum Error { MyError }` |
| **Revert** | `revert MyError();` | `return Err(Error::MyError);` |
| **Address** | `address` | `AccountId` |
| **Balance** | `uint256` | `Balance` |
| **Caller** | `msg.sender` | `self.env().caller()` |
| **Mapping** | `mapping(uint256 => address)` | `Mapping<u128, AccountId>` |

### 🎯 Migration Strategy

1. **Start with Storage**: Convert state variables to storage struct
2. **Add Annotations**: Add `#[ink(storage)]`, `#[ink(constructor)]`, `#[ink(message)]`
3. **Handle Errors**: Convert `require()` to `Result<T, Error>`
4. **Update Events**: Convert to struct format with `#[ink(event)]`
5. **Fix Mappings**: Use `Mapping<K, V>` and handle `Option` returns
6. **Test Everything**: Write comprehensive tests with `#[ink::test]`

### 🚀 Best Practices

1. **Use appropriate types**: `AccountId`, `Balance`, `Hash`, etc.
2. **Handle errors gracefully**: Always return `Result<T, Error>`
3. **Cache storage reads**: Don't read the same storage multiple times
4. **Write tests**: Test all functions and error conditions
5. **Follow Rust conventions**: Use snake_case, write documentation

This side-by-side comparison should help you understand the exact differences and make the migration process smoother! 🎯