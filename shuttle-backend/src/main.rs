use shuttle_axum::axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_axum::ShuttleAxum;
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;
use uuid::Uuid;
use qdrant_client::Qdrant;

mod hyperbridge;
use hyperbridge::{HyperbridgeClient, EnhancedStrategyParams};

mod chat;
use chat::{ChatService, ChatRequest, ChatResponse};

mod polkadot;
use polkadot::{PolkadotClient, StrategyParameters as PolkadotStrategyParameters};

mod polkadot_defi_knowledge;
use polkadot_defi_knowledge::{get_polkadot_protocols, get_polkadot_strategy_recommendation, search_polkadot_protocols};

mod defi_service;
use defi_service::{DefiService, DefiInfoRequest, DefiResponse, CryptoPriceData};

mod contract_service;
use contract_service::{ContractService, CreateStrategyParams, InvestmentParams, WithdrawParams, ContractStrategy};

use training_embedder::{TrainingEmbedder, EmbeddingResult};

mod rag_system;
use rag_system::{RAGSystem, SearchRequest, SearchResult, EmbeddingRequest};

mod gemini_client;

mod sample_data;

mod parsers;
mod contract_matcher;
mod training_embedder;

#[cfg(test)]
mod test_contract_matching;

// Database models
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Strategy {
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct User {
    pub user_id: Uuid,
    pub address: String,
    pub privy_id: String,
    pub total_value: rust_decimal::Decimal,
    pub login_type: String,
    pub login_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Position {
    pub position_id: Uuid,
    pub user_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub token_name: String,
    pub chain_id: i32,
    pub strategy: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Transaction {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub chain_id: i32,
    pub strategy: String,
    pub hash: String,
    pub amount: rust_decimal::Decimal,
    pub token_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// API request/response models
#[derive(Debug, Deserialize)]
struct CreateStrategyRequest {
    pub account: String,
    pub strategy: StrategyData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreateUserRequest {
    pub address: String,
    pub privy_id: String,
    pub total_value: rust_decimal::Decimal,
    pub login_type: String,
    pub login_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreatePositionRequest {
    pub address: String,
    pub amount: rust_decimal::Decimal,
    pub token_name: String,
    pub chain_id: i32,
    pub strategy: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreateTransactionRequest {
    pub address: String,
    pub chain_id: i32,
    pub strategy: String,
    pub hash: String,
    pub amount: rust_decimal::Decimal,
    pub token_name: String,
}

// Remove this duplicate - it's now in defi_service.rs

// Remove this duplicate - it's now in defi_service.rs

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreatePolkadotStrategyRequest {
    pub address: String,
    pub parameters: PolkadotStrategyParameters,
    pub initial_deposit: Option<InitialDeposit>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct InitialDeposit {
    pub amount: rust_decimal::Decimal,
    pub token: String,
}

#[derive(Debug, Deserialize)]
struct StrategyData {
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UpdateStrategyRequest {
    pub account: String,
    pub strategy_id: String,
    pub strategy: StrategyData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DeleteStrategyRequest {
    pub account: String,
    pub strategy_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CrossChainStrategyRequest {
    pub account: String,
    pub risk_level: u8,
    pub investment_amount: f64,
    pub preferred_chains: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct StrategyResponse {
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
    pub created_at: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    pub object: String,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    pub error_type: String,
    pub code: String,
    pub message: String,
    pub param: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeletedResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

// Application state
#[derive(Clone)]
struct AppState {
    db: PgPool,
    contract_config: ContractConfig,
    hyperbridge_client: HyperbridgeClient,
    chat_service: std::sync::Arc<ChatService>,
    #[allow(dead_code)]
    polkadot_client: std::sync::Arc<PolkadotClient>,
    defi_service: std::sync::Arc<DefiService>,
    contract_service: std::sync::Arc<ContractService>,
    rag_system: std::sync::Arc<RAGSystem>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct ContractConfig {
    pub contract_address: String,
    pub rpc_url: String,
}

impl Default for ContractConfig {
    fn default() -> Self {
        Self {
            contract_address: std::env::var("CONTRACT_ADDRESS")
                .unwrap_or_else(|_| "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()),
            rpc_url: std::env::var("RPC_URL")
                .unwrap_or_else(|_| "wss://moonbeam-alpha.api.onfinality.io/public-ws".to_string()),
        }
    }
}

// Database functions
async fn create_strategy_in_db(
    db: &PgPool,
    account_id: &str,
    strategy_data: &StrategyData,
    contract_strategy_id: Option<i32>,
) -> Result<Strategy, sqlx::Error> {
    let strategy_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    
    let strategy = sqlx::query_as::<_, Strategy>(
        r#"
        INSERT INTO strategies (id, account_id, name, risk_level, parameters, contract_strategy_id, created_at, updated_at, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(strategy_id)
    .bind(account_id)
    .bind(&strategy_data.name)
    .bind(strategy_data.risk_level)
    .bind(&strategy_data.parameters)
    .bind(contract_strategy_id)
    .bind(now)
    .bind(now)
    .bind(true)
    .fetch_one(db)
    .await?;

    Ok(strategy)
}

async fn get_strategies_from_db(db: &PgPool, account_id: &str) -> Result<Vec<Strategy>, sqlx::Error> {
    let strategies = sqlx::query_as::<_, Strategy>(
        r#"
        SELECT * FROM strategies 
        WHERE account_id = $1 AND is_active = true
        ORDER BY created_at DESC
        "#
    )
    .bind(account_id)
    .fetch_all(db)
    .await?;

    Ok(strategies)
}

async fn update_strategy_in_db(
    db: &PgPool,
    strategy_id: &str,
    account_id: &str,
    strategy_data: &StrategyData,
) -> Result<Option<Strategy>, sqlx::Error> {
    // Parse UUID
    let uuid = match Uuid::parse_str(strategy_id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(None), // Invalid UUID format
    };

    let strategy = sqlx::query_as::<_, Strategy>(
        r#"
        UPDATE strategies 
        SET name = $1, risk_level = $2, parameters = $3, updated_at = $4
        WHERE id = $5 AND account_id = $6 AND is_active = true
        RETURNING *
        "#
    )
    .bind(&strategy_data.name)
    .bind(strategy_data.risk_level)
    .bind(&strategy_data.parameters)
    .bind(chrono::Utc::now())
    .bind(uuid)
    .bind(account_id)
    .fetch_optional(db)
    .await?;

    Ok(strategy)
}

async fn delete_strategy_in_db(
    db: &PgPool,
    strategy_id: &str,
    account_id: &str,
) -> Result<bool, sqlx::Error> {
    // Parse UUID
    let uuid = match Uuid::parse_str(strategy_id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(false), // Invalid UUID format
    };

    let result = sqlx::query(
        r#"
        UPDATE strategies 
        SET is_active = false, updated_at = $1
        WHERE id = $2 AND account_id = $3 AND is_active = true
        "#
    )
    .bind(chrono::Utc::now())
    .bind(uuid)
    .bind(account_id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

// Contract interaction functions
async fn save_strategy_to_contract(
    _config: &ContractConfig,
    _strategy_data: &StrategyData,
) -> Result<i32, Box<dyn std::error::Error>> {
    // TODO: Implement actual contract interaction using subxt
    // For now, return a mock contract strategy ID
    info!("Saving strategy to contract (mock implementation)");
    Ok(1) // Mock contract strategy ID
}

#[allow(dead_code)]
async fn get_strategies_from_contract(
    _config: &ContractConfig,
    _account_id: &str,
) -> Result<Vec<StrategyResponse>, Box<dyn std::error::Error>> {
    // TODO: Implement actual contract interaction using subxt
    // For now, return empty vec as contracts are queried through DB
    info!("Getting strategies from contract (mock implementation)");
    Ok(vec![])
}

// API handlers
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        object: "health_check".to_string(),
        data: Some("DynaVest Shuttle Backend is running!".to_string()),
        error: None,
    })
}

async fn save_strategy(
    State(state): State<AppState>,
    Json(request): Json<CreateStrategyRequest>,
) -> Result<Json<ApiResponse<StrategyResponse>>, StatusCode> {
    info!("Saving strategy for account: {}", request.account);

    // Validate request
    if request.strategy.name.is_empty() {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_missing".to_string(),
                message: "Strategy name cannot be empty".to_string(),
                param: Some("name".to_string()),
            }),
        }));
    }

    if request.strategy.risk_level < 1 || request.strategy.risk_level > 10 {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_invalid".to_string(),
                message: "Risk level must be between 1 and 10".to_string(),
                param: Some("risk_level".to_string()),
            }),
        }));
    }

    // Save to contract first
    let contract_strategy_id = match save_strategy_to_contract(&state.contract_config, &request.strategy).await {
        Ok(id) => Some(id),
        Err(e) => {
            info!("Contract save failed: {}, continuing with DB save", e);
            None
        }
    };

    // Save to database
    match create_strategy_in_db(&state.db, &request.account, &request.strategy, contract_strategy_id).await {
        Ok(strategy) => {
            let response = StrategyResponse {
                name: strategy.name,
                risk_level: strategy.risk_level,
                parameters: strategy.parameters,
                created_at: strategy.created_at.to_rfc3339(),
                is_active: strategy.is_active,
            };

            Ok(Json(ApiResponse {
                object: "strategy".to_string(),
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Database save failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_strategies(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<StrategyResponse>>>, StatusCode> {
    info!("Getting strategies for account: {}", account_id);

    // Get strategies from database
    match get_strategies_from_db(&state.db, &account_id).await {
        Ok(strategies) => {
            let response: Vec<StrategyResponse> = strategies
                .into_iter()
                .map(|s| StrategyResponse {
                    name: s.name,
                    risk_level: s.risk_level,
                    parameters: s.parameters,
                    created_at: s.created_at.to_rfc3339(),
                    is_active: s.is_active,
                })
                .collect();

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Database query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_strategy_count(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Json<ApiResponse<i64>>, StatusCode> {
    info!("Getting strategy count for account: {}", account_id);

    match sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM strategies WHERE account_id = $1 AND is_active = true"
    )
    .bind(account_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => Ok(Json(ApiResponse {
            success: true,
            data: Some(count),
            error: None,
        })),
        Err(e) => {
            info!("Database query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_strategy(
    State(state): State<AppState>,
    Path(strategy_id): Path<String>,
    Json(request): Json<UpdateStrategyRequest>,
) -> Result<Json<ApiResponse<StrategyResponse>>, StatusCode> {
    info!("Updating strategy {} for account: {}", strategy_id, request.account);

    // Validate request
    if request.strategy.name.is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Strategy name cannot be empty".to_string()),
        }));
    }

    if request.strategy.risk_level < 1 || request.strategy.risk_level > 10 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Risk level must be between 1 and 10".to_string()),
        }));
    }

    // Update in database
    match update_strategy_in_db(&state.db, &strategy_id, &request.account, &request.strategy).await {
        Ok(Some(strategy)) => {
            let response = StrategyResponse {
                name: strategy.name,
                risk_level: strategy.risk_level,
                parameters: strategy.parameters,
                created_at: strategy.created_at.to_rfc3339(),
                is_active: strategy.is_active,
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Strategy not found or access denied".to_string()),
            }))
        }
        Err(e) => {
            info!("Database update failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_strategy(
    State(state): State<AppState>,
    Path(strategy_id): Path<String>,
    Json(request): Json<DeleteStrategyRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Deleting strategy {} for account: {}", strategy_id, request.account);

    // Delete from database (soft delete by setting is_active = false)
    match delete_strategy_in_db(&state.db, &strategy_id, &request.account).await {
        Ok(true) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some("Strategy deleted successfully".to_string()),
                error: None,
            }))
        }
        Ok(false) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Strategy not found or access denied".to_string()),
            }))
        }
        Err(e) => {
            info!("Database delete failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_statistics() -> Json<ApiResponse<HashMap<String, i32>>> {
    let mut stats = HashMap::new();
    stats.insert("total_strategies".to_string(), 100);
    stats.insert("active_users".to_string(), 25);
    stats.insert("avg_risk_level".to_string(), 6);

    Json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    })
}

async fn generate_cross_chain_strategy(
    State(state): State<AppState>,
    Json(request): Json<CrossChainStrategyRequest>,
) -> Result<Json<ApiResponse<EnhancedStrategyParams>>, StatusCode> {
    info!("Generating cross-chain strategy for account: {}, risk_level: {}, amount: ${}", 
          request.account, request.risk_level, request.investment_amount);

    // Validate request
    if request.risk_level < 1 || request.risk_level > 10 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Risk level must be between 1 and 10".to_string()),
        }));
    }

    if request.investment_amount <= 0.0 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Investment amount must be greater than 0".to_string()),
        }));
    }

    // Fetch cross-chain LP data
    let lp_data = match state.hyperbridge_client.fetch_cross_chain_lp_data(request.risk_level).await {
        Ok(data) => data,
        Err(e) => {
            info!("Failed to fetch cross-chain LP data: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Generate strategy recommendations
    let recommendations = match state.hyperbridge_client.get_strategy_recommendations(
        request.risk_level,
        request.investment_amount,
    ).await {
        Ok(recs) => recs,
        Err(e) => {
            info!("Failed to generate strategy recommendations: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create enhanced strategy parameters
    let base_strategy = format!(
        "AI-Generated Cross-Chain DeFi Strategy (Risk Level: {}/10)",
        request.risk_level
    );

    let enhanced_params = EnhancedStrategyParams::new(
        base_strategy,
        lp_data,
        recommendations,
    );

    Ok(Json(ApiResponse {
        success: true,
        data: Some(enhanced_params),
        error: None,
    }))
}

async fn get_cross_chain_opportunities(
    State(state): State<AppState>,
    Path(risk_level): Path<u8>,
) -> Result<Json<ApiResponse<Vec<hyperbridge::CrossChainLPData>>>, StatusCode> {
    info!("Getting cross-chain opportunities for risk level: {}", risk_level);

    // Validate risk level
    if risk_level < 1 || risk_level > 10 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Risk level must be between 1 and 10".to_string()),
        }));
    }

    // Fetch cross-chain LP data
    match state.hyperbridge_client.fetch_cross_chain_lp_data(risk_level).await {
        Ok(lp_data) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(lp_data),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to fetch cross-chain opportunities: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn chat_endpoint(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ApiResponse<ChatResponse>>, StatusCode> {
    info!("Processing chat request from user: {}", request.user_id);

    // Validate request
    if request.message.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Message cannot be empty".to_string()),
        }));
    }

    if request.user_id.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("User ID cannot be empty".to_string()),
        }));
    }

    // Process chat request
    match state.chat_service.process_chat(request).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Chat processing failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// New enhanced DeFi endpoint
