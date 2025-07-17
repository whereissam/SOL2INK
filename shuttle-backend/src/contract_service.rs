use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use subxt::{
    client::OnlineClient,
    config::SubstrateConfig,
    utils::AccountId32,
};
use tracing::info;
use std::sync::Mutex;
use std::collections::HashMap;

// Contract metadata and types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractStrategy {
    pub id: u32,
    pub name: String,
    pub creator: String,
    pub risk_level: u8,
    pub parameters: String,
    pub balance: u128,
    pub total_invested: u128,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStrategyParams {
    pub name: String,
    pub risk_level: u8,
    pub parameters: String,
    pub initial_investment: Option<u128>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentParams {
    pub strategy_id: u32,
    pub amount: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawParams {
    pub strategy_id: u32,
    pub amount: u128,
}

pub struct ContractService {
    #[allow(dead_code)]
    client: Option<OnlineClient<SubstrateConfig>>,
    #[allow(dead_code)]
    strategy_manager_address: AccountId32,
    #[allow(dead_code)]
    dynavest_strategy_address: AccountId32,
    // Mock storage for offline mode
    mock_strategies: Mutex<HashMap<String, Vec<ContractStrategy>>>,
    next_strategy_id: Mutex<u32>,
}

impl ContractService {
    pub async fn new() -> Result<Self> {
        // Try multiple RPC endpoints in order of preference
        let rpc_endpoints = vec![
            "wss://rococo-contracts-rpc.polkadot.io",
            "wss://rpc.polkadot.io",
            "wss://kusama-rpc.polkadot.io",
        ];

        let mut client = None;
        let mut last_error = None;

        for endpoint in rpc_endpoints {
            match OnlineClient::<SubstrateConfig>::from_url(endpoint).await {
                Ok(c) => {
                    info!("Successfully connected to RPC endpoint: {}", endpoint);
                    client = Some(c);
                    break;
                }
                Err(e) => {
                    info!("Failed to connect to {}: {}", endpoint, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // These would be the actual deployed contract addresses
        let strategy_manager_address = AccountId32::from_str(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
        )?;
        
        let dynavest_strategy_address = AccountId32::from_str(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
        )?;

        Ok(Self {
            client,
            strategy_manager_address,
            dynavest_strategy_address,
            mock_strategies: Mutex::new(HashMap::new()),
            next_strategy_id: Mutex::new(1),
        })
    }

    pub async fn new_mock() -> Result<Self> {
        // Create a mock client that doesn't require network connection
        // This will allow the service to start even if RPC endpoints are down
        info!("Creating mock ContractService for offline operation");
        
        let strategy_manager_address = AccountId32::from_str(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
        )?;
        
        let dynavest_strategy_address = AccountId32::from_str(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
        )?;

        Ok(Self {
            client: None, // No client for mock mode
            strategy_manager_address,
            dynavest_strategy_address,
            mock_strategies: Mutex::new(HashMap::new()),
            next_strategy_id: Mutex::new(1),
        })
    }

    pub async fn create_strategy_on_chain(
        &self,
        user_account: &str,
        params: CreateStrategyParams,
    ) -> Result<u32> {
        info!("Creating strategy on chain for user: {}", user_account);

        // For now, we'll return a mock strategy ID
        // In a real implementation, this would:
        // 1. Create a signed transaction
        // 2. Call the contract's save_strategy method
        // 3. Submit the transaction and wait for finalization
        // 4. Parse the events to get the strategy ID

        let strategy_id = self.mock_create_strategy(params).await?;
        info!("Strategy created with ID: {}", strategy_id);

        Ok(strategy_id)
    }

    pub async fn invest_in_strategy(
        &self,
        user_account: &str,
        params: InvestmentParams,
    ) -> Result<String> {
        info!("Investing in strategy {} for user: {}", params.strategy_id, user_account);

        // For now, we'll return a mock transaction hash
        // In a real implementation, this would:
        // 1. Create a signed transaction with the investment amount
        // 2. Call the contract's invest_in_strategy method
        // 3. Submit the transaction and wait for finalization
        // 4. Return the transaction hash

        let tx_hash = self.mock_invest_in_strategy(params).await?;
        info!("Investment transaction hash: {}", tx_hash);

        Ok(tx_hash)
    }

    pub async fn withdraw_from_strategy(
        &self,
        user_account: &str,
        params: WithdrawParams,
    ) -> Result<String> {
        info!("Withdrawing from strategy {} for user: {}", params.strategy_id, user_account);

        // For now, we'll return a mock transaction hash
        // In a real implementation, this would:
        // 1. Create a signed transaction
        // 2. Call the contract's withdraw_from_strategy method
        // 3. Submit the transaction and wait for finalization
        // 4. Return the transaction hash

        let tx_hash = self.mock_withdraw_from_strategy(params).await?;
        info!("Withdrawal transaction hash: {}", tx_hash);

        Ok(tx_hash)
    }

    pub async fn get_user_strategies(&self, user_account: &str) -> Result<Vec<ContractStrategy>> {
        info!("Getting strategies for user: {}", user_account);

        // For now, we'll return mock strategies
        // In a real implementation, this would:
        // 1. Query the contract's get_strategies method
        // 2. Parse the returned data into ContractStrategy structs

        let strategies = self.mock_get_user_strategies(user_account).await?;
        info!("Found {} strategies for user", strategies.len());

        Ok(strategies)
    }

    #[allow(dead_code)]
    pub async fn get_strategy_details(&self, strategy_id: u32) -> Result<Option<ContractStrategy>> {
        info!("Getting details for strategy: {}", strategy_id);

        // For now, we'll return a mock strategy
        // In a real implementation, this would:
        // 1. Query the contract's get_strategy method
        // 2. Parse the returned data into ContractStrategy struct

        let strategy = self.mock_get_strategy_details(strategy_id).await?;
        
        Ok(strategy)
    }

    #[allow(dead_code)]
    pub async fn update_strategy_parameters(
        &self,
        user_account: &str,
        strategy_id: u32,
        new_parameters: String,
    ) -> Result<String> {
        info!("Updating strategy {} parameters for user: {}", strategy_id, user_account);

        // For now, we'll return a mock transaction hash
        // In a real implementation, this would:
        // 1. Create a signed transaction
        // 2. Call the contract's update_strategy method
        // 3. Submit the transaction and wait for finalization
        // 4. Return the transaction hash

        let tx_hash = self.mock_update_strategy(strategy_id, new_parameters).await?;
        info!("Update transaction hash: {}", tx_hash);

        Ok(tx_hash)
    }

    #[allow(dead_code)]
    pub async fn deactivate_strategy(
        &self,
        user_account: &str,
        strategy_id: u32,
    ) -> Result<String> {
        info!("Deactivating strategy {} for user: {}", strategy_id, user_account);

        // For now, we'll return a mock transaction hash
        // In a real implementation, this would:
        // 1. Create a signed transaction
        // 2. Call the contract's deactivate_strategy method
        // 3. Submit the transaction and wait for finalization
        // 4. Return the transaction hash

        let tx_hash = self.mock_deactivate_strategy(strategy_id).await?;
        info!("Deactivation transaction hash: {}", tx_hash);

        Ok(tx_hash)
    }

    #[allow(dead_code)]
    pub async fn get_user_investment(&self, user_account: &str, strategy_id: u32) -> Result<u128> {
        info!("Getting investment for user {} in strategy {}", user_account, strategy_id);

        // For now, we'll return a mock investment amount
        // In a real implementation, this would:
        // 1. Query the contract's get_investment method
        // 2. Return the investment amount

        let investment = self.mock_get_user_investment(user_account, strategy_id).await?;
        
        Ok(investment)
    }

    #[allow(dead_code)]
    pub async fn get_strategy_count(&self) -> Result<u32> {
        info!("Getting total strategy count");

        // For now, we'll return a mock count
        // In a real implementation, this would:
        // 1. Query the contract's get_strategy_count method
        // 2. Return the count

        let count = self.mock_get_strategy_count().await?;
        
        Ok(count)
    }

    // Mock implementations for development/testing
    // These would be replaced with actual contract calls in production

    async fn mock_create_strategy(&self, params: CreateStrategyParams) -> Result<u32> {
        // Simulate some async work
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Get next strategy ID
        let strategy_id = {
            let mut next_id = self.next_strategy_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        // Create strategy object
        let strategy = ContractStrategy {
            id: strategy_id,
            name: params.name,
            creator: "mock_user".to_string(),
            risk_level: params.risk_level,
            parameters: params.parameters,
            balance: params.initial_investment.unwrap_or(0),
            total_invested: params.initial_investment.unwrap_or(0),
            is_active: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        // Store in mock storage
        {
            let mut strategies = self.mock_strategies.lock().unwrap();
            let user_strategies = strategies.entry("mock_user".to_string()).or_insert_with(Vec::new);
            user_strategies.push(strategy);
        }
        
        Ok(strategy_id)
    }

    async fn mock_invest_in_strategy(&self, _params: InvestmentParams) -> Result<String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock transaction hash
        Ok(format!(
            "0x{:x}",
            rand::random::<u64>()
        ))
    }

    async fn mock_withdraw_from_strategy(&self, _params: WithdrawParams) -> Result<String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock transaction hash
        Ok(format!(
            "0x{:x}",
            rand::random::<u64>()
        ))
    }

    async fn mock_get_user_strategies(&self, user_account: &str) -> Result<Vec<ContractStrategy>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return some mock strategies
        Ok(vec![
            ContractStrategy {
                id: 1,
                name: "Polkadot Yield Farming".to_string(),
                creator: user_account.to_string(),
                risk_level: 5,
                parameters: r#"{"protocol": "polkadot", "type": "yield_farming", "apy": 8.5}"#.to_string(),
                balance: 1000000000000, // 1 DOT
                total_invested: 1000000000000,
                is_active: true,
                created_at: chrono::Utc::now().timestamp() as u64,
                updated_at: chrono::Utc::now().timestamp() as u64,
            },
            ContractStrategy {
                id: 2,
                name: "Low Risk Staking".to_string(),
                creator: user_account.to_string(),
                risk_level: 2,
                parameters: r#"{"protocol": "polkadot", "type": "staking", "apy": 12.0}"#.to_string(),
                balance: 2000000000000, // 2 DOT
                total_invested: 2000000000000,
                is_active: true,
                created_at: chrono::Utc::now().timestamp() as u64,
                updated_at: chrono::Utc::now().timestamp() as u64,
            },
        ])
    }

    #[allow(dead_code)]
    async fn mock_get_strategy_details(&self, strategy_id: u32) -> Result<Option<ContractStrategy>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        if strategy_id == 0 {
            return Ok(None);
        }
        
        // Return a mock strategy
        Ok(Some(ContractStrategy {
            id: strategy_id,
            name: format!("Strategy {}", strategy_id),
            creator: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            risk_level: 5,
            parameters: r#"{"protocol": "polkadot", "type": "mock"}"#.to_string(),
            balance: 1000000000000,
            total_invested: 1000000000000,
            is_active: true,
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
        }))
    }

    #[allow(dead_code)]
    async fn mock_update_strategy(&self, _strategy_id: u32, _new_parameters: String) -> Result<String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock transaction hash
        Ok(format!(
            "0x{:x}",
            rand::random::<u64>()
        ))
    }

    #[allow(dead_code)]
    async fn mock_deactivate_strategy(&self, _strategy_id: u32) -> Result<String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock transaction hash
        Ok(format!(
            "0x{:x}",
            rand::random::<u64>()
        ))
    }

    #[allow(dead_code)]
    async fn mock_get_user_investment(&self, _user_account: &str, _strategy_id: u32) -> Result<u128> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock investment amount
        Ok(500000000000) // 0.5 DOT
    }

    #[allow(dead_code)]
    async fn mock_get_strategy_count(&self) -> Result<u32> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return a mock count
        Ok(42)
    }
}

