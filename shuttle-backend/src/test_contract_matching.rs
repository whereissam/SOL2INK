use crate::contract_matcher::ContractMatcher;
use crate::training_embedder::TrainingEmbedder;

#[tokio::test]
async fn test_contract_matching() {
    let solidity_path = "/Users/huangbozhang/Desktop/project/aidoc/solidity-examples".to_string();
    let ink_path = "/Users/huangbozhang/Desktop/project/aidoc/ink-examples-main".to_string();
    
    let matcher = ContractMatcher::new(solidity_path, ink_path);
    
    match matcher.find_contract_pairs() {
        Ok(result) => {
            println!("Found {} contract pairs:", result.pairs.len());
            for pair in &result.pairs {
                println!("  - {}: {} <-> {}", 
                    pair.contract_type, 
                    pair.solidity_path, 
                    pair.ink_path
                );
            }
            
            println!("\nUnmatched Solidity contracts: {}", result.unmatched_solidity.len());
            for unmatched in &result.unmatched_solidity {
                println!("  - {}", unmatched);
            }
            
            println!("\nUnmatched ink! contracts: {}", result.unmatched_ink.len());
            for unmatched in &result.unmatched_ink {
                println!("  - {}", unmatched);
            }
            
            assert!(result.pairs.len() > 0, "Should find at least one matching pair");
        }
        Err(e) => {
            panic!("Contract matching failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_specific_contract_pair() {
    let solidity_path = "/Users/huangbozhang/Desktop/project/aidoc/solidity-examples".to_string();
    let ink_path = "/Users/huangbozhang/Desktop/project/aidoc/ink-examples-main".to_string();
    
    let matcher = ContractMatcher::new(solidity_path, ink_path);
    
    match matcher.find_contract_pairs() {
        Ok(result) => {
            // Look for ERC20 pair specifically
            let erc20_pair = result.pairs.iter()
                .find(|p| p.contract_type == "SimpleERC20");
            
            if let Some(pair) = erc20_pair {
                println!("Found ERC20 pair:");
                println!("  Solidity: {}", pair.solidity_path);
                println!("  ink!: {}", pair.ink_path);
                println!("  Description: {}", pair.description);
                
                // Verify content was loaded
                assert!(!pair.solidity_content.is_empty(), "Solidity content should not be empty");
                assert!(!pair.ink_content.is_empty(), "ink! content should not be empty");
                
                // Verify it contains expected content
                assert!(pair.solidity_content.contains("contract SimpleERC20"), 
                    "Solidity content should contain contract definition");
                assert!(pair.ink_content.contains("#[ink::contract]"), 
                    "ink! content should contain contract attribute");
            } else {
                println!("Available pairs:");
                for pair in &result.pairs {
                    println!("  - {}", pair.contract_type);
                }
                panic!("ERC20 pair not found");
            }
        }
        Err(e) => {
            panic!("Contract matching failed: {}", e);
        }
    }
}