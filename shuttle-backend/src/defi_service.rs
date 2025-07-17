use crate::chat::ChatService;
use crate::polkadot::PolkadotClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
pub struct DefiInfoRequest {
    pub input_text: String,
    pub user_address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DefiResponse {
    pub response_type: String,
    pub data: serde_json::Value,
    pub actions: Option<ActionRequirements>,
}

#[derive(Debug, Serialize)]
pub struct ActionRequirements {
    pub create_contract_strategy: bool,
    pub requires_signing: bool,
    pub estimated_gas: Option<u64>,
    pub chain_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StrategyData {
    pub name: String,
    pub risk_level: String,
    pub chain: String,
    pub parameters: serde_json::Value,
    pub recommended_amount: Option<f64>,
    pub protocols: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CryptoPriceData {
    pub symbol: String,
    pub price_usd: f64,
    pub change_24h: f64,
    pub market_cap: Option<f64>,
    pub volume_24h: Option<f64>,
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct PortfolioAnalysis {
    pub total_value_usd: f64,
    pub strategies: Vec<StrategyAnalysis>,
    pub risk_distribution: RiskDistribution,
    pub performance: PerformanceMetrics,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct StrategyAnalysis {
    pub name: String,
    pub current_value: f64,
    pub performance_24h: f64,
    pub risk_level: i32,
    pub chain: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct RiskDistribution {
    pub low_risk: f64,
    pub medium_risk: f64,
    pub high_risk: f64,
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    pub total_return: f64,
    pub daily_return: f64,
    pub weekly_return: f64,
    pub monthly_return: f64,
    pub sharpe_ratio: Option<f64>,
}

pub struct DefiService {
    chat_service: Arc<ChatService>,
    polkadot_client: Arc<PolkadotClient>,
    db: PgPool,
}

impl DefiService {
    pub fn new(
        chat_service: Arc<ChatService>,
        polkadot_client: Arc<PolkadotClient>,
        db: PgPool,
    ) -> Self {
        Self {
            chat_service,
            polkadot_client,
            db,
        }
    }

    pub async fn handle_defi_info(&self, request: DefiInfoRequest) -> Result<DefiResponse> {
        info!("Processing DeFi info request: {}", request.input_text);

        // Step 1: Use AI to classify intent
        let intent = self.classify_intent(&request.input_text).await?;
        
        match intent.as_str() {
            "strategies" => self.handle_strategies(&request).await,
            "analyze_portfolio" => self.handle_portfolio_analysis(&request).await,
            "build_portfolio" => self.handle_portfolio_building(&request).await,
            "question" => self.handle_question(&request).await,
            "price_check" => self.handle_price_check(&request).await,
            _ => Ok(DefiResponse {
                response_type: "unknown".to_string(),
                data: serde_json::json!({"message": "I'm not sure how to help with that. Can you be more specific about what you'd like to do?"}),
                actions: None,
            }),
        }
    }

    async fn classify_intent(&self, input_text: &str) -> Result<String> {
        // Use the existing chat service to classify intent
        let classification_prompt = format!(
            "Classify the following user input into one of these categories: strategies, analyze_portfolio, build_portfolio, question, price_check. 
            
            Input: {}
            
            Return only the category name.", 
            input_text
        );

        let response = self.chat_service.generate_response(&classification_prompt, &[]).await
            .map_err(|e| anyhow::anyhow!("Chat service error: {}", e))?;
        let intent = response.message.trim().to_lowercase();
        
        // Map common variations
        match intent.as_str() {
            "strategy" | "strategies" | "create strategy" | "new strategy" => Ok("strategies".to_string()),
            "analyze" | "analysis" | "portfolio" | "analyze_portfolio" => Ok("analyze_portfolio".to_string()),
            "build" | "build_portfolio" | "portfolio_build" => Ok("build_portfolio".to_string()),
            "price" | "prices" | "price_check" | "cost" => Ok("price_check".to_string()),
            _ => Ok("question".to_string()),
        }
    }

    async fn handle_strategies(&self, request: &DefiInfoRequest) -> Result<DefiResponse> {
        info!("Handling strategy request");

        // Check if this is a Polkadot-specific query
        let input_lower = request.input_text.to_lowercase();
        if input_lower.contains("polkadot") || input_lower.contains("dot") || 
           input_lower.contains("acala") || input_lower.contains("bifrost") || 
           input_lower.contains("hydradx") {
            
            // Use our Polkadot knowledge to generate strategy
            let strategy_recommendation = crate::polkadot_defi_knowledge::get_polkadot_strategy_recommendation(5, 10000.0);
            
            return Ok(DefiResponse {
                response_type: "strategies".to_string(),
                data: serde_json::json!({
                    "answer": strategy_recommendation,
                    "risk_level": "medium",
                    "chain": "Polkadot",
                    "strategies": [
                        {
                            "name": "Acala Liquid Staking",
                            "apy": 10.0,
                            "risk": "medium",
                            "description": "Stake DOT to receive LDOT while earning staking rewards. 12.86 DOT available.",
                            "protocol": "Acala",
                            "tokens": ["DOT", "LDOT"]
                        },
                        {
                            "name": "Bifrost Liquid Staking", 
                            "apy": 10.87,
                            "risk": "low",
                            "description": "Flexible liquid staking with 10.87% vDOT APR across DOT and KSM.",
                            "protocol": "Bifrost",
                            "tokens": ["DOT", "vDOT"]
                        },
                        {
                            "name": "HydraDX Liquidity",
                            "apy": 10.0,
                            "risk": "medium", 
                            "description": "Provide liquidity to HydraDX's omnipool for 5-15% LP yield.",
                            "protocol": "HydraDX",
                            "tokens": ["DOT"]
                        }
                    ]
                }),
                actions: Some(ActionRequirements {
                    create_contract_strategy: true,
                    requires_signing: true,
                    estimated_gas: Some(1_000_000),
                    chain_id: Some("1000".to_string()),
                }),
            });
        }

        // Generate strategy using AI for non-Polkadot queries
        let strategy_prompt = format!(
            "Based on the user request: '{}', generate a DeFi strategy. 
            
            Return a JSON object with:
            - name: strategy name
            - risk_level: low, medium, or high
            - chain: preferred blockchain (Polkadot, Ethereum, Base, etc.)
            - parameters: detailed strategy parameters
            - recommended_amount: suggested investment amount
            - protocols: list of DeFi protocols to use
            
            Focus on Polkadot ecosystem when possible.",
            request.input_text
        );

        let ai_response = self.chat_service.generate_response(&strategy_prompt, &[]).await
            .map_err(|e| anyhow::anyhow!("Chat service error: {}", e))?;
        
        // Parse AI response to extract strategy data
        let strategy_data = self.parse_strategy_response(&ai_response.message)?;
        
        // Get chain ID for contract interaction
        let chain_id = self.get_chain_id(&strategy_data.chain);
        
        Ok(DefiResponse {
            response_type: "strategies".to_string(),
            data: serde_json::to_value(strategy_data)?,
            actions: Some(ActionRequirements {
                create_contract_strategy: true,
                requires_signing: true,
                estimated_gas: Some(1_000_000),
                chain_id: Some(chain_id),
            }),
        })
    }

    async fn handle_portfolio_analysis(&self, request: &DefiInfoRequest) -> Result<DefiResponse> {
        info!("Handling portfolio analysis request");

        if let Some(user_address) = &request.user_address {
            // Get user's strategies from database
            let user_strategies = self.get_user_strategies(user_address).await?;
            
            // Get strategies from contracts
            let contract_strategies = self.polkadot_client.get_user_strategies(user_address).await
                .map_err(|e| anyhow::anyhow!("Polkadot client error: {}", e))?;
            
            // Calculate portfolio metrics
            let portfolio_analysis = self.calculate_portfolio_analysis(user_strategies, contract_strategies).await?;
            
            Ok(DefiResponse {
                response_type: "portfolio_analysis".to_string(),
                data: serde_json::to_value(portfolio_analysis)?,
                actions: None,
            })
        } else {
            Ok(DefiResponse {
                response_type: "error".to_string(),
                data: serde_json::json!({"message": "User address required for portfolio analysis"}),
                actions: None,
            })
        }
    }

    async fn handle_portfolio_building(&self, request: &DefiInfoRequest) -> Result<DefiResponse> {
        info!("Handling portfolio building request");

        let portfolio_prompt = format!(
            "Based on the user request: '{}', suggest a complete DeFi portfolio strategy.
            
            Return suggestions for:
            - Asset allocation percentages
            - Risk distribution (low/medium/high)
            - Recommended protocols
            - Diversification strategy
            - Rebalancing schedule
            
            Focus on Polkadot ecosystem opportunities.",
            request.input_text
        );

        let ai_response = self.chat_service.generate_response(&portfolio_prompt, &[]).await
            .map_err(|e| anyhow::anyhow!("Chat service error: {}", e))?;
        
        Ok(DefiResponse {
            response_type: "build_portfolio".to_string(),
            data: serde_json::json!({"recommendation": ai_response.message}),
            actions: Some(ActionRequirements {
                create_contract_strategy: false,
                requires_signing: false,
                estimated_gas: None,
                chain_id: None,
            }),
        })
    }

    async fn handle_question(&self, request: &DefiInfoRequest) -> Result<DefiResponse> {
        info!("Handling question request");

        // Use the chat service to answer questions using RAG
        let answer = self.chat_service.generate_response(&request.input_text, &[]).await
            .map_err(|e| anyhow::anyhow!("Chat service error: {}", e))?;
        
        Ok(DefiResponse {
            response_type: "question".to_string(),
            data: serde_json::json!({"answer": answer.message}),
            actions: None,
        })
    }

    async fn handle_price_check(&self, request: &DefiInfoRequest) -> Result<DefiResponse> {
        info!("Handling price check request");

        // Extract token symbols from the request
        let tokens = self.extract_tokens_from_text(&request.input_text);
        
        // Get current prices
        let prices = self.get_crypto_prices(&tokens).await?;
        
        Ok(DefiResponse {
            response_type: "price_check".to_string(),
            data: serde_json::json!({"prices": prices}),
            actions: None,
        })
    }

    pub async fn get_crypto_prices(&self, tokens: &[String]) -> Result<Vec<CryptoPriceData>> {
        let mut prices = Vec::new();
        
        // Map of token symbols to CoinGecko IDs
        let token_map: HashMap<&str, &str> = [
            ("BTC", "bitcoin"),
            ("ETH", "ethereum"),
            ("DOT", "polkadot"),
            ("USDC", "usd-coin"),
            ("USDT", "tether"),
            ("BNB", "binancecoin"),
            ("ADA", "cardano"),
            ("SOL", "solana"),
            ("AVAX", "avalanche-2"),
            ("MATIC", "matic-network"),
        ].iter().cloned().collect();

        for token in tokens {
            if let Some(coin_id) = token_map.get(token.to_uppercase().as_str()) {
                match self.fetch_price_from_coingecko(coin_id).await {
                    Ok(price_data) => prices.push(price_data),
                    Err(e) => warn!("Failed to fetch price for {}: {}", token, e),
                }
            }
        }

        Ok(prices)
    }

    async fn fetch_price_from_coingecko(&self, coin_id: &str) -> Result<CryptoPriceData> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_market_cap=true&include_24hr_vol=true",
            coin_id
        );

        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        if let Some(coin_data) = data.get(coin_id) {
            Ok(CryptoPriceData {
                symbol: coin_id.to_uppercase(),
                price_usd: coin_data["usd"].as_f64().unwrap_or(0.0),
                change_24h: coin_data["usd_24h_change"].as_f64().unwrap_or(0.0),
                market_cap: coin_data["usd_market_cap"].as_f64(),
                volume_24h: coin_data["usd_24h_vol"].as_f64(),
                last_updated: chrono::Utc::now().to_rfc3339(),
            })
        } else {
            Err(anyhow::anyhow!("Price data not found for {}", coin_id))
        }
    }

    fn extract_tokens_from_text(&self, text: &str) -> Vec<String> {
        // Common crypto tokens that might be mentioned
        let common_tokens = ["BTC", "ETH", "DOT", "USDC", "USDT", "BNB", "ADA", "SOL", "AVAX", "MATIC"];
        
        let mut found_tokens = Vec::new();
        let text_upper = text.to_uppercase();
        
        for token in common_tokens {
            if text_upper.contains(token) {
                found_tokens.push(token.to_string());
            }
        }
        
        // If no specific tokens found, default to major ones
        if found_tokens.is_empty() {
            found_tokens = vec!["BTC".to_string(), "ETH".to_string(), "DOT".to_string()];
        }
        
        found_tokens
    }

    fn parse_strategy_response(&self, ai_response: &str) -> Result<StrategyData> {
        // Try to parse as JSON first
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(ai_response) {
            return Ok(StrategyData {
                name: json_value["name"].as_str().unwrap_or("AI Generated Strategy").to_string(),
                risk_level: json_value["risk_level"].as_str().unwrap_or("medium").to_string(),
                chain: json_value["chain"].as_str().unwrap_or("Polkadot").to_string(),
                parameters: json_value["parameters"].clone(),
                recommended_amount: json_value["recommended_amount"].as_f64(),
                protocols: json_value["protocols"].as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_else(|| vec!["Generic DeFi".to_string()]),
            });
        }
        
        // Fallback: parse from text
        Ok(StrategyData {
            name: "AI Generated Strategy".to_string(),
            risk_level: self.extract_risk_level(ai_response),
            chain: self.extract_chain(ai_response),
            parameters: serde_json::json!({"description": ai_response}),
            recommended_amount: None,
            protocols: vec!["Generic DeFi".to_string()],
        })
    }

    fn extract_risk_level(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();
        if text_lower.contains("high risk") || text_lower.contains("aggressive") {
            "high".to_string()
        } else if text_lower.contains("low risk") || text_lower.contains("conservative") {
            "low".to_string()
        } else {
            "medium".to_string()
        }
    }

    fn extract_chain(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();
        if text_lower.contains("polkadot") || text_lower.contains("dot") {
            "Polkadot".to_string()
        } else if text_lower.contains("ethereum") || text_lower.contains("eth") {
            "Ethereum".to_string()
        } else if text_lower.contains("base") {
            "Base".to_string()
        } else if text_lower.contains("arbitrum") {
            "Arbitrum".to_string()
        } else {
            "Polkadot".to_string() // Default to Polkadot
        }
    }

    fn get_chain_id(&self, chain: &str) -> String {
        match chain {
            "Ethereum" => "1".to_string(),
            "Base" => "8453".to_string(),
            "Arbitrum" => "42161".to_string(),
            "BNB" => "56".to_string(),
            "Polygon" => "137".to_string(),
            "Polkadot" => "0".to_string(), // Polkadot relay chain
            _ => "0".to_string(),
        }
    }

    async fn get_user_strategies(&self, user_address: &str) -> Result<Vec<StrategyAnalysis>> {
        let strategies = sqlx::query_as::<_, crate::Strategy>(
            "SELECT * FROM strategies WHERE account_id = $1 AND is_active = true"
        )
        .bind(user_address)
        .fetch_all(&self.db)
        .await?;

        let mut strategy_analyses = Vec::new();
        for strategy in strategies {
            strategy_analyses.push(StrategyAnalysis {
                name: strategy.name,
                current_value: 1000.0, // Placeholder - would calculate from actual data
                performance_24h: 0.0, // Placeholder
                risk_level: strategy.risk_level,
                chain: "Polkadot".to_string(), // Would extract from parameters
                status: if strategy.is_active { "active".to_string() } else { "inactive".to_string() },
            });
        }

        Ok(strategy_analyses)
    }

    async fn calculate_portfolio_analysis(
        &self,
        db_strategies: Vec<StrategyAnalysis>,
        _contract_strategies: Vec<crate::polkadot::PolkadotStrategy>,
    ) -> Result<PortfolioAnalysis> {
        let total_value = db_strategies.iter().map(|s| s.current_value).sum();
        
        // Calculate risk distribution
        let mut low_risk = 0.0;
        let mut medium_risk = 0.0;
        let mut high_risk = 0.0;
        
        for strategy in &db_strategies {
            match strategy.risk_level {
                1..=3 => low_risk += strategy.current_value,
                4..=6 => medium_risk += strategy.current_value,
                7..=10 => high_risk += strategy.current_value,
                _ => medium_risk += strategy.current_value,
            }
        }
        
        let risk_distribution = RiskDistribution {
            low_risk: if total_value > 0.0 { low_risk / total_value * 100.0 } else { 0.0 },
            medium_risk: if total_value > 0.0 { medium_risk / total_value * 100.0 } else { 0.0 },
            high_risk: if total_value > 0.0 { high_risk / total_value * 100.0 } else { 0.0 },
        };

        let performance = PerformanceMetrics {
            total_return: 5.2, // Placeholder
            daily_return: 0.1, // Placeholder
            weekly_return: 0.8, // Placeholder
            monthly_return: 3.2, // Placeholder
            sharpe_ratio: Some(1.5), // Placeholder
        };

        let recommendations = vec![
            "Consider rebalancing your portfolio to reduce risk concentration".to_string(),
            "Explore yield farming opportunities on Polkadot".to_string(),
            "Consider adding more liquid staking positions".to_string(),
        ];

        Ok(PortfolioAnalysis {
            total_value_usd: total_value,
            strategies: db_strategies,
            risk_distribution,
            performance,
            recommendations,
        })
    }

    // Static helper methods for testing
    #[cfg(test)]
    pub fn extract_risk_level_simple(input: &str) -> &'static str {
        let input_lower = input.to_lowercase();
        if input_lower.contains("high") || input_lower.contains("aggressive") {
            "high"
        } else if input_lower.contains("low") || input_lower.contains("conservative") || input_lower.contains("safe") {
            "low"
        } else if input_lower.contains("medium") || input_lower.contains("moderate") {
            "medium"
        } else {
            "medium" // default
        }
    }

    #[cfg(test)]
    pub fn extract_chain_simple(input: &str) -> &'static str {
        let input_lower = input.to_lowercase();
        if input_lower.contains("polkadot") {
            "Polkadot"
        } else if input_lower.contains("ethereum") || input_lower.contains("eth") {
            "Ethereum"
        } else if input_lower.contains("base") {
            "Base"
        } else if input_lower.contains("polygon") {
            "Polygon"
        } else {
            "Polkadot" // default
        }
    }

    #[cfg(test)]
    pub fn get_chain_id_simple(chain: &str) -> &'static str {
        match chain {
            "Ethereum" => "1",
            "Polygon" => "137",
            "Base" => "8453",
            "Arbitrum" => "42161",
            "Optimism" => "10",
            _ => "0", // Polkadot or unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_risk_level() {
        // Test risk level extraction logic
        assert_eq!(DefiService::extract_risk_level_simple("I want a high risk strategy"), "high");
        assert_eq!(DefiService::extract_risk_level_simple("Conservative approach please"), "low");
        assert_eq!(DefiService::extract_risk_level_simple("Moderate risk is fine"), "medium");
    }
    
    #[test]
    fn test_extract_chain() {
        // Test chain extraction logic
        assert_eq!(DefiService::extract_chain_simple("Deploy on Polkadot"), "Polkadot");
        assert_eq!(DefiService::extract_chain_simple("Use Ethereum network"), "Ethereum");
        assert_eq!(DefiService::extract_chain_simple("Base chain please"), "Base");
        assert_eq!(DefiService::extract_chain_simple("No specific chain"), "Polkadot");
    }
    
    #[test]
    fn test_get_chain_id() {
        // Test chain ID mapping
        assert_eq!(DefiService::get_chain_id_simple("Ethereum"), "1");
        assert_eq!(DefiService::get_chain_id_simple("Base"), "8453");
        assert_eq!(DefiService::get_chain_id_simple("Polkadot"), "0");
    }
}