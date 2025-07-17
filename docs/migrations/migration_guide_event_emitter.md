# Event Emitter Implementation: Solidity vs ink!

## Overview
A comprehensive event management contract demonstrating various event patterns, indexing strategies, anonymous events, and event tracking. This example shows how to handle different types of events, batch operations, and event analytics in both blockchain platforms.

## Solidity Implementation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title EventEmitter
/// @notice A contract demonstrating various event patterns similar to ink! events example
/// @dev Shows different types of events and event filtering
contract EventEmitter {
    // State variables
    address public owner;
    uint256 public eventCount;
    mapping(bytes32 => uint256) public eventTypeCount;

    // Events with different indexing patterns
    event SimpleEvent(string message);
    event UserAction(address indexed user, string action, uint256 timestamp);
    event ValueChanged(address indexed user, uint256 indexed oldValue, uint256 indexed newValue);
    event DataLogged(bytes32 indexed dataHash, bytes data);
    event MultipleValues(
        address indexed user,
        uint256 indexed category,
        string name,
        uint256 value,
        bool success
    );

    // Custom event types
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event EmergencyStop(address indexed triggeredBy, string reason);
    event BatchProcessed(address indexed processor, uint256 batchSize, uint256 successCount);

    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    /// @notice Constructor sets owner
    constructor() {
        owner = msg.sender;
        emit OwnershipTransferred(address(0), msg.sender);
    }

    /// @notice Emit a simple event with a message
    /// @param message The message to emit
    function emitSimpleEvent(string memory message) public {
        emit SimpleEvent(message);
        _incrementEventCount("SimpleEvent");
    }

    /// @notice Emit a user action event
    /// @param action The action performed
    function emitUserAction(string memory action) public {
        emit UserAction(msg.sender, action, block.timestamp);
        _incrementEventCount("UserAction");
    }

    /// @notice Emit a value change event
    /// @param oldValue The old value
    /// @param newValue The new value
    function emitValueChange(uint256 oldValue, uint256 newValue) public {
        emit ValueChanged(msg.sender, oldValue, newValue);
        _incrementEventCount("ValueChanged");
    }

    /// @notice Emit a data logging event
    /// @param data The data to log
    function emitDataLog(bytes memory data) public {
        bytes32 dataHash = keccak256(data);
        emit DataLogged(dataHash, data);
        _incrementEventCount("DataLogged");
    }

    /// @notice Emit a complex event with multiple values
    /// @param category The category of the event
    /// @param name The name associated with the event
    /// @param value The value to log
    /// @param success Whether the operation was successful
    function emitMultipleValues(
        uint256 category,
        string memory name,
        uint256 value,
        bool success
    ) public {
        emit MultipleValues(msg.sender, category, name, value, success);
        _incrementEventCount("MultipleValues");
    }

    /// @notice Emit an emergency stop event (owner only)
    /// @param reason The reason for the emergency stop
    function emitEmergencyStop(string memory reason) public onlyOwner {
        emit EmergencyStop(msg.sender, reason);
        _incrementEventCount("EmergencyStop");
    }

    /// @notice Emit a batch processing event
    /// @param batchSize The size of the batch
    /// @param successCount The number of successful operations
    function emitBatchProcessed(uint256 batchSize, uint256 successCount) public {
        emit BatchProcessed(msg.sender, batchSize, successCount);
        _incrementEventCount("BatchProcessed");
    }

    /// @notice Transfer ownership and emit event
    /// @param newOwner The new owner address
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
        _incrementEventCount("OwnershipTransferred");
    }

    /// @notice Batch emit multiple events
    /// @param messages Array of messages to emit
    function batchEmitEvents(string[] memory messages) public {
        for (uint256 i = 0; i < messages.length; i++) {
            emit SimpleEvent(messages[i]);
            _incrementEventCount("SimpleEvent");
        }
        
        // Emit summary event
        emit BatchProcessed(msg.sender, messages.length, messages.length);
        _incrementEventCount("BatchProcessed");
    }

    /// @notice Get the count of events by type
    /// @param eventType The type of event (as bytes32)
    /// @return The count of events of that type
    function getEventTypeCount(bytes32 eventType) public view returns (uint256) {
        return eventTypeCount[eventType];
    }

    /// @notice Get the count of events by type (string version)
    /// @param eventType The type of event (as string)
    /// @return The count of events of that type
    function getEventTypeCount(string memory eventType) public view returns (uint256) {
        return eventTypeCount[keccak256(abi.encodePacked(eventType))];
    }

    /// @notice Get the total event count
    /// @return The total number of events emitted
    function getTotalEventCount() public view returns (uint256) {
        return eventCount;
    }

    /// @notice Internal function to increment event count
    /// @param eventType The type of event
    function _incrementEventCount(string memory eventType) internal {
        eventCount++;
        bytes32 eventTypeHash = keccak256(abi.encodePacked(eventType));
        eventTypeCount[eventTypeHash]++;
    }

    /// @notice Demonstrate event with dynamic data
    /// @param keys Array of keys
    /// @param values Array of values
    function emitDynamicEvent(string[] memory keys, uint256[] memory values) public {
        require(keys.length == values.length, "Keys and values length mismatch");
        
        for (uint256 i = 0; i < keys.length; i++) {
            emit MultipleValues(msg.sender, i, keys[i], values[i], true);
            _incrementEventCount("MultipleValues");
        }
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
mod event_emitter {
    use ink::{
        prelude::{
            string::String,
            vec::Vec,
        },
        storage::Mapping,
    };

    /// The storage struct that holds our contract's state
    #[ink(storage)]
    pub struct EventEmitter {
        /// Contract owner
        owner: AccountId,
        /// Total number of events emitted
        event_count: u64,
        /// Count of events by type (hash of event type name)
        event_type_count: Mapping<[u8; 32], u64>,
        /// Current boolean value for simple toggle operations
        value: bool,
    }

    /// Events that our contract can emit

    /// Simple event with just a message
    #[ink(event)]
    pub struct SimpleEvent {
        message: String,
    }

    /// User action event with indexed user
    #[ink(event)]
    pub struct UserAction {
        #[ink(topic)]
        user: AccountId,
        action: String,
        timestamp: u64,
    }

    /// Value change event with indexed values
    #[ink(event)]
    pub struct ValueChanged {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        old_value: u128,
        #[ink(topic)]
        new_value: u128,
    }

    /// Data logging event
    #[ink(event)]
    pub struct DataLogged {
        #[ink(topic)]
        data_hash: [u8; 32],
        data: Vec<u8>,
    }

    /// Complex event with multiple values
    #[ink(event)]
    pub struct MultipleValues {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        category: u32,
        name: String,
        value: u128,
        success: bool,
    }

    /// Ownership transfer event
    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        #[ink(topic)]
        new_owner: AccountId,
    }

    /// Emergency stop event
    #[ink(event)]
    pub struct EmergencyStop {
        #[ink(topic)]
        triggered_by: AccountId,
        reason: String,
    }

    /// Batch processing event
    #[ink(event)]
    pub struct BatchProcessed {
        #[ink(topic)]
        processor: AccountId,
        batch_size: u32,
        success_count: u32,
    }

    /// Anonymous event (no signature topic)
    #[ink(event)]
    #[ink(anonymous)]
    pub struct AnonymousEvent {
        #[ink(topic)]
        topic: [u8; 32],
        field_1: u32,
    }

    /// Custom signature event
    #[ink(
        event,
        signature_topic = "1111111111111111111111111111111111111111111111111111111111111111"
    )]
    pub struct CustomSignatureEvent {
        value: bool,
    }

    /// Error types that our contract can return
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        ArrayLengthMismatch,
        ZeroAddress,
    }

    /// Type alias for our Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl EventEmitter {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                owner: caller,
                event_count: 0,
                event_type_count: Mapping::default(),
                value: false,
            };

            // Emit initial ownership event
            instance.env().emit_event(OwnershipTransferred {
                previous_owner: None,
                new_owner: caller,
            });
            instance.increment_event_count("OwnershipTransferred");

            instance
        }

        /// Emit a simple event with a message
        #[ink(message)]
        pub fn emit_simple_event(&mut self, message: String) {
            self.env().emit_event(SimpleEvent { message });
            self.increment_event_count("SimpleEvent");
        }

        /// Emit a user action event
        #[ink(message)]
        pub fn emit_user_action(&mut self, action: String) {
            let caller = self.env().caller();
            let timestamp = self.env().block_timestamp();

            self.env().emit_event(UserAction {
                user: caller,
                action,
                timestamp,
            });
            self.increment_event_count("UserAction");
        }

        /// Emit a value change event
        #[ink(message)]
        pub fn emit_value_change(&mut self, old_value: u128, new_value: u128) {
            let caller = self.env().caller();

            self.env().emit_event(ValueChanged {
                user: caller,
                old_value,
                new_value,
            });
            self.increment_event_count("ValueChanged");
        }

        /// Emit a data logging event
        #[ink(message)]
        pub fn emit_data_log(&mut self, data: Vec<u8>) {
            use ink::env::hash::{Blake2x256, HashOutput};
            
            let mut hash_output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut hash_output);

            self.env().emit_event(DataLogged {
                data_hash: hash_output,
                data,
            });
            self.increment_event_count("DataLogged");
        }

        /// Emit a complex event with multiple values
        #[ink(message)]
        pub fn emit_multiple_values(
            &mut self,
            category: u32,
            name: String,
            value: u128,
            success: bool,
        ) {
            let caller = self.env().caller();

            self.env().emit_event(MultipleValues {
                user: caller,
                category,
                name,
                value,
                success,
            });
            self.increment_event_count("MultipleValues");
        }

        /// Emit an emergency stop event (owner only)
        #[ink(message)]
        pub fn emit_emergency_stop(&mut self, reason: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.env().emit_event(EmergencyStop {
                triggered_by: caller,
                reason,
            });
            self.increment_event_count("EmergencyStop");

            Ok(())
        }

        /// Emit a batch processing event
        #[ink(message)]
        pub fn emit_batch_processed(&mut self, batch_size: u32, success_count: u32) {
            let caller = self.env().caller();

            self.env().emit_event(BatchProcessed {
                processor: caller,
                batch_size,
                success_count,
            });
            self.increment_event_count("BatchProcessed");
        }

        /// Transfer ownership and emit event
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let previous_owner = self.owner;
            self.owner = new_owner;

            self.env().emit_event(OwnershipTransferred {
                previous_owner: Some(previous_owner),
                new_owner,
            });
            self.increment_event_count("OwnershipTransferred");

            Ok(())
        }

        /// Batch emit multiple events
        #[ink(message)]
        pub fn batch_emit_events(&mut self, messages: Vec<String>) {
            let message_count = messages.len();

            for message in messages {
                self.env().emit_event(SimpleEvent { message });
                self.increment_event_count("SimpleEvent");
            }

            // Emit summary event
            self.env().emit_event(BatchProcessed {
                processor: self.env().caller(),
                batch_size: message_count as u32,
                success_count: message_count as u32,
            });
            self.increment_event_count("BatchProcessed");
        }

        /// Emit dynamic events with key-value pairs
        #[ink(message)]
        pub fn emit_dynamic_event(&mut self, keys: Vec<String>, values: Vec<u128>) -> Result<()> {
            if keys.len() != values.len() {
                return Err(Error::ArrayLengthMismatch);
            }

            let caller = self.env().caller();

            for (i, (key, value)) in keys.into_iter().zip(values.into_iter()).enumerate() {
                self.env().emit_event(MultipleValues {
                    user: caller,
                    category: i as u32,
                    name: key,
                    value,
                    success: true,
                });
                self.increment_event_count("MultipleValues");
            }

            Ok(())
        }

        /// Emit an anonymous event
        #[ink(message)]
        pub fn emit_anonymous_event(&mut self, topic: [u8; 32], field_1: u32) {
            self.env().emit_event(AnonymousEvent { topic, field_1 });
            self.increment_event_count("AnonymousEvent");
        }

        /// Emit custom signature event
        #[ink(message)]
        pub fn emit_custom_signature_event(&mut self) {
            self.value = !self.value;
            self.env().emit_event(CustomSignatureEvent { value: self.value });
            self.increment_event_count("CustomSignatureEvent");
        }

        /// Flip the internal value and emit event
        #[ink(message)]
        pub fn flip_with_event(&mut self) {
            self.value = !self.value;
            self.env().emit_event(SimpleEvent {
                message: format!("Value flipped to: {}", self.value),
            });
            self.increment_event_count("SimpleEvent");
        }

        /// Get the count of events by type (using string)
        #[ink(message)]
        pub fn get_event_type_count(&self, event_type: String) -> u64 {
            use ink::env::hash::{Blake2x256, HashOutput};
            
            let mut hash_output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(event_type.as_bytes(), &mut hash_output);
            
            self.event_type_count.get(&hash_output).unwrap_or(0)
        }

        /// Get the total event count
        #[ink(message)]
        pub fn get_total_event_count(&self) -> u64 {
            self.event_count
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

        /// Get the current value
        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }

        /// Internal helper to increment event count
        fn increment_event_count(&mut self, event_type: &str) {
            use ink::env::hash::{Blake2x256, HashOutput};
            
            self.event_count += 1;
            
            let mut hash_output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(event_type.as_bytes(), &mut hash_output);
            
            let current_count = self.event_type_count.get(&hash_output).unwrap_or(0);
            self.event_type_count.insert(&hash_output, &(current_count + 1));
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = EventEmitter::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.get_owner(), accounts.alice);
            assert_eq!(contract.get_total_event_count(), 1); // Constructor emits OwnershipTransferred
        }

        #[ink::test]
        fn emit_simple_event_works() {
            let mut contract = EventEmitter::new();
            
            contract.emit_simple_event("Hello World".to_string());
            assert_eq!(contract.get_total_event_count(), 2); // Constructor + SimpleEvent
            assert_eq!(contract.get_event_type_count("SimpleEvent".to_string()), 1);
        }

        #[ink::test]
        fn emit_user_action_works() {
            let mut contract = EventEmitter::new();
            
            contract.emit_user_action("Login".to_string());
            assert_eq!(contract.get_event_type_count("UserAction".to_string()), 1);
        }

        #[ink::test]
        fn emit_value_change_works() {
            let mut contract = EventEmitter::new();
            
            contract.emit_value_change(100, 200);
            assert_eq!(contract.get_event_type_count("ValueChanged".to_string()), 1);
        }

        #[ink::test]
        fn emit_multiple_values_works() {
            let mut contract = EventEmitter::new();
            
            contract.emit_multiple_values(
                1,
                "test".to_string(),
                42,
                true,
            );
            assert_eq!(contract.get_event_type_count("MultipleValues".to_string()), 1);
        }

        #[ink::test]
        fn batch_emit_works() {
            let mut contract = EventEmitter::new();
            
            let messages = vec![
                "Message 1".to_string(),
                "Message 2".to_string(),
                "Message 3".to_string(),
            ];
            
            contract.batch_emit_events(messages);
            assert_eq!(contract.get_event_type_count("SimpleEvent".to_string()), 3);
            assert_eq!(contract.get_event_type_count("BatchProcessed".to_string()), 1);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = EventEmitter::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert_eq!(contract.transfer_ownership(accounts.bob), Ok(()));
            assert_eq!(contract.get_owner(), accounts.bob);
            assert_eq!(contract.get_event_type_count("OwnershipTransferred".to_string()), 2);
        }

        #[ink::test]
        fn transfer_ownership_fails_for_non_owner() {
            let mut contract = EventEmitter::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Change caller to Bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            assert_eq!(contract.transfer_ownership(accounts.charlie), Err(Error::NotOwner));
        }

        #[ink::test]
        fn emit_emergency_stop_works() {
            let mut contract = EventEmitter::new();
            
            assert_eq!(contract.emit_emergency_stop("System malfunction".to_string()), Ok(()));
            assert_eq!(contract.get_event_type_count("EmergencyStop".to_string()), 1);
        }

        #[ink::test]
        fn emit_dynamic_event_works() {
            let mut contract = EventEmitter::new();
            
            let keys = vec!["key1".to_string(), "key2".to_string()];
            let values = vec![100, 200];
            
            assert_eq!(contract.emit_dynamic_event(keys, values), Ok(()));
            assert_eq!(contract.get_event_type_count("MultipleValues".to_string()), 2);
        }

        #[ink::test]
        fn emit_dynamic_event_fails_with_mismatched_arrays() {
            let mut contract = EventEmitter::new();
            
            let keys = vec!["key1".to_string()];
            let values = vec![100, 200]; // Different length
            
            assert_eq!(contract.emit_dynamic_event(keys, values), Err(Error::ArrayLengthMismatch));
        }

        #[ink::test]
        fn flip_with_event_works() {
            let mut contract = EventEmitter::new();
            
            assert_eq!(contract.get_value(), false);
            contract.flip_with_event();
            assert_eq!(contract.get_value(), true);
        }
    }
}
```

## Key Migration Points

### 1. Event Definition Syntax
**Solidity:**
```solidity
event UserAction(address indexed user, string action, uint256 timestamp);
emit UserAction(msg.sender, "login", block.timestamp);
```

**ink!:**
```rust
#[ink(event)]
pub struct UserAction {
    #[ink(topic)]
    user: AccountId,
    action: String,
    timestamp: u64,
}

