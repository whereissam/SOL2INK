// Simple test server for Swagger API testing without database dependencies
use shuttle_axum::axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use uuid::Uuid;
use utoipa::{OpenApi, ToSchema};
// Removed SwaggerUI due to compatibility issues - serving raw OpenAPI spec instead

// API Models
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ApiResponse<T> {
    pub object: String,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ApiError {
    pub error_type: String,
    pub code: String,
    pub message: String,
    pub param: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
struct CreateStrategyRequest {
    pub account: String,
    pub strategy: StrategyData,
}

#[derive(Debug, Deserialize, ToSchema)]
struct StrategyData {
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct StrategyResponse {
    pub id: String,
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
    pub created_at: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
struct ChatRequest {
    pub message: String,
    pub user_id: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
struct ChatResponse {
    pub message: String,
    pub keywords: Vec<String>,
    pub ui_suggestions: Vec<String>,
    pub session_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
struct AskRequest {
    pub query: String,
}

// OpenAPI specification
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        save_strategy,
        get_strategies,
        get_strategy_count,
        chat_endpoint,
        ask_endpoint,
        ask_get_endpoint,
        get_statistics
    ),
    components(
        schemas(
            ApiResponse<String>,
            ApiResponse<StrategyResponse>,
            ApiResponse<Vec<StrategyResponse>>,
            ApiResponse<i64>,
            ApiResponse<ChatResponse>,
            ApiError,
            CreateStrategyRequest,
            StrategyData,
            StrategyResponse,
            ChatRequest,
            ChatResponse,
            AskRequest
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "strategies", description = "Strategy management endpoints"),
        (name = "chat", description = "Chat and AI endpoints"),
        (name = "rag", description = "RAG and semantic search endpoints")
    ),
    info(
        title = "DynaVest Backend API",
        version = "1.0.0",
        description = "API for DynaVest - AI-powered investment strategy platform with Swagger documentation"
    )
)]
struct ApiDoc;

// API Handlers
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = ApiResponse<String>)
    )
)]
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        object: "health_check".to_string(),
        success: true,
        data: Some("✅ DynaVest Backend with Swagger is running perfectly!".to_string()),
        error: None,
    })
}

#[utoipa::path(
    post,
    path = "/strategies",
    tag = "strategies",
    request_body = CreateStrategyRequest,
    responses(
        (status = 200, description = "Strategy created successfully", body = ApiResponse<StrategyResponse>),
        (status = 400, description = "Invalid request")
    )
)]
async fn save_strategy(
    Json(request): Json<CreateStrategyRequest>,
) -> Result<Json<ApiResponse<StrategyResponse>>, StatusCode> {
    // Validation
    if request.strategy.name.is_empty() {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            success: false,
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
            success: false,
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_invalid".to_string(),
                message: "Risk level must be between 1 and 10".to_string(),
                param: Some("risk_level".to_string()),
            }),
        }));
    }

    // Mock response
    let response = StrategyResponse {
        id: Uuid::new_v4().to_string(),
        name: request.strategy.name,
        risk_level: request.strategy.risk_level,
        parameters: request.strategy.parameters,
        created_at: chrono::Utc::now().to_rfc3339(),
        is_active: true,
    };

    Ok(Json(ApiResponse {
        object: "strategy".to_string(),
        success: true,
        data: Some(response),
        error: None,
    }))
}

#[utoipa::path(
    get,
    path = "/strategies/account/{account}",
    tag = "strategies",
    params(
        ("account" = String, Path, description = "Account ID to get strategies for")
    ),
    responses(
        (status = 200, description = "Strategies retrieved successfully", body = ApiResponse<Vec<StrategyResponse>>)
    )
)]
async fn get_strategies(
    Path(account_id): Path<String>,
) -> Json<ApiResponse<Vec<StrategyResponse>>> {
    // Mock response with sample strategies
    let strategies = vec![
        StrategyResponse {
            id: Uuid::new_v4().to_string(),
            name: "Conservative DeFi Strategy".to_string(),
            risk_level: 3,
            parameters: r#"{"protocols": ["Aave", "Compound"], "allocation": {"stablecoins": 0.8, "eth": 0.2}}"#.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            is_active: true,
        },
        StrategyResponse {
            id: Uuid::new_v4().to_string(),
            name: "Aggressive Yield Farming".to_string(),
            risk_level: 8,
            parameters: r#"{"protocols": ["Uniswap", "SushiSwap"], "allocation": {"LP_tokens": 0.9, "governance": 0.1}}"#.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            is_active: true,
        },
    ];

    Json(ApiResponse {
        object: "strategies_list".to_string(),
        success: true,
        data: Some(strategies),
        error: None,
    })
}