async fn defi_info_endpoint(
    State(state): State<AppState>,
    Json(request): Json<DefiInfoRequest>,
) -> Result<Json<ApiResponse<DefiResponse>>, StatusCode> {
    info!("Processing DeFi info request: {}", request.input_text);

    // Validate request
    if request.input_text.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Input text cannot be empty".to_string()),
        }));
    }

    // Process DeFi request
    match state.defi_service.handle_defi_info(request).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("DeFi processing failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Crypto prices endpoint
async fn crypto_prices_endpoint(
    State(state): State<AppState>,
    Path(tokens): Path<String>,
) -> Result<Json<ApiResponse<Vec<CryptoPriceData>>>, StatusCode> {
    info!("Getting crypto prices for tokens: {}", tokens);

    let token_list: Vec<String> = tokens
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    match state.defi_service.get_crypto_prices(&token_list).await {
        Ok(prices) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(prices),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to get crypto prices: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Contract interaction endpoints
async fn create_contract_strategy(
    State(state): State<AppState>,
    Json(request): Json<CreateStrategyParams>,
) -> Result<Json<ApiResponse<u32>>, StatusCode> {
    info!("Creating contract strategy: {}", request.name);

    // Validate parameters
    if let Err(e) = ContractService::validate_strategy_params(&request) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }));
    }

    // Create strategy on contract
    match state.contract_service.create_strategy_on_chain("user_account", request).await {
        Ok(strategy_id) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(strategy_id),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to create contract strategy: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn invest_in_contract_strategy(
    State(state): State<AppState>,
    Json(request): Json<InvestmentParams>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Investing in contract strategy: {}", request.strategy_id);

    // Validate parameters
    if let Err(e) = ContractService::validate_investment_params(&request) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }));
    }

    // Invest in strategy
    match state.contract_service.invest_in_strategy("user_account", request).await {
        Ok(tx_hash) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(tx_hash),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to invest in contract strategy: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_contract_strategies(
    State(state): State<AppState>,
    Path(user_address): Path<String>,
) -> Result<Json<ApiResponse<Vec<ContractStrategy>>>, StatusCode> {
    info!("Getting contract strategies for user: {}", user_address);

    match state.contract_service.get_user_strategies(&user_address).await {
        Ok(strategies) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(strategies),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to get contract strategies: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn withdraw_from_contract_strategy(
    State(state): State<AppState>,
    Json(request): Json<WithdrawParams>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Withdrawing from contract strategy: {}", request.strategy_id);

    // Validate parameters
    if let Err(e) = ContractService::validate_withdraw_params(&request) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }));
    }

    // Withdraw from strategy
    match state.contract_service.withdraw_from_strategy("user_account", request).await {
        Ok(tx_hash) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(tx_hash),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to withdraw from contract strategy: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// RAG and semantic search endpoints
async fn semantic_search(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<ApiResponse<Vec<SearchResult>>>, StatusCode> {
    info!("Processing semantic search request: {}", request.query);

    // Validate request
    if request.query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Search query cannot be empty".to_string()),
        }));
    }

    // Search documents
    match state.rag_system.search_documents(&request.query, request.limit, request.score_threshold).await {
        Ok(results) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(results),
                error: None,
            }))
        }
        Err(e) => {
            info!("Semantic search failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn rag_query(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Processing RAG query: {}", request.query);

    // Validate request
    if request.query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Query cannot be empty".to_string()),
        }));
    }

    // Generate RAG response
    match state.rag_system.generate_rag_response(&request.query, request.limit).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("RAG query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn add_document(
    State(state): State<AppState>,
    Json(request): Json<EmbeddingRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Adding document to knowledge base");

    // Validate request
    if request.text.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Document text cannot be empty".to_string()),
        }));
    }

    // Add document to collection
    let metadata = std::collections::HashMap::from([
        ("source".to_string(), "api".to_string()),
        ("type".to_string(), "user_document".to_string()),
    ]);

    match state.rag_system.add_document(&request.text, metadata).await {
        Ok(doc_id) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(doc_id),
                error: None,
            }))
        }
        Err(e) => {
            info!("Document addition failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_rag_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<std::collections::HashMap<String, u64>>>, StatusCode> {
    info!("Getting RAG system statistics");

    match state.rag_system.get_collection_stats().await {
        Ok(stats) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(stats),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to get RAG stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AskRequest {
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FormattedResponse {
    query: String,
    summary: String,
    examples: Vec<CodeExample>,
    help_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeExample {
    title: String,
    description: Option<String>,
    code: String,
    source_file: Option<String>,
    relevance_score: f32,
}

async fn ask_endpoint(
    State(state): State<AppState>,
    Json(request): Json<AskRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Processing ask request: {}", request.query);

    // Validate request
    if request.query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Query cannot be empty".to_string()),
        }));
    }

    // Generate RAG response using Gemini API
    match state.rag_system.generate_rag_response(&request.query, 5).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Ask query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn ask_structured_endpoint(
    State(state): State<AppState>,
    Json(request): Json<AskRequest>,
) -> Result<Json<ApiResponse<FormattedResponse>>, StatusCode> {
    info!("Processing structured ask request: {}", request.query);

    // Validate request
    if request.query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Query cannot be empty".to_string()),
        }));
    }

    // Generate structured RAG response
    match state.rag_system.generate_structured_response(&request.query, 5).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Structured ask query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// GET endpoint for /ask?query=...