self.env().emit_event(UserAction {
    user: self.env().caller(),
    action: "login".to_string(),
    timestamp: self.env().block_timestamp(),
});
```

### 2. Event Indexing
**Solidity:**
```solidity
event ValueChanged(address indexed user, uint256 indexed oldValue, uint256 indexed newValue);
```

**ink!:**
```rust
#[ink(event)]
pub struct ValueChanged {
    #[ink(topic)]  // Indexed field
    user: AccountId,
    #[ink(topic)]  // Indexed field  
    old_value: u128,
    #[ink(topic)]  // Indexed field
    new_value: u128,
}
```

### 3. Anonymous Events
**Solidity:**
```solidity
event Transfer(address indexed from, address indexed to, uint256 value) anonymous;
```

**ink!:**
```rust
#[ink(event)]
#[ink(anonymous)]
pub struct AnonymousEvent {
    #[ink(topic)]
    topic: [u8; 32],
    field_1: u32,
}
```

### 4. Custom Event Signatures
**Solidity:**
```solidity
// Solidity doesn't have direct custom signature support
```

**ink!:**
```rust
#[ink(
    event,
    signature_topic = "1111111111111111111111111111111111111111111111111111111111111111"
)]
pub struct CustomSignatureEvent {
    value: bool,
}
```

### 5. Event Emission
**Solidity:**
```solidity
emit SimpleEvent("Hello World");
```

**ink!:**
```rust
self.env().emit_event(SimpleEvent {
    message: "Hello World".to_string(),
});
```

## Migration Steps

### Step 1: Convert Event Definitions
1. Replace `event` declarations with `#[ink(event)]` structs
2. Use `#[ink(topic)]` for indexed fields
3. Convert Solidity types to Rust types

