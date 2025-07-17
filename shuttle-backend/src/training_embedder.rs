use crate::contract_matcher::{ContractMatcher, ContractPair, ContractMatchResult};
use crate::rag_system::RAGSystem;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingPair {
    pub solidity_content: String,
    pub ink_content: String,
    pub contract_type: String,
    pub description: String,
    pub migration_notes: String,
    pub combined_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResult {
    pub success: bool,
    pub processed_pairs: usize,
    pub document_ids: Vec<String>,
    pub errors: Vec<String>,
}

pub struct TrainingEmbedder {
    pub contract_matcher: ContractMatcher,
    pub rag_system: std::sync::Arc<RAGSystem>,
}

impl TrainingEmbedder {
    pub fn new(
        solidity_path: String,
        ink_path: String,
        rag_system: std::sync::Arc<RAGSystem>,
    ) -> Self {
        Self {
            contract_matcher: ContractMatcher::new(solidity_path, ink_path),
            rag_system,
        }
    }

    pub async fn embed_contract_pairs(&self) -> Result<EmbeddingResult, String> {
        println!("Starting contract pair embedding process...");
        
        // Find contract pairs
        let match_result = self.contract_matcher.find_contract_pairs()?;
        println!("Found {} contract pairs", match_result.pairs.len());

        let mut document_ids = Vec::new();
        let mut errors = Vec::new();
        let mut processed_pairs = 0;

        for pair in match_result.pairs {
            match self.create_training_pair(&pair).await {
                Ok(training_pair) => {
                    match self.embed_training_pair(training_pair).await {
                        Ok(doc_id) => {
                            document_ids.push(doc_id);
                            processed_pairs += 1;
                            println!("Successfully embedded: {}", pair.contract_type);
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to embed {}: {}", pair.contract_type, e);
                            errors.push(error_msg);
                            println!("Error embedding {}: {}", pair.contract_type, e);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to create training pair for {}: {}", pair.contract_type, e);
                    errors.push(error_msg);
                    println!("Error creating training pair for {}: {}", pair.contract_type, e);
                }
            }
        }

        Ok(EmbeddingResult {
            success: errors.is_empty(),
            processed_pairs,
            document_ids,
            errors,
        })
    }

    async fn create_training_pair(&self, pair: &ContractPair) -> Result<TrainingPair, String> {
        let migration_notes = self.generate_migration_notes(&pair.contract_type);
        let combined_content = self.create_combined_content(pair, &migration_notes);

        Ok(TrainingPair {
            solidity_content: pair.solidity_content.clone(),
            ink_content: pair.ink_content.clone(),
            contract_type: pair.contract_type.clone(),
            description: pair.description.clone(),
            migration_notes,
            combined_content,
        })
    }

    async fn embed_training_pair(&self, training_pair: TrainingPair) -> Result<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("contract_type".to_string(), training_pair.contract_type.clone());
        metadata.insert("source".to_string(), "solidity_ink_training".to_string());
        metadata.insert("type".to_string(), "contract_migration_pair".to_string());
        metadata.insert("description".to_string(), training_pair.description.clone());

        self.rag_system
            .add_document(&training_pair.combined_content, metadata)
            .await
            .map_err(|e| format!("Failed to add document to RAG system: {}", e))
    }

    fn generate_migration_notes(&self, contract_type: &str) -> String {
        match contract_type {
            "SimpleERC20" => {
                r#"
## Migration Notes: Solidity ERC20 to ink! ERC20

### Key Differences:
1. **Storage**: Solidity uses `mapping(address => uint256)` while ink! uses `Mapping<AccountId, Balance>`
2. **Error Handling**: Solidity uses `require()` statements, ink! uses `Result<T, E>` with custom error enums
3. **Events**: Solidity events are automatically indexed, ink! requires explicit `#[ink(topic)]` annotations
4. **Function Modifiers**: Solidity modifiers become explicit checks in ink! functions
5. **Constructor**: Solidity constructor becomes `#[ink(constructor)]` in ink!

### Migration Steps:
1. Replace `mapping` with `Mapping` in storage
2. Convert `require()` statements to `ensure!()` or explicit error handling
3. Add `#[ink(storage)]`, `#[ink(constructor)]`, `#[ink(message)]` annotations
4. Define custom error enum with `#[ink::scale_derive(Encode, Decode, TypeInfo)]`
5. Use `self.env().caller()` instead of `msg.sender`
6. Emit events with `self.env().emit_event()`

### Common Patterns:
- Solidity: `require(condition, "error message");`
- ink!: `ensure!(condition, Error::CustomError);`

- Solidity: `msg.sender`
- ink!: `self.env().caller()`

- Solidity: `emit Transfer(from, to, value);`
- ink!: `self.env().emit_event(Transfer { from, to, value });`
"#.to_string()
            }
            "Flipper" => {
                r#"
## Migration Notes: Solidity Flipper to ink! Flipper

### Key Differences:
1. **Storage**: Both use simple boolean storage, but ink! requires `#[ink(storage)]`
2. **State Access**: Solidity direct access vs ink! `self.value`
3. **Function Annotations**: ink! requires `#[ink(message)]` for public functions

### Migration Steps:
1. Wrap storage in struct with `#[ink(storage)]`
2. Add `#[ink(constructor)]` and `#[ink(message)]` annotations
3. Use `self.value` instead of direct variable access
4. Return values explicitly (ink! functions can return values)

### Pattern Comparison:
- Solidity: `bool public value;`
- ink!: `#[ink(storage)] pub struct Flipper { value: bool }`

- Solidity: `function flip() public { value = !value; }`
- ink!: `#[ink(message)] pub fn flip(&mut self) { self.value = !self.value; }`
"#.to_string()
            }
            "Counter" => {
                r#"
## Migration Notes: Solidity Counter to ink! Incrementer

### Key Differences:
1. **Storage**: Solidity `uint256` becomes ink! `i32` or `u32`
2. **Overflow Protection**: Solidity has built-in overflow protection, ink! uses checked arithmetic
3. **Access Control**: Both can implement similar patterns

### Migration Steps:
1. Define storage struct with `#[ink(storage)]`
2. Use `saturating_add()` or `checked_add()` for safe arithmetic
3. Add proper error handling for overflow/underflow
4. Use `#[ink(constructor)]` for initialization

### Safety Patterns:
- Solidity: `count++` (automatic overflow protection)
- ink!: `self.count = self.count.saturating_add(1)` (explicit safety)
"#.to_string()
            }
            "SimpleNFT" => {
                r#"
## Migration Notes: Solidity ERC721 to ink! ERC721

### Key Differences:
1. **Token ID Type**: Solidity `uint256` vs ink! `u32` or custom type
2. **Storage Maps**: Multiple mappings become `Mapping<K, V>` in ink!
3. **Approval System**: Similar logic but different syntax
4. **Safe Transfer**: ink! has built-in safety checks

### Migration Steps:
1. Define `TokenId` type alias
2. Convert all mappings to ink! `Mapping<K, V>`
3. Implement proper error handling for transfers
4. Add `#[ink(event)]` for Transfer and Approval events
5. Use `ensure!()` for validation checks

### Storage Pattern:
- Solidity: `mapping(uint256 => address) private _owners;`
- ink!: `token_owner: Mapping<TokenId, AccountId>`
"#.to_string()
            }
            _ => format!(
                r#"
## Migration Notes: {} Contract

### General Migration Guidelines:
1. **Storage**: Convert Solidity storage variables to ink! storage struct
2. **Functions**: Add `#[ink(message)]` for public functions, `#[ink(constructor)]` for constructor
3. **Error Handling**: Replace `require()` with `ensure!()` or explicit error handling
4. **Events**: Define events with `#[ink(event)]` and emit with `self.env().emit_event()`
5. **Access Control**: Use `self.env().caller()` instead of `msg.sender`

### Common Patterns:
- Storage: `#[ink(storage)] pub struct ContractName {{ field: Type }}`
- Constructor: `#[ink(constructor)] pub fn new() -> Self`
- Messages: `#[ink(message)] pub fn function_name(&self) -> ReturnType`
- Events: `#[ink(event)] pub struct EventName {{ field: Type }}`
"#, contract_type
            )
        }
    }

    fn create_combined_content(&self, pair: &ContractPair, migration_notes: &str) -> String {
        format!(
            r#"# {contract_type} Implementation: Solidity vs ink!

## Overview
{description}

## Solidity Implementation

```solidity
{solidity_code}
```

## ink! Implementation

```rust
{ink_code}
```

{migration_notes}

## Usage Examples

### Solidity Usage:
```solidity
// Deploy contract
{contract_type} token = new {contract_type}();

// Basic interactions depend on contract type
```

### ink! Usage:
```rust
// In your ink! contract tests
#[ink::test]
fn test_contract() {{
    let contract = {contract_type}::new();
    // Test contract functionality
}}
```

## Key Takeaways

1. **Syntax**: ink! uses Rust syntax with special attributes
2. **Safety**: ink! provides compile-time safety guarantees
3. **Efficiency**: ink! contracts are typically more gas-efficient
4. **Tooling**: ink! integrates with Rust's excellent tooling ecosystem

## Common Questions

**Q: How do I migrate from Solidity to ink!?**
A: Follow the migration steps above, focusing on storage layout, error handling, and function annotations.

**Q: Are there any performance differences?**
A: ink! contracts are generally more gas-efficient due to Rust's zero-cost abstractions and compile-time optimizations.

**Q: Can I use existing Solidity libraries in ink!?**
A: No, you need to use ink!-specific libraries or implement equivalent functionality in Rust.
"#,
            contract_type = pair.contract_type,
            description = pair.description,
            solidity_code = pair.solidity_content,
            ink_code = pair.ink_content,
            migration_notes = migration_notes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_migration_notes() {
        let embedder = TrainingEmbedder::new(
            "test_solidity".to_string(),
            "test_ink".to_string(),
            std::sync::Arc::new(unsafe { std::mem::zeroed() }), // Mock for test
        );

        let notes = embedder.generate_migration_notes("SimpleERC20");
        assert!(notes.contains("Migration Notes"));
        assert!(notes.contains("mapping"));
        assert!(notes.contains("Mapping"));

        let flipper_notes = embedder.generate_migration_notes("Flipper");
        assert!(flipper_notes.contains("Flipper"));
        assert!(flipper_notes.contains("boolean"));
    }

    #[test]
    fn test_create_combined_content() {
        let embedder = TrainingEmbedder::new(
            "test_solidity".to_string(),
            "test_ink".to_string(),
            std::sync::Arc::new(unsafe { std::mem::zeroed() }), // Mock for test
        );

        let pair = ContractPair {
            solidity_path: "test.sol".to_string(),
            ink_path: "test.rs".to_string(),
            contract_type: "TestContract".to_string(),
            description: "Test contract".to_string(),
            solidity_content: "pragma solidity ^0.8.0;\ncontract Test {}".to_string(),
            ink_content: "#[ink::contract]\nmod test {}".to_string(),
        };

        let migration_notes = "Test migration notes";
        let combined = embedder.create_combined_content(&pair, migration_notes);

        assert!(combined.contains("TestContract"));
        assert!(combined.contains("```solidity"));
        assert!(combined.contains("```rust"));
        assert!(combined.contains("Test migration notes"));
    }
}