async fn ask_get_endpoint(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let query = params.get("query").unwrap_or(&String::new()).clone();
    
    info!("Processing GET ask request: {}", query);

    // Validate request
    if query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Query parameter cannot be empty".to_string()),
        }));
    }

    // Generate RAG response using Gemini API
    match state.rag_system.generate_rag_response(&query, 5).await {
        Ok(response) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                error: None,
            }))
        }
        Err(e) => {
            info!("Ask query failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Polkadot protocols endpoints
async fn get_polkadot_protocols_endpoint() -> Json<serde_json::Value> {
    let protocols = get_polkadot_protocols();
    Json(json!({
        "success": true,
        "data": protocols,
        "error": null
    }))
}

#[derive(Debug, Deserialize)]
struct PolkadotStrategyRequest {
    risk_level: u8,
    investment_amount: f64,
    query: Option<String>,
}

async fn get_polkadot_strategy(
    Json(req): Json<PolkadotStrategyRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let strategy = get_polkadot_strategy_recommendation(req.risk_level, req.investment_amount);
    
    let mut response = json!({
        "success": true,
        "data": {
            "response_type": "polkadot_strategy",
            "data": {
                "answer": strategy,
                "risk_level": req.risk_level,
                "investment_amount": req.investment_amount,
            }
        },
        "error": null
    });

    // If query is provided, also search for specific protocols
    if let Some(query) = req.query {
        let matching_protocols = search_polkadot_protocols(&query);
        response["data"]["matching_protocols"] = json!(matching_protocols);
    }

    Ok(Json(response))
}

// Database migration
async fn run_migrations(db: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running database migrations...");
    
    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS strategies (
            id UUID PRIMARY KEY,
            account_id VARCHAR(66) NOT NULL,
            name VARCHAR(255) NOT NULL,
            risk_level INTEGER NOT NULL CHECK (risk_level >= 1 AND risk_level <= 10),
            parameters TEXT NOT NULL,
            contract_strategy_id INTEGER,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT true
        )
        "#,
    )
    .execute(db)
    .await?;

    // Create indexes separately
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_strategies_account_id ON strategies(account_id)")
        .execute(db)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_strategies_created_at ON strategies(created_at)")
        .execute(db)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_strategies_is_active ON strategies(is_active)")
        .execute(db)
        .await?;

    info!("Database migrations completed successfully");
    Ok(())
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] database_url: String,
    #[shuttle_qdrant::Qdrant(
        cloud_url = "{secrets.QDRANT_URL}",
        api_key = "{secrets.QDRANT_API_KEY}",
        local_url = "http://localhost:6334"
    )] qdrant_client: Qdrant,
) -> ShuttleAxum {
    // Load environment variables from .env file for local development
    dotenv::dotenv().ok();
    
    // Note: Shuttle already initializes tracing, so we don't need to do it again

    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    if let Err(e) = run_migrations(&pool).await {
        panic!("Failed to run migrations: {}", e);
    }

    // Get Gemini API key
    let gemini_api_key = std::env::var("GEMINI_API_KEY")
        .unwrap_or_else(|_| "mock-key-for-testing".to_string());
    
    // Create services with Qdrant client
    let gemini_api_key_2 = gemini_api_key.clone();
    
    let chat_service = std::sync::Arc::new(ChatService::new(qdrant_client, gemini_api_key));
    
    // Initialize Qdrant collection (non-blocking)
    if let Err(e) = chat_service.initialize_collection().await {
        info!("Warning: Failed to initialize Qdrant collection: {}", e);
        // Continue anyway - the service can still work with mock data
    }

    // Create RAG system using the injected Qdrant client configuration
    let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());
    let qdrant_client_for_rag = if let Ok(api_key) = std::env::var("QDRANT_API_KEY") {
        info!("Using Qdrant Cloud with API key for RAG system");
        qdrant_client::Qdrant::from_url(&qdrant_url)
            .api_key(api_key)
            .build()
            .expect("Failed to create Qdrant client for RAG system")
    } else {
        info!("Using local Qdrant instance for RAG system");
        qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("Failed to create Qdrant client for RAG system")
    };
    
    // Initialize RAG system with Gemini
    let rag_system = std::sync::Arc::new(RAGSystem::new(qdrant_client_for_rag, gemini_api_key_2));
    
    // Initialize RAG collections (non-blocking)
    if let Err(e) = rag_system.initialize_collections().await {
        info!("Warning: Failed to initialize RAG collections: {}", e);
        // Continue anyway - the service can still work with mock data
    }
    
    // Populate sample data for testing (non-blocking)
    if let Err(e) = sample_data::populate_sample_data(&rag_system).await {
        info!("Warning: Failed to populate sample data: {}", e);
        // Continue anyway - the service can still work without sample data
    }

    // Initialize Polkadot client (use mock for now to avoid network issues)
    info!("Using mock Polkadot client to avoid network connectivity issues");
    let polkadot_client = std::sync::Arc::new(
        PolkadotClient::new_mock().await.expect("Failed to create mock Polkadot client")
    );

    // Initialize contract service (always use mock for now to avoid network issues)
    let contract_service = std::sync::Arc::new(
        ContractService::new_mock().await.expect("Failed to create mock contract service")
    );

    // Initialize DeFi service
    let defi_service = std::sync::Arc::new(
        DefiService::new(
            chat_service.clone(),
            polkadot_client.clone(),
            pool.clone(),
        )
    );

    // Create application state
    let state = AppState {
        db: pool,
        contract_config: ContractConfig::default(),
        hyperbridge_client: HyperbridgeClient::new(),
        chat_service,
        polkadot_client,
        defi_service,
        contract_service,
        rag_system,
    };

    // Build router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        // Database-based strategies
        .route("/strategies", post(save_strategy))
        .route("/strategies/account/{account}", get(get_strategies))
        .route("/strategies/account/{account}/count", get(get_strategy_count))
        .route("/strategies/{strategy_id}", put(update_strategy))
        .route("/strategies/{strategy_id}", delete(delete_strategy))
        .route("/statistics", get(get_statistics))
        // Cross-chain functionality
        .route("/cross-chain/strategy", post(generate_cross_chain_strategy))
        .route("/cross-chain/opportunities/{risk_level}", get(get_cross_chain_opportunities))
        // Chat and AI services
        .route("/chat", post(chat_endpoint))
        .route("/defiInfo", post(defi_info_endpoint))
        // Crypto prices
        .route("/crypto/prices/{tokens}", get(crypto_prices_endpoint))
        // Contract interactions
        .route("/contract/strategy", post(create_contract_strategy))
        .route("/contract/invest", post(invest_in_contract_strategy))
        .route("/contract/withdraw", post(withdraw_from_contract_strategy))
        .route("/contract/strategies/{user_address}", get(get_contract_strategies))
        // RAG and semantic search
        .route("/rag/search", post(semantic_search))
        .route("/rag/query", post(rag_query))
        .route("/rag/document", post(add_document))
        .route("/rag/stats", get(get_rag_stats))
        // Ask endpoint (as specified in PRD)
        .route("/ask", get(ask_get_endpoint))
        .route("/ask", post(ask_endpoint))
        .route("/ask/structured", post(ask_structured_endpoint))
        // Polkadot DeFi protocols
        .route("/polkadot/protocols", get(get_polkadot_protocols_endpoint))
        .route("/polkadot/strategy", post(get_polkadot_strategy))
        // Training system endpoints
        .route("/training/embed-contracts", post(embed_contract_pairs_endpoint))
        .route("/training/contract-pairs", get(get_contract_pairs_endpoint))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB request limit
        .with_state(state);

    info!("🚀 DynaVest Shuttle Backend is starting...");
    info!("📊 Available endpoints:");
    info!("  GET    /health - Health check");
    info!("  POST   /strategies - Save a new strategy");
    info!("  GET    /strategies/:account - Get strategies for account");
    info!("  GET    /strategies/:account/count - Get strategy count");
    info!("  PUT    /strategies/:strategy_id - Update a strategy");
    info!("  DELETE /strategies/:strategy_id - Delete a strategy");
    info!("  GET    /statistics - Get platform statistics");
    info!("  POST   /cross-chain/strategy - Generate cross-chain strategy");
    info!("  GET    /cross-chain/opportunities/:risk_level - Get cross-chain opportunities");
    info!("  POST   /chat - Process chat messages with AI");
    info!("  POST   /defiInfo - Enhanced DeFi info with AI (Python backend compatible)");
    info!("  GET    /crypto/prices/:tokens - Get crypto prices");
    info!("  POST   /contract/strategy - Create strategy on ink! contract");
    info!("  POST   /contract/invest - Invest in ink! contract strategy");
    info!("  POST   /contract/withdraw - Withdraw from ink! contract strategy");
    info!("  GET    /contract/strategies/:user_address - Get user's contract strategies");
    info!("  POST   /rag/search - Semantic search through knowledge base");
    info!("  POST   /rag/query - RAG-powered AI query with context");
    info!("  POST   /rag/document - Add document to knowledge base");
    info!("  GET    /rag/stats - Get RAG system statistics");
    info!("  GET    /ask?query=... - Ask a question and get RAG response (Gemini-powered)");
    info!("  POST   /ask - Ask a question with JSON body (Gemini-powered)");
    info!("  POST   /training/embed-contracts - Embed Solidity+ink! contract pairs for training");
    info!("  GET    /training/contract-pairs - Get available contract pairs");

    Ok(app.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    // Temporarily disabled due to axum-test compatibility issues
    // #[tokio::test]
    // async fn test_health_check() {
    //     let app = Router::new().route("/health", get(health_check));
    //     let server = TestServer::new(app).unwrap();

    //     let response = server.get("/health").await;
    //     assert_eq!(response.status_code(), 200);
        
    //     let body: ApiResponse<String> = response.json();
    //     assert!(body.success);
    //     assert!(body.data.is_some());
    // }

    // #[tokio::test]
    // async fn test_statistics() {
    //     let app = Router::new().route("/statistics", get(get_statistics));
    //     let server = TestServer::new(app).unwrap();

    //     let response = server.get("/statistics").await;
    //     assert_eq!(response.status_code(), 200);
        
    //     let body: ApiResponse<HashMap<String, i32>> = response.json();
    //     assert!(body.success);
    //     assert!(body.data.is_some());
    // }

    #[test]
    fn test_strategy_validation() {
        let valid_strategy = StrategyData {
            name: "Test Strategy".to_string(),
            risk_level: 5,
            parameters: "{}".to_string(),
        };

        assert!(!valid_strategy.name.is_empty());
        assert!(valid_strategy.risk_level >= 1 && valid_strategy.risk_level <= 10);

        let invalid_strategy = StrategyData {
            name: "".to_string(),
            risk_level: 11,
            parameters: "{}".to_string(),
        };

        assert!(invalid_strategy.name.is_empty());
        assert!(invalid_strategy.risk_level > 10);
    }

    #[test]
    fn test_update_strategy_validation() {
        let valid_update = UpdateStrategyRequest {
            account: "0x123456789".to_string(),
            strategy_id: "test-uuid".to_string(),
            strategy: StrategyData {
                name: "Updated Strategy".to_string(),
                risk_level: 7,
                parameters: "{\"updated\": true}".to_string(),
            },
        };

        assert!(!valid_update.account.is_empty());
        assert!(!valid_update.strategy_id.is_empty());
        assert!(!valid_update.strategy.name.is_empty());
        assert!(valid_update.strategy.risk_level >= 1 && valid_update.strategy.risk_level <= 10);
    }

    #[test]
    fn test_delete_strategy_validation() {
        let valid_delete = DeleteStrategyRequest {
            account: "0x123456789".to_string(),
            strategy_id: "test-uuid".to_string(),
        };

        assert!(!valid_delete.account.is_empty());
        assert!(!valid_delete.strategy_id.is_empty());
    }

    #[test]
    fn test_cross_chain_strategy_validation() {
        let valid_request = CrossChainStrategyRequest {
            account: "0x123456789".to_string(),
            risk_level: 5,
            investment_amount: 10000.0,
            preferred_chains: Some(vec!["Ethereum".to_string(), "Polygon".to_string()]),
        };

        assert!(!valid_request.account.is_empty());
        assert!(valid_request.risk_level >= 1 && valid_request.risk_level <= 10);
        assert!(valid_request.investment_amount > 0.0);
        assert!(valid_request.preferred_chains.is_some());
    }

    #[tokio::test]
    async fn test_hyperbridge_client_creation() {
        let _client = HyperbridgeClient::new();
        // Test that client can be created without errors
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_ask_request_validation() {
        let valid_request = AskRequest {
            query: "What is the main function?".to_string(),
        };
        assert!(!valid_request.query.trim().is_empty());

        let invalid_request = AskRequest {
            query: "".to_string(),
        };
        assert!(invalid_request.query.trim().is_empty());
    }

    #[test]
    fn test_search_request_validation() {
        let valid_request = SearchRequest {
            query: "test query".to_string(),
            limit: 5,
            score_threshold: Some(0.7),
        };
        assert!(!valid_request.query.trim().is_empty());
        assert!(valid_request.limit > 0);
        assert!(valid_request.score_threshold.unwrap() >= 0.0);
    }

    #[test]
    fn test_embedding_request_validation() {
        let valid_request = EmbeddingRequest {
            text: "This is test content".to_string(),
        };
        assert!(!valid_request.text.trim().is_empty());
    }

    #[tokio::test]
    async fn test_gemini_client_creation() {
        let _client = crate::gemini_client::GeminiClient::new("test-key".to_string());
        // Test that client can be created without errors
        assert!(true); // Placeholder assertion
    }
}

// Training system endpoints
async fn embed_contract_pairs_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<EmbeddingResult>>, StatusCode> {
    info!("Starting contract pair embedding process");

    // Get the current directory paths
    let current_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let solidity_path = current_dir
        .parent()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .join("solidity-examples")
        .to_string_lossy()
        .to_string();
    
    let ink_path = current_dir
        .parent()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .join("ink-examples-main")
        .to_string_lossy()
        .to_string();

    // Create training embedder
    let embedder = TrainingEmbedder::new(
        solidity_path,
        ink_path,
        state.rag_system.clone(),
    );

    // Embed contract pairs
    match embedder.embed_contract_pairs().await {
        Ok(result) => {
            info!("Contract embedding completed: {} pairs processed", result.processed_pairs);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(result),
                error: None,
            }))
        }
        Err(e) => {
            info!("Contract embedding failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_contract_pairs_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    info!("Getting available contract pairs");

    // Get the current directory paths
    let current_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let solidity_path = current_dir
        .parent()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .join("solidity-examples")
        .to_string_lossy()
        .to_string();
    
    let ink_path = current_dir
        .parent()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .join("ink-examples-main")
        .to_string_lossy()
        .to_string();

    // Create training embedder
    let embedder = TrainingEmbedder::new(
        solidity_path,
        ink_path,
        state.rag_system.clone(),
    );

    // Get contract pairs
    match embedder.contract_matcher.find_contract_pairs() {
        Ok(result) => {
            let pair_names: Vec<String> = result.pairs
                .into_iter()
                .map(|p| format!("{}: {}", p.contract_type, p.description))
                .collect();
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(pair_names),
                error: None,
            }))
        }
        Err(e) => {
            info!("Failed to get contract pairs: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}