### Step 2: Update Event Emission
1. Replace `emit EventName(...)` with `self.env().emit_event(EventName { ... })`
2. Use struct initialization syntax
3. Handle type conversions (e.g., `string` to `String`)

### Step 3: Handle Special Event Types
1. Add `#[ink(anonymous)]` for anonymous events
2. Use custom signature topics where needed
3. Consider event filtering requirements

### Step 4: Event Tracking and Analytics
1. Implement event counting with storage mappings
2. Use hashing for event type identification
3. Add getter functions for analytics

### Step 5: Testing Event Emission
1. Use `ink::env::test::recorded_events()` for testing
2. Verify event data and topics
3. Test event filtering and indexing

## Common Patterns

### Event Counting and Analytics
```rust
fn increment_event_count(&mut self, event_type: &str) {
    use ink::env::hash::{Blake2x256, HashOutput};
    
    self.event_count += 1;
    
    let mut hash_output = <Blake2x256 as HashOutput>::Type::default();
    ink::env::hash_bytes::<Blake2x256>(event_type.as_bytes(), &mut hash_output);
    
    let current_count = self.event_type_count.get(&hash_output).unwrap_or(0);
    self.event_type_count.insert(&hash_output, &(current_count + 1));
}
```

### Batch Event Emission
```rust
#[ink(message)]
pub fn batch_emit_events(&mut self, messages: Vec<String>) {
    for message in messages.iter() {
        self.env().emit_event(SimpleEvent { message: message.clone() });
    }
    
    self.env().emit_event(BatchProcessed {
        processor: self.env().caller(),
        batch_size: messages.len() as u32,
        success_count: messages.len() as u32,
    });
}
```

### Event Data Hashing
```rust
fn emit_data_log(&mut self, data: Vec<u8>) {
    use ink::env::hash::{Blake2x256, HashOutput};
    
    let mut hash_output = <Blake2x256 as HashOutput>::Type::default();
    ink::env::hash_bytes::<Blake2x256>(&data, &mut hash_output);

    self.env().emit_event(DataLogged {
        data_hash: hash_output,
        data,
    });
}
```

## Best Practices

### 1. Event Structure Design
- Use appropriate data types for efficiency
- Consider indexing strategy for searchability
- Group related fields in single events

### 2. Topic Usage
- Use `#[ink(topic)]` sparingly (max 3-4 topics per event)
- Index fields that will be filtered on
- Balance searchability with gas costs

### 3. Event Testing
- Always test event emission in unit tests
- Verify event data integrity
- Test event filtering and indexing

### 4. Performance Considerations
- Avoid large data in indexed fields
- Use appropriate data types for event fields
- Consider event frequency and storage costs

### 5. Analytics and Monitoring
- Implement event counting for analytics
- Use consistent event naming conventions
- Consider event versioning for upgrades

This migration demonstrates how Solidity's event system translates to ink!'s more structured and type-safe approach, with better compile-time guarantees and more flexible event configuration options.