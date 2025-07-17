use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Cross-chain liquidity pool data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainLPData {
    pub protocol: String,
    pub chain: String,
    pub token_pair: String,
    pub liquidity_usd: f64,
    pub volume_24h: f64,
    pub apy: f64,
    pub risk_score: u8,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Uniswap V3 pool data from Ethereum
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UniswapV3Pool {
    id: String,
    token0: UniswapToken,
    token1: UniswapToken,
    #[serde(rename = "totalValueLockedUSD")]
    total_value_locked_usd: String,
    #[serde(rename = "volumeUSD")]
    volume_usd: String,
    #[serde(rename = "feeTier")]
    fee_tier: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UniswapToken {
    symbol: String,
    name: String,
    decimals: String,
}

#[derive(Debug, Deserialize)]
struct UniswapResponse {
    data: UniswapData,
}

#[derive(Debug, Deserialize)]
struct UniswapData {
    pools: Vec<UniswapV3Pool>,
}

/// Hyperbridge-compatible cross-chain data fetcher
#[derive(Clone)]
pub struct HyperbridgeClient {
    http_client: Client,
    #[allow(dead_code)]
    ethereum_rpc_url: String,
    #[allow(dead_code)]
    polygon_rpc_url: String,
    uniswap_subgraph_url: String,
}

impl HyperbridgeClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            ethereum_rpc_url: "https://mainnet.infura.io/v3/demo".to_string(),
            polygon_rpc_url: "https://polygon-mainnet.infura.io/v3/demo".to_string(),
            uniswap_subgraph_url: "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string(),
        }
    }

    /// Fetch cross-chain LP data for strategy generation
    pub async fn fetch_cross_chain_lp_data(&self, risk_level: u8) -> Result<Vec<CrossChainLPData>> {
        info!("Fetching cross-chain LP data for risk level: {}", risk_level);
        
        let mut all_lp_data = Vec::new();
        
        // Fetch Uniswap V3 data from Ethereum
        match self.fetch_uniswap_v3_data().await {
            Ok(mut uniswap_data) => {
                info!("Fetched {} Uniswap V3 pools", uniswap_data.len());
                all_lp_data.append(&mut uniswap_data);
            }
            Err(e) => {
                warn!("Failed to fetch Uniswap V3 data: {}", e);
            }
        }
        
        // Fetch additional DeFi protocols (mock data for now)
        match self.fetch_compound_data().await {
            Ok(mut compound_data) => {
                info!("Fetched {} Compound pools", compound_data.len());
                all_lp_data.append(&mut compound_data);
            }
            Err(e) => {
                warn!("Failed to fetch Compound data: {}", e);
            }
        }
        
        // Filter by risk level
        let filtered_data = self.filter_by_risk_level(all_lp_data, risk_level);
        
        info!("Returning {} LP opportunities matching risk level {}", filtered_data.len(), risk_level);
        Ok(filtered_data)
    }

    /// Fetch Uniswap V3 pool data from Ethereum via The Graph
    async fn fetch_uniswap_v3_data(&self) -> Result<Vec<CrossChainLPData>> {
        let query = r#"
        {
            pools(first: 20, orderBy: totalValueLockedUSD, orderDirection: desc) {
                id
                token0 {
                    symbol
                    name
                    decimals
                }
                token1 {
                    symbol
                    name
                    decimals
                }
                totalValueLockedUSD
                volumeUSD
                feeTier
            }
        }
        "#;

        let request_body = serde_json::json!({
            "query": query
        });

        let response = self.http_client
            .post(&self.uniswap_subgraph_url)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch Uniswap data: {}", response.status()));
        }

        let uniswap_response: UniswapResponse = response.json().await?;
        let mut lp_data = Vec::new();

        for pool in uniswap_response.data.pools {
            let tvl = pool.total_value_locked_usd.parse::<f64>().unwrap_or(0.0);
            let volume = pool.volume_usd.parse::<f64>().unwrap_or(0.0);
            let fee_tier = pool.fee_tier.parse::<u32>().unwrap_or(3000);
            
            // Calculate estimated APY based on fees and volume
            let estimated_apy = if tvl > 0.0 {
                (volume * (fee_tier as f64 / 1000000.0) * 365.0) / tvl * 100.0
            } else {
                0.0
            };

            // Calculate risk score based on TVL and volatility
            let risk_score = self.calculate_risk_score(tvl, estimated_apy);

            lp_data.push(CrossChainLPData {
                protocol: "Uniswap V3".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: format!("{}/{}", pool.token0.symbol, pool.token1.symbol),
                liquidity_usd: tvl,
                volume_24h: volume,
                apy: estimated_apy,
                risk_score,
                last_updated: chrono::Utc::now(),
            });
        }

        Ok(lp_data)
    }

    /// Fetch Compound lending data (mock implementation)
    async fn fetch_compound_data(&self) -> Result<Vec<CrossChainLPData>> {
        info!("Fetching Compound lending data (mock implementation)");
        
        // Mock Compound data for demonstration
        let compound_pools = vec![
            CrossChainLPData {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: "USDC".to_string(),
                liquidity_usd: 1_500_000_000.0,
                volume_24h: 50_000_000.0,
                apy: 3.2,
                risk_score: 2,
                last_updated: chrono::Utc::now(),
            },
            CrossChainLPData {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: "ETH".to_string(),
                liquidity_usd: 800_000_000.0,
                volume_24h: 30_000_000.0,
                apy: 2.8,
                risk_score: 3,
                last_updated: chrono::Utc::now(),
            },
            CrossChainLPData {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: "WBTC".to_string(),
                liquidity_usd: 400_000_000.0,
                volume_24h: 15_000_000.0,
                apy: 1.9,
                risk_score: 4,
                last_updated: chrono::Utc::now(),
            },
        ];

        Ok(compound_pools)
    }

    /// Calculate risk score based on TVL and APY
    fn calculate_risk_score(&self, tvl: f64, apy: f64) -> u8 {
        let mut risk_score = 5; // Default medium risk
        
        // Lower risk for higher TVL
        if tvl > 1_000_000_000.0 {
            risk_score -= 2;
        } else if tvl > 100_000_000.0 {
            risk_score -= 1;
        }
        
        // Higher risk for higher APY
        if apy > 20.0 {
            risk_score += 3;
        } else if apy > 10.0 {
            risk_score += 2;
        } else if apy > 5.0 {
            risk_score += 1;
        }
        
        // Ensure score is in valid range (1-10)
        risk_score.max(1).min(10)
    }

    /// Filter LP data by risk level
    fn filter_by_risk_level(&self, lp_data: Vec<CrossChainLPData>, target_risk: u8) -> Vec<CrossChainLPData> {
        lp_data
            .into_iter()
            .filter(|pool| {
                // Allow pools within ±2 risk levels
                let risk_diff = (pool.risk_score as i8 - target_risk as i8).abs();
                risk_diff <= 2
            })
            .collect()
    }

    /// Get cross-chain strategy recommendations
    pub async fn get_strategy_recommendations(&self, risk_level: u8, investment_amount: f64) -> Result<Vec<StrategyRecommendation>> {
        info!("Getting strategy recommendations for risk level: {}, amount: ${}", risk_level, investment_amount);
        
        let lp_data = self.fetch_cross_chain_lp_data(risk_level).await?;
        let mut recommendations = Vec::new();

        for pool in lp_data.iter().take(5) { // Top 5 recommendations
            let allocation_percentage = self.calculate_allocation_percentage(pool, risk_level, investment_amount);
            let allocated_amount = investment_amount * (allocation_percentage / 100.0);

            recommendations.push(StrategyRecommendation {
                protocol: pool.protocol.clone(),
                chain: pool.chain.clone(),
                token_pair: pool.token_pair.clone(),
                allocation_percentage,
                allocated_amount,
                expected_apy: pool.apy,
                risk_score: pool.risk_score,
                reasoning: self.generate_reasoning(pool, risk_level),
            });
        }

        Ok(recommendations)
    }

    /// Calculate allocation percentage based on risk and diversification
    fn calculate_allocation_percentage(&self, pool: &CrossChainLPData, risk_level: u8, _investment_amount: f64) -> f64 {
        let base_allocation: f64 = match risk_level {
            1..=3 => 30.0, // Conservative: larger single allocations
            4..=6 => 20.0, // Moderate: balanced allocations
            7..=10 => 15.0, // Aggressive: more diversified
            _ => 20.0,
        };

        // Adjust based on pool quality
        let quality_multiplier: f64 = if pool.liquidity_usd > 500_000_000.0 && pool.risk_score <= 4 {
            1.2
        } else if pool.liquidity_usd > 100_000_000.0 {
            1.0
        } else {
            0.8
        };

        (base_allocation * quality_multiplier).min(50.0) // Cap at 50%
    }

    /// Generate reasoning for strategy recommendation
    fn generate_reasoning(&self, pool: &CrossChainLPData, risk_level: u8) -> String {
        let risk_desc = match risk_level {
            1..=3 => "conservative",
            4..=6 => "moderate",
            7..=10 => "aggressive",
            _ => "moderate",
        };

        format!(
            "This {} pool on {} offers {:.2}% APY with a risk score of {}/10, suitable for {} investors. TVL of ${:.1}M provides good liquidity.",
            pool.protocol,
            pool.chain,
            pool.apy,
            pool.risk_score,
            risk_desc,
            pool.liquidity_usd / 1_000_000.0
        )
    }
}

