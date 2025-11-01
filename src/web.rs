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

// ============================================================================
// TESTS MODULE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    /// Helper function to convert response body to JSON string
    async fn body_to_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    // ========================================================================
    // Response Structure Tests
    // ========================================================================

    #[test]
    fn test_random_response_serialization() {
        let response = RandomResponse {
            coins: vec![Coin::Penny, Coin::Nickel],
            value: 6,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"coins\""));
        assert!(json.contains("\"value\""));
        assert!(json.contains("Penny"));
        assert!(json.contains("Nickel"));
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            service: "coins-api".to_string(),
            version: "0.1.0".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"service\":\"coins-api\""));
        assert!(json.contains("\"version\":\"0.1.0\""));
    }

    #[test]
    fn test_combination_detail_serialization() {
        let detail = CombinationDetail {
            index: 5,
            coins: vec![Coin::Penny, Coin::Dime],
            value: 11,
        };

        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("\"index\":5"));
        assert!(json.contains("\"value\":11"));
        assert!(json.contains("Penny"));
        assert!(json.contains("Dime"));
    }

    #[test]
    fn test_all_combinations_response_structure() {
        let response = AllCombinationsResponse {
            total_combinations: 2,
            combinations: vec![
                CombinationDetail {
                    index: 0,
                    coins: vec![],
                    value: 0,
                },
                CombinationDetail {
                    index: 1,
                    coins: vec![Coin::Penny],
                    value: 1,
                },
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"total_combinations\":2"));
        assert!(json.contains("\"combinations\""));
    }

    // ========================================================================
    // HTTP Endpoint Tests
    // ========================================================================

    #[tokio::test]
    async fn test_root_endpoint() {
        let app = create_router();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = body_to_json(response.into_body()).await;
        assert_eq!(body["service"], "Coin Combinations API");
        assert!(body["endpoints"].is_object());
        assert_eq!(body["endpoints"]["/"], "API information");
        assert_eq!(body["endpoints"]["/health"], "Health check");
        assert_eq!(
            body["endpoints"]["/random"],
            "Get a random coin combination"
        );
        assert_eq!(
            body["endpoints"]["/all"],
            "Get all possible coin combinations (16 total)"
        );
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = body_to_json(response.into_body()).await;
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "coins-api");
        assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_random_endpoint_returns_valid_response() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/random")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = body_to_json(response.into_body()).await;
        assert!(body["coins"].is_array());
        assert!(body["value"].is_number());

        let value = body["value"].as_u64().unwrap();
        assert!(value <= 41, "Value should be at most 41 cents");
    }

    #[tokio::test]
    async fn test_random_endpoint_produces_variety() {
        let app = create_router();

        let mut results = Vec::new();
        for _ in 0..10 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/random")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            let body = body_to_json(response.into_body()).await;
            results.push(format!("{:?}", body["coins"]));
        }

        // With 10 attempts, we should get at least 2 different results
        results.sort();
        results.dedup();
        assert!(results.len() >= 2, "Random endpoint should produce variety");
    }

    #[tokio::test]
    async fn test_all_endpoint_returns_all_combinations() {
        let app = create_router();

        let response = app
            .oneshot(Request::builder().uri("/all").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = body_to_json(response.into_body()).await;
        assert_eq!(body["total_combinations"], 16);
        assert!(body["combinations"].is_array());

        let combinations = body["combinations"].as_array().unwrap();
        assert_eq!(combinations.len(), 16);

        // Check first combination (empty set)
        assert_eq!(combinations[0]["index"], 0);
        assert_eq!(combinations[0]["value"], 0);
        assert_eq!(combinations[0]["coins"].as_array().unwrap().len(), 0);

        // Check last combination (all coins)
        assert_eq!(combinations[15]["index"], 15);
        assert_eq!(combinations[15]["value"], 41);
        assert_eq!(combinations[15]["coins"].as_array().unwrap().len(), 4);
    }

    #[tokio::test]
    async fn test_all_endpoint_combination_values_are_correct() {
        let app = create_router();

        let response = app
            .oneshot(Request::builder().uri("/all").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = body_to_json(response.into_body()).await;
        let combinations = body["combinations"].as_array().unwrap();

        // Verify specific combinations
        // Combination 1: Penny only (value = 1)
        assert_eq!(combinations[1]["value"], 1);

        // Combination 2: Nickel only (value = 5)
        assert_eq!(combinations[2]["value"], 5);

        // Combination 4: Dime only (value = 10)
        assert_eq!(combinations[4]["value"], 10);

        // Combination 8: Quarter only (value = 25)
        assert_eq!(combinations[8]["value"], 25);

        // Combination 3: Penny + Nickel (value = 6)
        assert_eq!(combinations[3]["value"], 6);

        // Combination 5: Penny + Dime (value = 11)
        assert_eq!(combinations[5]["value"], 11);
    }

    #[tokio::test]
    async fn test_invalid_route_returns_404() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // ========================================================================
    // Router Configuration Tests
    // ========================================================================

    #[test]
    fn test_router_creation() {
        let router = create_router();
        // If router creation succeeds, this test passes
        // This ensures all routes are configured without panicking
        assert!(std::mem::size_of_val(&router) > 0);
    }

    #[test]
    fn test_app_state_creation() {
        let state = AppState {};
        let cloned = state.clone();
        // Verify Clone trait works
        assert_eq!(
            std::mem::size_of_val(&state),
            std::mem::size_of_val(&cloned)
        );
    }

    // ========================================================================
    // Content-Type Header Tests
    // ========================================================================

    #[tokio::test]
    async fn test_response_content_type_is_json() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let content_type = response.headers().get("content-type").unwrap();
        assert!(
            content_type.to_str().unwrap().contains("application/json"),
            "Response should have JSON content-type"
        );
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[tokio::test]
    async fn test_multiple_requests_to_same_endpoint() {
        let app = create_router();

        // Make 5 requests to /health
        for _ in 0..5 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/health")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_random_endpoint_value_matches_coins() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/random")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_to_json(response.into_body()).await;
        let coins_array = body["coins"].as_array().unwrap();
        let value = body["value"].as_u64().unwrap() as u32;

        // Calculate expected value from coins
        let mut expected_value = 0u32;
        for coin in coins_array {
            let coin_name = coin.as_str().unwrap();
            expected_value += match coin_name {
                "Penny" => 1,
                "Nickel" => 5,
                "Dime" => 10,
                "Quarter" => 25,
                _ => 0,
            };
        }

        assert_eq!(
            value, expected_value,
            "Value should match sum of coin values"
        );
    }
}