#[utoipa::path(
    get,
    path = "/strategies/account/{account}/count",
    tag = "strategies",
    params(
        ("account" = String, Path, description = "Account ID to get strategy count for")
    ),
    responses(
        (status = 200, description = "Strategy count retrieved successfully", body = ApiResponse<i64>)
    )
)]
async fn get_strategy_count(
    Path(_account_id): Path<String>,
) -> Json<ApiResponse<i64>> {
    Json(ApiResponse {
        object: "count".to_string(),
        success: true,
        data: Some(2),
        error: None,
    })
}

#[utoipa::path(
    post,
    path = "/chat",
    tag = "chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Chat processed successfully", body = ApiResponse<ChatResponse>)
    )
)]
async fn chat_endpoint(
    Json(request): Json<ChatRequest>,
) -> Result<Json<ApiResponse<ChatResponse>>, StatusCode> {
    if request.message.trim().is_empty() {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            success: false,
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_missing".to_string(),
                message: "Message cannot be empty".to_string(),
                param: Some("message".to_string()),
            }),
        }));
    }

    // Mock AI response
    let response = ChatResponse {
        message: format!("🤖 AI Response: I understand you're asking about '{}'. For DeFi strategies, I recommend starting with conservative approaches like stablecoin lending on Aave or Compound. Consider your risk tolerance and always diversify your portfolio.", request.message),
        keywords: vec!["DeFi".to_string(), "strategy".to_string(), "risk".to_string()],
        ui_suggestions: vec!["Show risk calculator".to_string(), "Display strategy templates".to_string()],
        session_id: request.session_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
    };

    Ok(Json(ApiResponse {
        object: "chat_response".to_string(),
        success: true,
        data: Some(response),
        error: None,
    }))
}

#[utoipa::path(
    post,
    path = "/ask",
    tag = "rag",
    request_body = AskRequest,
    responses(
        (status = 200, description = "Question answered successfully", body = ApiResponse<String>)
    )
)]
async fn ask_endpoint(
    Json(request): Json<AskRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    if request.query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            success: false,
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_missing".to_string(),
                message: "Query cannot be empty".to_string(),
                param: Some("query".to_string()),
            }),
        }));
    }

    // Mock RAG response
    let answer = format!(
        "📚 RAG Response: Based on the knowledge base, here's what I found about '{}': \n\n🔹 Cross-chain strategies involve deploying assets across multiple blockchains\n🔹 Key protocols: Polkadot, Cosmos, Ethereum Layer 2s\n🔹 Risk considerations: Bridge security, liquidity fragmentation\n🔹 Recommended allocation: Start with 10-20% of portfolio",
        request.query
    );

    Ok(Json(ApiResponse {
        object: "ask_response".to_string(),
        success: true,
        data: Some(answer),
        error: None,
    }))
}

#[utoipa::path(
    get,
    path = "/ask",
    tag = "rag", 
    params(
        ("query" = String, Query, description = "Question to ask the RAG system")
    ),
    responses(
        (status = 200, description = "Question answered successfully", body = ApiResponse<String>)
    )
)]
async fn ask_get_endpoint(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let query = params.get("query").unwrap_or(&String::new()).clone();
    
    if query.trim().is_empty() {
        return Ok(Json(ApiResponse {
            object: "error".to_string(),
            success: false,
            data: None,
            error: Some(ApiError {
                error_type: "invalid_request_error".to_string(),
                code: "parameter_missing".to_string(),
                message: "Query parameter cannot be empty".to_string(),
                param: Some("query".to_string()),
            }),
        }));
    }

    let answer = format!("📚 GET RAG Response for: '{}'", query);

    Ok(Json(ApiResponse {
        object: "ask_response".to_string(),
        success: true,
        data: Some(answer),
        error: None,
    }))
}

#[utoipa::path(
    get,
    path = "/statistics",
    tag = "strategies",
    responses(
        (status = 200, description = "Statistics retrieved successfully")
    )
)]
async fn get_statistics() -> Json<ApiResponse<HashMap<String, i32>>> {
    let mut stats = HashMap::new();
    stats.insert("total_strategies".to_string(), 156);
    stats.insert("active_users".to_string(), 47);
    stats.insert("avg_risk_level".to_string(), 6);

    Json(ApiResponse {
        object: "statistics".to_string(),
        success: true,
        data: Some(stats),
        error: None,
    })
}

async fn serve_openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api-docs/openapi.json", get(serve_openapi_spec))
        .route("/health", get(health_check))
        .route("/strategies", post(save_strategy))
        .route("/strategies/account/{account}", get(get_strategies))
        .route("/strategies/account/{account}/count", get(get_strategy_count))
        .route("/chat", post(chat_endpoint))
        .route("/ask", post(ask_endpoint))
        .route("/ask", get(ask_get_endpoint))
        .route("/statistics", get(get_statistics))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)));

    println!("🚀 DynaVest Test Server starting...");
    println!("🔗 OpenAPI Spec: http://localhost:3000/api-docs/openapi.json");
    println!("✅ Health Check: http://localhost:3000/health");
    println!("📊 Statistics: http://localhost:3000/statistics");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    shuttle_axum::axum::serve(listener, app).await.unwrap();
}