/// Strategy recommendation based on cross-chain data
#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyRecommendation {
    pub protocol: String,
    pub chain: String,
    pub token_pair: String,
    pub allocation_percentage: f64,
    pub allocated_amount: f64,
    pub expected_apy: f64,
    pub risk_score: u8,
    pub reasoning: String,
}

/// Enhanced strategy parameters including cross-chain data
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedStrategyParams {
    pub base_strategy: String,
    pub cross_chain_data: Vec<CrossChainLPData>,
    pub recommendations: Vec<StrategyRecommendation>,
    pub total_expected_apy: f64,
    pub diversification_score: f64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl EnhancedStrategyParams {
    pub fn new(
        base_strategy: String,
        cross_chain_data: Vec<CrossChainLPData>,
        recommendations: Vec<StrategyRecommendation>,
    ) -> Self {
        let total_expected_apy = recommendations
            .iter()
            .map(|r| r.expected_apy * (r.allocation_percentage / 100.0))
            .sum();

        let diversification_score = Self::calculate_diversification_score(&recommendations);

        Self {
            base_strategy,
            cross_chain_data,
            recommendations,
            total_expected_apy,
            diversification_score,
            generated_at: chrono::Utc::now(),
        }
    }

    fn calculate_diversification_score(recommendations: &[StrategyRecommendation]) -> f64 {
        let unique_protocols = recommendations
            .iter()
            .map(|r| &r.protocol)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let unique_chains = recommendations
            .iter()
            .map(|r| &r.chain)
            .collect::<std::collections::HashSet<_>>()
            .len();

        // Score based on diversification across protocols and chains
        let protocol_score = (unique_protocols as f64 / recommendations.len() as f64) * 50.0;
        let chain_score = (unique_chains as f64 / recommendations.len() as f64) * 50.0;

        (protocol_score + chain_score).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_score_calculation() {
        let client = HyperbridgeClient::new();
        
        // High TVL, low APY = low risk  
        assert_eq!(client.calculate_risk_score(2_000_000_000.0, 3.0), 3);
        
        // Low TVL, high APY = high risk
        assert_eq!(client.calculate_risk_score(10_000_000.0, 25.0), 8);
        
        // Medium TVL, medium APY = medium risk
        assert_eq!(client.calculate_risk_score(500_000_000.0, 8.0), 5);
    }

    #[test]
    fn test_diversification_score() {
        let recommendations = vec![
            StrategyRecommendation {
                protocol: "Uniswap V3".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: "USDC/ETH".to_string(),
                allocation_percentage: 50.0,
                allocated_amount: 5000.0,
                expected_apy: 8.0,
                risk_score: 4,
                reasoning: "Test".to_string(),
            },
            StrategyRecommendation {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                token_pair: "USDC".to_string(),
                allocation_percentage: 30.0,
                allocated_amount: 3000.0,
                expected_apy: 3.0,
                risk_score: 2,
                reasoning: "Test".to_string(),
            },
            StrategyRecommendation {
                protocol: "Aave".to_string(),
                chain: "Polygon".to_string(),
                token_pair: "WMATIC".to_string(),
                allocation_percentage: 20.0,
                allocated_amount: 2000.0,
                expected_apy: 6.0,
                risk_score: 3,
                reasoning: "Test".to_string(),
            },
        ];

        let score = EnhancedStrategyParams::calculate_diversification_score(&recommendations);
        assert!(score > 75.0); // Should be well diversified
    }
}