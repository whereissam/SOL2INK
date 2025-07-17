use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractPair {
    pub solidity_path: String,
    pub ink_path: String,
    pub contract_type: String,
    pub description: String,
    pub solidity_content: String,
    pub ink_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMatchResult {
    pub pairs: Vec<ContractPair>,
    pub unmatched_solidity: Vec<String>,
    pub unmatched_ink: Vec<String>,
}

pub struct ContractMatcher {
    pub solidity_base_path: String,
    pub ink_base_path: String,
}

impl ContractMatcher {
    pub fn new(solidity_base_path: String, ink_base_path: String) -> Self {
        Self {
            solidity_base_path,
            ink_base_path,
        }
    }

    pub fn find_contract_pairs(&self) -> Result<ContractMatchResult, String> {
        let mut pairs = Vec::new();
        let mut unmatched_solidity = Vec::new();
        let mut unmatched_ink = Vec::new();

        // Define known contract mappings
        let contract_mappings = self.get_contract_mappings();

        // Find Solidity contracts
        let solidity_contracts = self.find_solidity_contracts()?;
        
        for solidity_contract in solidity_contracts {
            let contract_name = self.extract_contract_name(&solidity_contract);
            
            if let Some(ink_path) = contract_mappings.get(&contract_name) {
                // Check if ink contract exists
                let full_ink_path = format!("{}/{}", self.ink_base_path, ink_path);
                if Path::new(&full_ink_path).exists() {
                    // Read both contract contents
                    let solidity_content = fs::read_to_string(&solidity_contract)
                        .map_err(|e| format!("Failed to read Solidity contract: {}", e))?;
                    let ink_content = fs::read_to_string(&full_ink_path)
                        .map_err(|e| format!("Failed to read ink! contract: {}", e))?;

                    pairs.push(ContractPair {
                        solidity_path: solidity_contract.clone(),
                        ink_path: full_ink_path,
                        contract_type: contract_name.clone(),
                        description: self.get_contract_description(&contract_name),
                        solidity_content,
                        ink_content,
                    });
                } else {
                    unmatched_solidity.push(solidity_contract);
                }
            } else {
                unmatched_solidity.push(solidity_contract);
            }
        }

        // Find unmatched ink contracts
        for (contract_name, ink_path) in contract_mappings {
            let full_ink_path = format!("{}/{}", self.ink_base_path, ink_path);
            if Path::new(&full_ink_path).exists() {
                // Check if this ink contract was already matched
                if !pairs.iter().any(|p| p.contract_type == contract_name) {
                    unmatched_ink.push(full_ink_path);
                }
            }
        }

        Ok(ContractMatchResult {
            pairs,
            unmatched_solidity,
            unmatched_ink,
        })
    }

    fn find_solidity_contracts(&self) -> Result<Vec<String>, String> {
        let mut contracts = Vec::new();
        let src_path = format!("{}/src", self.solidity_base_path);
        
        if let Ok(entries) = fs::read_dir(&src_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("sol") {
                        contracts.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        Ok(contracts)
    }

    fn extract_contract_name(&self, file_path: &str) -> String {
        Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    fn get_contract_mappings(&self) -> HashMap<String, String> {
        let mut mappings = HashMap::new();
        
        // Token standards
        mappings.insert("SimpleERC20".to_string(), "erc20/lib.rs".to_string());
        mappings.insert("SimpleNFT".to_string(), "erc721/lib.rs".to_string());
        mappings.insert("SimpleERC1155".to_string(), "erc1155/lib.rs".to_string());
        
        // Basic contracts
        mappings.insert("Flipper".to_string(), "flipper/lib.rs".to_string());
        mappings.insert("Counter".to_string(), "incrementer/lib.rs".to_string());
        mappings.insert("SimpleStorage".to_string(), "contract-storage/lib.rs".to_string());
        mappings.insert("MultiSigWallet".to_string(), "multisig/lib.rs".to_string());
        mappings.insert("SimpleEscrow".to_string(), "payment-channel/lib.rs".to_string());
        mappings.insert("EventEmitter".to_string(), "events/lib.rs".to_string());
        
        // Cross-contract calls
        mappings.insert("CallerContract".to_string(), "basic-contract-caller/lib.rs".to_string());
        mappings.insert("TargetContract".to_string(), "basic-contract-caller/other-contract/lib.rs".to_string());
        
        mappings
    }

    fn get_contract_description(&self, contract_name: &str) -> String {
        match contract_name {
            "SimpleERC20" => "ERC20 fungible token implementation with basic transfer, approve, and allowance functionality".to_string(),
            "SimpleNFT" => "ERC721 non-fungible token implementation with minting, burning, and transfer capabilities".to_string(),
            "SimpleERC1155" => "Multi-token standard supporting both fungible and non-fungible tokens with batch operations".to_string(),
            "Flipper" => "Simple boolean state contract that can be flipped between true and false".to_string(),
            "Counter" => "Basic counter contract with increment and decrement functionality".to_string(),
            "SimpleStorage" => "Basic storage contract demonstrating state management and data persistence".to_string(),
            "MultiSigWallet" => "Multi-signature wallet requiring multiple approvals for transactions".to_string(),
            "SimpleEscrow" => "Escrow contract for holding funds until conditions are met".to_string(),
            "EventEmitter" => "Contract demonstrating event emission and indexing patterns".to_string(),
            "CallerContract" => "Contract that calls other contracts, demonstrating cross-contract interactions".to_string(),
            "TargetContract" => "Target contract for cross-contract calls and interactions".to_string(),
            _ => format!("Smart contract implementation: {}", contract_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_matcher_creation() {
        let matcher = ContractMatcher::new(
            "/path/to/solidity".to_string(),
            "/path/to/ink".to_string(),
        );
        
        assert_eq!(matcher.solidity_base_path, "/path/to/solidity");
        assert_eq!(matcher.ink_base_path, "/path/to/ink");
    }

    #[test]
    fn test_extract_contract_name() {
        let matcher = ContractMatcher::new("".to_string(), "".to_string());
        
        assert_eq!(matcher.extract_contract_name("/path/to/SimpleERC20.sol"), "SimpleERC20");
        assert_eq!(matcher.extract_contract_name("Flipper.sol"), "Flipper");
    }

    #[test]
    fn test_get_contract_mappings() {
        let matcher = ContractMatcher::new("".to_string(), "".to_string());
        let mappings = matcher.get_contract_mappings();
        
        assert!(mappings.contains_key("SimpleERC20"));
        assert_eq!(mappings.get("SimpleERC20"), Some(&"erc20/lib.rs".to_string()));
        assert!(mappings.contains_key("Flipper"));
        assert_eq!(mappings.get("Flipper"), Some(&"flipper/lib.rs".to_string()));
    }

    #[test]
    fn test_get_contract_description() {
        let matcher = ContractMatcher::new("".to_string(), "".to_string());
        
        let description = matcher.get_contract_description("SimpleERC20");
        assert!(description.contains("ERC20"));
        assert!(description.contains("fungible token"));
        
        let unknown_description = matcher.get_contract_description("UnknownContract");
        assert!(unknown_description.contains("UnknownContract"));
    }
}