// Helper functions for contract interaction
impl ContractService {
    pub fn validate_strategy_params(params: &CreateStrategyParams) -> Result<()> {
        if params.name.is_empty() {
            return Err(anyhow::anyhow!("Strategy name cannot be empty"));
        }
        
        if params.risk_level < 1 || params.risk_level > 10 {
            return Err(anyhow::anyhow!("Risk level must be between 1 and 10"));
        }
        
        if params.parameters.is_empty() {
            return Err(anyhow::anyhow!("Strategy parameters cannot be empty"));
        }
        
        Ok(())
    }

    pub fn validate_investment_params(params: &InvestmentParams) -> Result<()> {
        if params.amount == 0 {
            return Err(anyhow::anyhow!("Investment amount must be greater than 0"));
        }
        
        Ok(())
    }

    pub fn validate_withdraw_params(params: &WithdrawParams) -> Result<()> {
        if params.amount == 0 {
            return Err(anyhow::anyhow!("Withdrawal amount must be greater than 0"));
        }
        
        Ok(())
    }

    #[allow(dead_code)]
    pub fn format_balance_for_display(balance: u128) -> String {
        // Convert from planck to DOT (assuming 12 decimal places)
        let dot_balance = balance as f64 / 1_000_000_000_000.0;
        format!("{:.4} DOT", dot_balance)
    }

