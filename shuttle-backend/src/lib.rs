// Library exports for dynavest-shuttle-backend
pub mod contract_matcher;
pub mod training_embedder;
pub mod rag_system;
pub mod gemini_client;
pub mod parsers;
pub mod sample_data;
pub mod hyperbridge;
pub mod chat;
pub mod polkadot;
pub mod polkadot_defi_knowledge;
pub mod defi_service;
pub mod contract_service;

// Re-export commonly used items
pub use contract_matcher::{ContractMatcher, ContractPair, ContractMatchResult};
pub use training_embedder::{TrainingEmbedder, TrainingPair, EmbeddingResult};
pub use rag_system::RAGSystem;

// Common data structures needed by multiple modules
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedResponse {
    pub query: String,
    pub summary: String,
    pub examples: Vec<CodeExample>,
    pub help_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeExample {
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub source_file: Option<String>,
    pub relevance_score: f32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Strategy {
    pub id: Uuid,
    pub account_id: String,
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
    pub contract_strategy_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}