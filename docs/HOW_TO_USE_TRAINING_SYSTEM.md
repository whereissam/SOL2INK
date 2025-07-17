# How to Use the Solidity to ink! Training System

## 🎯 Complete Step-by-Step Guide

This guide walks you through the entire process of creating, training, and testing the AI system that helps developers migrate from Solidity to ink!.

---

## 📋 Table of Contents

1. [System Overview](#system-overview)
2. [Prerequisites](#prerequisites)
3. [Step 1: Writing Solidity Contracts](#step-1-writing-solidity-contracts)
4. [Step 2: Finding Matching ink! Contracts](#step-2-finding-matching-ink-contracts)
5. [Step 3: Training the AI System](#step-3-training-the-ai-system)
6. [Step 4: Testing the Training](#step-4-testing-the-training)
7. [Step 5: Querying the AI](#step-5-querying-the-ai)
8. [Advanced Usage](#advanced-usage)
9. [Troubleshooting](#troubleshooting)

---

## System Overview

The training system automatically:
1. **Matches** Solidity contracts with equivalent ink! implementations
2. **Generates** comprehensive migration guides
3. **Embeds** training data into vector database
4. **Provides** AI-powered migration assistance

### 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Solidity        │    │ ink!            │    │ AI Training     │
│ Contracts       │────│ Contracts       │────│ System          │
│ (.sol files)    │    │ (.rs files)     │    │ (Vector DB)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
                                             ┌─────────────────┐
                                             │ Query Interface │
                                             │ (API + Web)     │
                                             └─────────────────┘
```

---

## Prerequisites

### 🔧 Development Environment

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install Foundry (for Solidity)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# 3. Install ink! CLI
cargo install --force --locked cargo-contract

# 4. Install Python (for testing)
python3 -m pip install requests
```

### 📁 Project Structure

```
project/
├── solidity-examples/          # Solidity contracts
│   ├── src/
│   │   ├── SimpleERC20.sol
│   │   ├── Flipper.sol
│   │   └── ...
│   └── foundry.toml
├── ink-examples-main/          # ink! contracts
│   ├── erc20/lib.rs
│   ├── flipper/lib.rs
│   └── ...
├── shuttle-backend/            # Training system
│   ├── src/
│   │   ├── contract_matcher.rs
│   │   ├── training_embedder.rs
│   │   └── main.rs
│   └── Cargo.toml
└── test_training_system.py     # Test script
```

---

## Step 1: Writing Solidity Contracts

### 🔵 Creating a New Solidity Contract

Let's create a simple voting contract as an example:

```bash
cd solidity-examples/src
```

Create `SimpleVoting.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleVoting
/// @notice A simple voting contract for demonstration
contract SimpleVoting {
    struct Proposal {
        string description;
        uint256 voteCount;
        bool executed;
    }
    
    mapping(uint256 => Proposal) public proposals;
    mapping(address => mapping(uint256 => bool)) public hasVoted;
    mapping(address => bool) public isVoter;
    
    address public owner;
    uint256 public proposalCount;
    
    event ProposalCreated(uint256 indexed proposalId, string description);
    event VoteCast(uint256 indexed proposalId, address indexed voter);
    event ProposalExecuted(uint256 indexed proposalId);
    
    error NotOwner();
    error NotVoter();
    error AlreadyVoted();
    error ProposalNotFound();
    error ProposalAlreadyExecuted();
    
    constructor() {
        owner = msg.sender;
        isVoter[msg.sender] = true;
        proposalCount = 0;
    }
    
    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }
    
    modifier onlyVoter() {
        if (!isVoter[msg.sender]) revert NotVoter();
        _;
    }
    
    function addVoter(address voter) external onlyOwner {
        isVoter[voter] = true;
    }
    
    function createProposal(string memory description) external onlyOwner returns (uint256) {
        uint256 proposalId = proposalCount++;
        proposals[proposalId] = Proposal({
            description: description,
            voteCount: 0,
            executed: false
        });
        
        emit ProposalCreated(proposalId, description);
        return proposalId;
    }
    
    function vote(uint256 proposalId) external onlyVoter {
        if (proposalId >= proposalCount) revert ProposalNotFound();
        if (hasVoted[msg.sender][proposalId]) revert AlreadyVoted();
        
        hasVoted[msg.sender][proposalId] = true;
        proposals[proposalId].voteCount++;
        
        emit VoteCast(proposalId, msg.sender);
    }
    
    function executeProposal(uint256 proposalId) external onlyOwner {
        if (proposalId >= proposalCount) revert ProposalNotFound();
        if (proposals[proposalId].executed) revert ProposalAlreadyExecuted();
        
        proposals[proposalId].executed = true;
        emit ProposalExecuted(proposalId);
    }
    
    function getProposal(uint256 proposalId) external view returns (Proposal memory) {
        if (proposalId >= proposalCount) revert ProposalNotFound();
        return proposals[proposalId];
    }
}
```

### 🔨 Compile the Contract

```bash
cd solidity-examples
forge build
```

Expected output:
```
Compiling 1 files with Solc 0.8.30
Solc 0.8.30 finished in 123.45ms
Compiler run successful!
```

---

## Step 2: Finding Matching ink! Contracts

### 🟠 Create the ink! Equivalent

Create the matching ink! contract in `ink-examples-main/voting/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod voting {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Proposal {
        pub description: String,
        pub vote_count: u32,
        pub executed: bool,
    }

    #[ink(storage)]
    pub struct Voting {
        proposals: Mapping<u32, Proposal>,
        has_voted: Mapping<(AccountId, u32), bool>,
        is_voter: Mapping<AccountId, bool>,
        owner: AccountId,
        proposal_count: u32,
    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u32,
        description: String,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        proposal_id: u32,
        #[ink(topic)]
        voter: AccountId,
    }

    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        NotVoter,
        AlreadyVoted,
        ProposalNotFound,
        ProposalAlreadyExecuted,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Voting {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut is_voter = Mapping::new();
            is_voter.insert(caller, &true);
            
            Self {
                proposals: Mapping::new(),
                has_voted: Mapping::new(),
                is_voter,
                owner: caller,
                proposal_count: 0,
            }
        }

        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            
            self.is_voter.insert(voter, &true);
            Ok(())
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, description: String) -> Result<u32> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            
            let proposal_id = self.proposal_count;
            self.proposal_count += 1;
            
            let proposal = Proposal {
                description: description.clone(),
                vote_count: 0,
                executed: false,
            };
            
            self.proposals.insert(proposal_id, &proposal);
            
            self.env().emit_event(ProposalCreated {
                proposal_id,
                description,
            });
            
            Ok(proposal_id)
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<()> {
            let caller = self.env().caller();
            
            if !self.is_voter.get(caller).unwrap_or(false) {
                return Err(Error::NotVoter);
            }
            
            if proposal_id >= self.proposal_count {
                return Err(Error::ProposalNotFound);
            }
            
            if self.has_voted.get((caller, proposal_id)).unwrap_or(false) {
                return Err(Error::AlreadyVoted);
            }
            
            self.has_voted.insert((caller, proposal_id), &true);
            
            if let Some(mut proposal) = self.proposals.get(proposal_id) {
                proposal.vote_count += 1;
                self.proposals.insert(proposal_id, &proposal);
            }
            
            self.env().emit_event(VoteCast {
                proposal_id,
                voter: caller,
            });
            
            Ok(())
        }

        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            
            if proposal_id >= self.proposal_count {
                return Err(Error::ProposalNotFound);
            }
            
            if let Some(mut proposal) = self.proposals.get(proposal_id) {
                if proposal.executed {
                    return Err(Error::ProposalAlreadyExecuted);
                }
                
                proposal.executed = true;
                self.proposals.insert(proposal_id, &proposal);
                
                self.env().emit_event(ProposalExecuted { proposal_id });
            }
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Result<Proposal> {
            if proposal_id >= self.proposal_count {
                return Err(Error::ProposalNotFound);
            }
            
            self.proposals.get(proposal_id).ok_or(Error::ProposalNotFound)
        }

        #[ink(message)]
        pub fn get_proposal_count(&self) -> u32 {
            self.proposal_count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = Voting::new();
            assert_eq!(contract.get_proposal_count(), 0);
        }

        #[ink::test]
        fn create_proposal_works() {
            let mut contract = Voting::new();
            let result = contract.create_proposal("Test proposal".to_string());
            assert_eq!(result, Ok(0));
            assert_eq!(contract.get_proposal_count(), 1);
        }

        #[ink::test]
        fn vote_works() {
            let mut contract = Voting::new();
            let _ = contract.create_proposal("Test proposal".to_string());
            let result = contract.vote(0);
            assert_eq!(result, Ok(()));
        }
    }
}
```

### 🔧 Update Contract Matcher

Add the new contract pair to `shuttle-backend/src/contract_matcher.rs`:

```rust
fn get_contract_mappings(&self) -> HashMap<String, String> {
    let mut mappings = HashMap::new();
    
    // Existing mappings...
    mappings.insert("SimpleERC20".to_string(), "erc20/lib.rs".to_string());
    mappings.insert("Flipper".to_string(), "flipper/lib.rs".to_string());
    
    // Add new voting contract mapping
    mappings.insert("SimpleVoting".to_string(), "voting/lib.rs".to_string());
    
    mappings
}

fn get_contract_description(&self, contract_name: &str) -> String {
    match contract_name {
        "SimpleERC20" => "ERC20 fungible token implementation...".to_string(),
        "Flipper" => "Simple boolean state contract...".to_string(),
        
        // Add new description
        "SimpleVoting" => "Simple voting contract with proposal creation and voting functionality".to_string(),
        
        _ => format!("Smart contract implementation: {}", contract_name),
    }
}
```

---

## Step 3: Training the AI System

### 🚀 Start the Backend Server

```bash
cd shuttle-backend
cargo run --bin dynavest-shuttle-backend
```

Expected output:
```
🚀 DynaVest Shuttle Backend is starting...
📊 Available endpoints:
  GET    /health - Health check
  POST   /training/embed-contracts - Embed Solidity+ink! contract pairs for training
  GET    /training/contract-pairs - Get available contract pairs
  GET    /ask?query=... - Ask a question and get RAG response
```

### 🔍 Check Available Contract Pairs

```bash
curl -X GET http://localhost:8000/training/contract-pairs
```

Expected response:
```json
{
  "success": true,
  "data": [
    "SimpleERC20: ERC20 fungible token implementation with basic transfer, approve, and allowance functionality",
    "Flipper: Simple boolean state contract that can be flipped between true and false",
    "SimpleVoting: Simple voting contract with proposal creation and voting functionality",
    "Counter: Basic counter contract with increment and decrement functionality",
    "... more contracts ..."
  ],
  "error": null
}
```

### 🧠 Train the AI System

```bash
curl -X POST http://localhost:8000/training/embed-contracts
```

Expected response:
```json
{
  "success": true,
  "data": {
    "processed_pairs": 12,
    "document_ids": [
      "doc_001", "doc_002", "doc_003", "...",
    ],
    "errors": []
  },
  "error": null
}
```

### 📊 Monitor Training Progress

The system will output logs like:
```
Starting contract pair embedding process...
Found 12 contract pairs
Successfully embedded: SimpleERC20
Successfully embedded: Flipper
Successfully embedded: SimpleVoting
Successfully embedded: Counter
...
Contract embedding completed: 12 pairs processed
```

---

## Step 4: Testing the Training

### 🧪 Use the Python Test Script

```bash
python3 test_training_system.py
```

Expected output:
```
🚀 Testing Solidity to ink! Training System
==================================================
🏥 Testing server health...
✅ Server is running

🔍 Testing contract pairs endpoint...
✅ Found 12 contract pairs:
  - SimpleERC20: ERC20 fungible token implementation...
  - Flipper: Simple boolean state contract...
  - SimpleVoting: Simple voting contract...
  - ...

🧠 Testing contract embedding endpoint...
✅ Embedding successful!
  - Processed pairs: 12
  - Document IDs: 12
  - Errors: 0

💬 Testing ERC20 migration query...
✅ Query successful!
Query: How do I implement ERC20 in ink! if I know Solidity?
Answer: To implement ERC20 in ink!, you need to understand several key differences from Solidity...

💬 Testing Flipper migration query...
✅ Query successful!
Query: Show me how to convert a Solidity Flipper contract to ink!
Answer: Converting a Solidity Flipper to ink! involves these main steps...

🎉 Training system tests completed!
```

### 🔧 Manual Testing

Test individual components:

```bash
# Test health
curl http://localhost:8000/health

# Test contract pairs
curl http://localhost:8000/training/contract-pairs

# Test embedding
curl -X POST http://localhost:8000/training/embed-contracts

# Test query
curl "http://localhost:8000/ask?query=How%20do%20I%20create%20a%20voting%20contract%20in%20ink%21%3F"
```

---

## Step 5: Querying the AI

### 💬 Basic Queries

```bash
# ERC20 migration
curl "http://localhost:8000/ask?query=How%20do%20I%20migrate%20ERC20%20from%20Solidity%20to%20ink%21%3F"

# Flipper contract
curl "http://localhost:8000/ask?query=Show%20me%20Flipper%20contract%20differences%20between%20Solidity%20and%20ink%21"

# Voting contract
curl "http://localhost:8000/ask?query=How%20do%20I%20implement%20voting%20in%20ink%21%3F"

# Storage patterns
curl "http://localhost:8000/ask?query=How%20do%20mappings%20work%20in%20ink%21%20vs%20Solidity%3F"

# Error handling
curl "http://localhost:8000/ask?query=How%20do%20I%20handle%20errors%20in%20ink%21%3F"
```

### 🎯 Advanced Queries

```bash
# Complex migration questions
curl "http://localhost:8000/ask?query=I%20have%20a%20Solidity%20contract%20with%20modifiers%20and%20events.%20How%20do%20I%20convert%20it%20to%20ink%21%3F"

# Performance comparisons
curl "http://localhost:8000/ask?query=What%20are%20the%20gas%20differences%20between%20Solidity%20and%20ink%21%3F"

# Testing strategies
curl "http://localhost:8000/ask?query=How%20do%20I%20test%20ink%21%20contracts%20coming%20from%20Solidity%3F"
```

### 📱 Web Interface Testing

If you have a web interface:

```javascript
// JavaScript example
async function queryMigration() {
    const response = await fetch('/ask', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            query: 'How do I implement ERC20 in ink!?'
        })
    });
    
    const result = await response.json();
    console.log(result.data);
}
```

---

## Advanced Usage

### 🔄 Adding New Contract Pairs

1. **Create Solidity contract** in `solidity-examples/src/`
2. **Create ink! equivalent** in `ink-examples-main/`
3. **Update contract matcher** with new mapping
4. **Rebuild and restart** the backend
5. **Re-train the system** with new contracts

### 🎯 Custom Training Data

Add custom migration notes in `training_embedder.rs`:

```rust
fn generate_migration_notes(&self, contract_name: &str) -> String {
    match contract_name {
        "SimpleVoting" => {
            r#"
## Migration Notes: Solidity Voting to ink! Voting

### Key Differences:
1. **Struct Storage**: Solidity structs vs ink! structs with derive attributes
2. **Nested Mappings**: `mapping(address => mapping(uint256 => bool))` vs `Mapping<(AccountId, u32), bool>`
3. **Access Control**: Solidity modifiers vs explicit checks with Result

### Migration Steps:
1. Convert struct definitions with proper derive attributes
2. Use tuple keys for nested mappings
3. Replace modifiers with explicit permission checks
4. Handle errors with Result types instead of reverting

### Common Patterns:
- Solidity: `modifier onlyOwner() { require(msg.sender == owner); _; }`
- ink!: `if self.env().caller() != self.owner { return Err(Error::NotOwner); }`
"#.to_string()
        }
        // ... other contracts
    }
}
```

### 📊 Analytics and Monitoring

Monitor training effectiveness:

```bash
# Check system stats
curl http://localhost:8000/rag/stats

# Query response quality
curl "http://localhost:8000/ask?query=test%20query" | jq '.data' | wc -w
```

---

## Troubleshooting

### 🚨 Common Issues

#### 1. **Contract Not Found**
```
Error: Contract pair not found for SimpleVoting
```

**Solution:**
- Check contract name matches exactly
- Verify contract exists in both directories
- Update contract matcher mappings

#### 2. **Compilation Errors**
```
Error: Solidity compilation failed
```

**Solution:**
```bash
cd solidity-examples
forge build --force
```

#### 3. **Training Failed**
```
Error: Failed to embed contract pairs
```

**Solution:**
- Check Qdrant connection
- Verify Gemini API key
- Restart the backend server

#### 4. **No AI Response**
```
Error: Empty response from AI
```

**Solution:**
- Check if training was completed
- Verify vector database has data
- Try simpler queries first

### 🔧 Debug Commands

```bash
# Check logs
tail -f shuttle-backend/server.log

# Test contract parsing
cd shuttle-backend
cargo test contract_matching -- --nocapture

# Verify embeddings
curl http://localhost:8000/rag/stats
```

### 📞 Support

1. **Check documentation** in tutorial files
2. **Review test outputs** for patterns
3. **Examine contract examples** for reference
4. **Test with simple queries** first

---

## Summary

### ✅ **What You've Accomplished:**

1. **Created** matching Solidity and ink! contracts
2. **Compiled** both contract types successfully
3. **Trained** the AI system with contract pairs
4. **Tested** the training with comprehensive queries
5. **Deployed** a working migration assistant

### 🎯 **Next Steps:**

1. **Add more contract types** to expand training data
2. **Create custom queries** for specific use cases
3. **Integrate with development workflow**
4. **Share with the community** for feedback

### 🚀 **Production Deployment:**

- **Scale the backend** for multiple users
- **Add authentication** for secure access
- **Monitor usage** and improve responses
- **Expand contract library** continuously

**Your Solidity to ink! training system is now fully operational!** 🎉

Developers can now easily migrate their contracts with AI-powered assistance, complete with side-by-side comparisons, migration guides, and interactive help. The system learns from your contract pairs and provides increasingly better assistance over time.

---

*For more detailed examples and advanced usage, refer to the complete tutorial documentation and side-by-side comparison guides.*