    #[allow(dead_code)]
    pub fn parse_strategy_parameters(params: &str) -> Result<serde_json::Value> {
        serde_json::from_str(params)
            .map_err(|e| anyhow::anyhow!("Failed to parse strategy parameters: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_strategy_params() {
        let valid_params = CreateStrategyParams {
            name: "Test Strategy".to_string(),
            risk_level: 5,
            parameters: "{}".to_string(),
            initial_investment: Some(1000000000000),
        };
        
        assert!(ContractService::validate_strategy_params(&valid_params).is_ok());
        
        let invalid_params = CreateStrategyParams {
            name: "".to_string(),
            risk_level: 11,
            parameters: "".to_string(),
            initial_investment: None,
        };
        
        assert!(ContractService::validate_strategy_params(&invalid_params).is_err());
    }

    #[test]
    fn test_validate_investment_params() {
        let valid_params = InvestmentParams {
            strategy_id: 1,
            amount: 1000000000000,
        };
        
        assert!(ContractService::validate_investment_params(&valid_params).is_ok());
        
        let invalid_params = InvestmentParams {
            strategy_id: 1,
            amount: 0,
        };
        
        assert!(ContractService::validate_investment_params(&invalid_params).is_err());
    }

    #[test]
    fn test_format_balance_for_display() {
        assert_eq!(
            ContractService::format_balance_for_display(1000000000000),
            "1.0000 DOT"
        );
        
        assert_eq!(
            ContractService::format_balance_for_display(500000000000),
            "0.5000 DOT"
        );
    }

    #[test]
    fn test_parse_strategy_parameters() {
        let valid_json = r#"{"protocol": "polkadot", "type": "staking"}"#;
        assert!(ContractService::parse_strategy_parameters(valid_json).is_ok());
        
        let invalid_json = "invalid json";
        assert!(ContractService::parse_strategy_parameters(invalid_json).is_err());
    }

    #[tokio::test]
    async fn test_mock_create_strategy() {
        let service = ContractService::new().await.unwrap();
        
        let params = CreateStrategyParams {
            name: "Test Strategy".to_string(),
            risk_level: 5,
            parameters: "{}".to_string(),
            initial_investment: Some(1000000000000),
        };
        
        let strategy_id = service.mock_create_strategy(params).await.unwrap();
        assert!(strategy_id > 0);
    }

    #[tokio::test]
    async fn test_mock_get_user_strategies() {
        let service = ContractService::new().await.unwrap();
        
        let strategies = service.mock_get_user_strategies("test_user").await.unwrap();
        assert_eq!(strategies.len(), 2);
        assert_eq!(strategies[0].name, "Polkadot Yield Farming");
        assert_eq!(strategies[1].name, "Low Risk Staking");
    }
}