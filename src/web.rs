// ============================================================================
// WEB MODULE: HTTP API Server
// ============================================================================
// This module provides a REST API for the coin combinations functionality

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{generate_all_combinations, generate_random_combination, total_value, Coin};

// ============================================================================
// Response Structures
// ============================================================================

/// Response for /random endpoint
#[derive(Serialize)]
pub struct RandomResponse {
    pub coins: Vec<Coin>,
    pub value: u32,
}

/// Response for /all endpoint
#[derive(Serialize)]
pub struct AllCombinationsResponse {
    pub total_combinations: usize,
    pub combinations: Vec<CombinationDetail>,
}

/// Details of a single combination
#[derive(Serialize)]
pub struct CombinationDetail {
    pub index: usize,
    pub coins: Vec<Coin>,
    pub value: u32,
}

/// Response for /health endpoint
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

// ============================================================================
// Application State
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    // Could add database connections, cache, etc. here in the future
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// GET /random - Returns a random coin combination
async fn get_random_combination() -> impl IntoResponse {
    let combination = generate_random_combination();
    let value = total_value(&combination);

    let response = RandomResponse {
        coins: combination,
        value,
    };

    (StatusCode::OK, Json(response))
}

/// GET /all - Returns all possible coin combinations
async fn get_all_combinations() -> impl IntoResponse {
    let all_combinations = generate_all_combinations();

    let combinations: Vec<CombinationDetail> = all_combinations
        .iter()
        .enumerate()
        .map(|(index, coins)| CombinationDetail {
            index,
            coins: coins.clone(),
            value: total_value(coins),
        })
        .collect();

    let response = AllCombinationsResponse {
        total_combinations: combinations.len(),
        combinations,
    };

    (StatusCode::OK, Json(response))
}

/// GET /health - Health check endpoint
async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "coins-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    (StatusCode::OK, Json(response))
}

/// GET / - Root endpoint with API information
async fn root() -> impl IntoResponse {
    let info = serde_json::json!({
        "service": "Coin Combinations API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "/": "API information",
            "/health": "Health check",
            "/random": "Get a random coin combination",
            "/all": "Get all possible coin combinations (16 total)"
        }
    });

    (StatusCode::OK, Json(info))
}

// ============================================================================
// Router Configuration
// ============================================================================

/// Creates and configures the Axum router with all endpoints
pub fn create_router() -> Router {
    let state = Arc::new(AppState {});

    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/random", get(get_random_combination))
        .route("/all", get(get_all_combinations))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

// ============================================================================
// Server Launch
// ============================================================================

/// Starts the HTTP server on the specified address
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Coin Combinations API server");
    info!("Listening on http://{}", addr);
    info!("Endpoints:");
    info!("  GET /        - API information");
    info!("  GET /health  - Health check");
    info!("  GET /random  - Random coin combination");
    info!("  GET /all     - All combinations");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
