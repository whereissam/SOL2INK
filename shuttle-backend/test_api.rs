// Simple API test without database dependencies
use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("DynaVest Backend API Test - All endpoints working! 🚀".to_string()),
        error: None,
    })
}

async fn get_statistics() -> Json<ApiResponse<HashMap<String, i32>>> {
    let mut stats = HashMap::new();
    stats.insert("total_strategies".to_string(), 42);
    stats.insert("active_users".to_string(), 15);
    stats.insert("avg_risk_level".to_string(), 6);

    Json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    })
}

async fn mock_strategies() -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let mock_strategies = vec![
        serde_json::json!({
            "name": "Conservative Portfolio",
            "risk_level": 3,
            "parameters": "{\"allocation\": {\"BTC\": 40, \"ETH\": 30, \"USDC\": 30}}",
            "created_at": "2024-01-15T10:30:00Z",
            "is_active": true
        }),
        serde_json::json!({
            "name": "Aggressive DeFi",
            "risk_level": 8,
            "parameters": "{\"allocation\": {\"DOT\": 50, \"AAVE\": 30, \"UNI\": 20}}",
            "created_at": "2024-01-14T15:45:00Z",
            "is_active": true
        })
    ];

    Json(ApiResponse {
        success: true,
        data: Some(mock_strategies),
        error: None,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/statistics", get(get_statistics))
        .route("/strategies/mock", get(mock_strategies))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let port = 8000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    println!("🚀 DynaVest API Test Server running on http://localhost:{}", port);
    println!("📊 Available test endpoints:");
    println!("  - GET  /health - Health check");
    println!("  - GET  /statistics - Platform statistics");
    println!("  - GET  /strategies/mock - Mock strategy data");
    println!("\n💡 Test the endpoints:");
    println!("  curl http://localhost:{}/health", port);
    println!("  curl http://localhost:{}/statistics", port);
    println!("  curl http://localhost:{}/strategies/mock", port);

    axum::serve(listener, app).await?;
    Ok(())
}