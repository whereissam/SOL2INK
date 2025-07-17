use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolkadotProtocol {
    pub name: String,
    pub liquid_staking_apr: Option<String>,
    pub lending_yield_apr: Option<String>,
    pub highlights: Vec<String>,
    pub category: String,
    pub risk_level: u8, // 1-10
    pub tvl_usd: Option<f64>,
    pub supported_assets: Vec<String>,
}

pub fn get_polkadot_protocols() -> HashMap<String, PolkadotProtocol> {
    let mut protocols = HashMap::new();

    protocols.insert("acala".to_string(), PolkadotProtocol {
        name: "Acala".to_string(),
        liquid_staking_apr: Some("8-12%".to_string()),
        lending_yield_apr: Some("5-20%".to_string()),
        highlights: vec![
            "Multi-product DeFi: DEX, stablecoin, staking".to_string(),
            "EVM compatibility".to_string(),
            "12.86 DOT available".to_string(),
            "aUSD stablecoin support".to_string(),
        ],
        category: "DeFi Hub + DEX".to_string(),
        risk_level: 5, // Medium risk
        tvl_usd: Some(70_000_000.0),
        supported_assets: vec!["DOT".to_string(), "ACA".to_string(), "aUSD".to_string(), "LDOT".to_string()],
    });

    protocols.insert("bifrost".to_string(), PolkadotProtocol {
        name: "Bifrost".to_string(),
        liquid_staking_apr: Some("10.87%".to_string()),
        lending_yield_apr: None,
        highlights: vec![
            "Flexible liquid staking across DOT, KSM".to_string(),
            "vDOT/vKSM derivatives".to_string(),
            "Low-medium risk".to_string(),
            "Cross-chain support".to_string(),
        ],
        category: "Liquid Staking".to_string(),
        risk_level: 4, // Low-Medium risk
        tvl_usd: Some(110_000_000.0),
        supported_assets: vec!["DOT".to_string(), "vDOT".to_string(), "vKSM".to_string(), "BNC".to_string()],
    });

    protocols.insert("hydradx".to_string(), PolkadotProtocol {
        name: "HydraDX".to_string(),
        liquid_staking_apr: None,
        lending_yield_apr: Some("5-15%".to_string()),
        highlights: vec![
            "Low-slippage DEX using omnipool".to_string(),
            "Soon expanding to lending & stablecoins".to_string(),
            "AMM with LP yield".to_string(),
            "Medium risk".to_string(),
        ],
        category: "AMM DEX".to_string(),
        risk_level: 5, // Medium risk
        tvl_usd: Some(50_000_000.0),
        supported_assets: vec!["DOT".to_string(), "HDX".to_string()],
    });

    protocols
}

pub fn get_polkadot_strategy_recommendation(risk_level: u8, amount: f64) -> String {
    let protocols = get_polkadot_protocols();
    
    let recommendations = match risk_level {
        1..=3 => {
            // Low risk: Focus on liquid staking
            vec![
                "Bifrost (vDOT) - 10.87% APR with low-medium risk",
                "Acala liquid staking - 8-12% APR with established DeFi hub",
            ]
        },
        4..=6 => {
            // Medium risk: Mix of staking and lending
            vec![
                "Acala DeFi hub - Multiple opportunities with 5-20% yields, 12.86 DOT available",
                "Bifrost liquid staking - 10.87% vDOT with flexible cross-chain support",
                "HydraDX AMM - 5-15% LP yields with low-slippage omnipool",
            ]
        },
        7..=10 => {
            // High risk: Aggressive yield farming
            vec![
                "Acala high-yield lending - Up to 20% APR with EVM compatibility",
                "Multi-protocol strategy across Acala + Bifrost + HydraDX",
                "Advanced DeFi strategies with cross-chain opportunities",
            ]
        },
        _ => vec!["Please specify a risk level between 1-10"],
    };

    let mut strategy = format!("🎯 **Polkadot DeFi Strategy (Risk Level: {}/10)**\n\n", risk_level);
    strategy.push_str(&format!("For ${:.2} investment:\n\n", amount));
    
    for (i, rec) in recommendations.iter().enumerate() {
        strategy.push_str(&format!("{}. {}\n", i + 1, rec));
    }
    
    strategy.push_str("\n📊 **Available Protocols:**\n");
    for protocol in protocols.values() {
        strategy.push_str(&format!("• **{}** ({}): {}\n", 
            protocol.name, 
            protocol.category, 
            protocol.highlights.join(", ")
        ));
    }
    
    strategy.push_str("\n💡 **Real Data:**\n");
    strategy.push_str("• Acala: 12.86 DOT available, 8-12% LDOT staking, 5-20% lending\n");
    strategy.push_str("• Bifrost: 10.87% vDOT staking, ~$110M TVL, flexible liquid staking\n");
    strategy.push_str("• HydraDX: 5-15% LP yield, omnipool technology, expanding features\n");
    
    strategy
}

pub fn search_polkadot_protocols(query: &str) -> Vec<PolkadotProtocol> {
    let protocols = get_polkadot_protocols();
    let query_lower = query.to_lowercase();
    
    protocols.values()
        .filter(|protocol| {
            protocol.name.to_lowercase().contains(&query_lower) ||
            protocol.category.to_lowercase().contains(&query_lower) ||
            protocol.highlights.iter().any(|h| h.to_lowercase().contains(&query_lower))
        })
        .cloned()
        .collect()
}