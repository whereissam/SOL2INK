// Standalone version of DynaVest backend for Docker deployment
// Alternative to Shuttle.dev for immediate deployment

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;
use uuid::Uuid;
use std::env;

// Database models (reuse from main.rs)
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

// API request/response models
#[derive(Debug, Deserialize)]
struct CreateStrategyRequest {
    pub account: String,
    pub strategy: StrategyData,
}

#[derive(Debug, Deserialize)]
struct StrategyData {
    pub name: String,
    pub risk_level: i32,
    pub parameters: String,
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
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// Application state
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// Database functions
async fn create_strategy_in_db(
    db: &PgPool,
    account_id: &str,
    strategy_data: &StrategyData,
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
    .bind(None::<i32>) // contract_strategy_id
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

// API handlers
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("DynaVest Backend is running! 🚀".to_string()),
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

    // Save to database
    match create_strategy_in_db(&state.db, &request.account, &request.strategy).await {
        Ok(strategy) => {
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

// Database migration
async fn run_migrations(db: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running database migrations...");
    
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
        );
        
        CREATE INDEX IF NOT EXISTS idx_strategies_account_id ON strategies(account_id);
        CREATE INDEX IF NOT EXISTS idx_strategies_created_at ON strategies(created_at);
        CREATE INDEX IF NOT EXISTS idx_strategies_is_active ON strategies(is_active);
        "#,
    )
    .execute(db)
    .await?;

    info!("Database migrations completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/dynavest".to_string());

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    // Run migrations
    if let Err(e) = run_migrations(&pool).await {
        panic!("Failed to run migrations: {}", e);
    }

    // Create application state
    let state = AppState { db: pool };

    // Build router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/strategies", post(save_strategy))
        .route("/strategies/:account", get(get_strategies))
        .route("/statistics", get(get_statistics))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB request limit
        .with_state(state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    info!("🚀 DynaVest Backend is starting on port {}...", port);
    info!("📊 Available endpoints:");
    info!("  - GET  /health - Health check");
    info!("  - POST /strategies - Save strategy");
    info!("  - GET  /strategies/:account - Get strategies");
    info!("  - GET  /statistics - Platform stats");

    axum::serve(listener, app).await?;

    Ok(())
}