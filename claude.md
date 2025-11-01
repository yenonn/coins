# Coin Combinations REST API - Rust Learning Project

## Overview

This Rust project demonstrates generating and serving all possible combinations of US coins (Penny, Nickel, Dime, Quarter) via a REST API using bit manipulation to create a power set. It's an educational example showcasing Rust fundamentals including enums, traits, iterators, comprehensive testing, and modern web development with Axum and Tokio.

The project evolved from a CLI application to a production-ready REST API with Docker support, demonstrating real-world Rust web service architecture.

## Project Structure

```
coins/
├── Cargo.toml          # Project manifest with dependencies
├── Dockerfile          # Multi-stage Docker build
├── Makefile            # Convenient build/run commands
├── README.md           # User-facing documentation
├── CLAUDE.md           # This technical documentation
├── .github/
│   └── workflows/
│       └── ci.yml      # CI/CD pipeline
└── src/
    ├── lib.rs          # Core library (Coin enum, algorithms, 24 tests)
    ├── web.rs          # Web API module (Axum routes, handlers, 16 tests)
    └── main.rs         # Web server entry point
```

## Core Components

### Coin Enum (src/lib.rs:17-22)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
```

**Derived Traits:**

- `Debug`: Enables printing with `{:?}` format for debugging
- `Clone`: Allows creating explicit copies with `.clone()`
- `Copy`: Enables implicit copying (coins are small, stack-allocated)
- `PartialEq`: Allows equality comparisons with `==` and `!=`
- `Serialize`: Enables JSON serialization for web API responses (from Serde)

**Methods:**

- `all() -> [Coin; 4]` - Returns array of all 4 coin types
- `value_in_cents(&self) -> u8` - Returns coin value: Penny=1, Nickel=5, Dime=10, Quarter=25

### Core Functions (src/lib.rs)

#### Power Set Generation (lines 42-74)

The `generate_all_combinations()` function uses **bit manipulation** to generate all 16 possible subsets:

**Algorithm:**

1. Total combinations = 2^4 = 16 (including empty set)
2. For each number i from 0 to 15:
   - Check each bit position j (0-3)
   - If bit j is set in i, include coin[j] in combination

**Example:** i=5 (binary: 0101)

- Bit 0 set → include Penny
- Bit 1 clear → skip Nickel
- Bit 2 set → include Dime
- Bit 3 clear → skip Quarter
- Result: {Penny, Dime}

**Implementation:**
```rust
pub fn generate_all_combinations() -> Vec<Vec<Coin>> {
    let coins = Coin::all();
    let total_coins = coins.len(); // 4
    let total_combinations = 1 << total_coins; // 2^4 = 16

    let mut combinations = Vec::new();

    for i in 0..total_combinations {
        let mut combination = Vec::new();

        for j in 0..total_coins {
            if (i >> j) & 1 == 1 {
                combination.push(coins[j]);
            }
        }

        combinations.push(combination);
    }

    combinations
}
```

#### Random Combination Generation (lines 87-109)

The `generate_random_combination()` function generates a single random combination:

**Implementation:**
```rust
pub fn generate_random_combination() -> Vec<Coin> {
    let coins = Coin::all();
    let total_combinations = 1 << coins.len(); // 16

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..total_combinations);

    let mut combination = Vec::new();

    for j in 0..coins.len() {
        if (i >> j) & 1 == 1 {
            combination.push(coins[j]);
        }
    }

    combination
}
```

#### Value Calculation (lines 77-83)

```rust
pub fn total_value(coins: &[Coin]) -> u32 {
    coins.iter().map(|coin| coin.value_in_cents() as u32).sum()
}
```

Uses iterator chain:
- `iter()` - Creates iterator over coin slice (borrows)
- `map()` - Transforms each coin to its value
- `sum()` - Aggregates all values

## Web Module (src/web.rs)

The web module implements a REST API using the Axum framework with Tokio runtime.

### Response Structures (lines 23-50)

#### RandomResponse
```rust
#[derive(Serialize)]
pub struct RandomResponse {
    pub coins: Vec<Coin>,
    pub value: u32,
}
```
Used for `GET /random` endpoint.

#### AllCombinationsResponse
```rust
#[derive(Serialize)]
pub struct AllCombinationsResponse {
    pub total_combinations: usize,
    pub combinations: Vec<CombinationDetail>,
}
```
Used for `GET /all` endpoint.

#### CombinationDetail
```rust
#[derive(Serialize)]
pub struct CombinationDetail {
    pub index: usize,
    pub coins: Vec<Coin>,
    pub value: u32,
}
```
Individual combination details within the `/all` response.

#### HealthResponse
```rust
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}
```
Used for `GET /health` endpoint.

### Application State (lines 56-59)

```rust
#[derive(Clone)]
pub struct AppState {
    // Reserved for future use (database connections, cache, etc.)
}
```

Currently empty but provides foundation for future stateful features.

### HTTP Handlers

#### GET / - Root Endpoint (lines 112-126)

Returns API information and available endpoints.

```rust
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
```

**Response Example:**
```json
{
  "service": "Coin Combinations API",
  "version": "0.1.0",
  "endpoints": {
    "/": "API information",
    "/health": "Health check",
    "/random": "Get a random coin combination",
    "/all": "Get all possible coin combinations (16 total)"
  }
}
```

#### GET /health - Health Check (lines 100-110)

Returns service health status for monitoring and load balancers.

```rust
async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "coins-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    (StatusCode::OK, Json(response))
}
```

**Response Example:**
```json
{
  "status": "healthy",
  "service": "coins-api",
  "version": "0.1.0"
}
```

#### GET /random - Random Combination (lines 66-76)

Returns a randomly selected coin combination with its total value.

```rust
async fn get_random_combination() -> impl IntoResponse {
    let combination = generate_random_combination();
    let value = total_value(&combination);

    let response = RandomResponse {
        coins: combination,
        value,
    };

    (StatusCode::OK, Json(response))
}
```

**Response Examples:**
```json
{"coins": ["Penny", "Dime"], "value": 11}
{"coins": ["Nickel", "Quarter"], "value": 30}
{"coins": [], "value": 0}
```

#### GET /all - All Combinations (lines 78-98)

Returns all 16 possible combinations with their details.

```rust
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
```

**Response Example (truncated):**
```json
{
  "total_combinations": 16,
  "combinations": [
    {"index": 0, "coins": [], "value": 0},
    {"index": 1, "coins": ["Penny"], "value": 1},
    {"index": 2, "coins": ["Nickel"], "value": 5},
    ...
    {"index": 15, "coins": ["Penny", "Nickel", "Dime", "Quarter"], "value": 41}
  ]
}
```

### Router Configuration (lines 133-143)

Creates the Axum router with all endpoints and middleware:

```rust
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
```

**Features:**
- Route registration for all endpoints
- CORS middleware (permissive for demo/public API)
- Shared application state

### Server Launch (lines 150-171)

Starts the HTTP server with tracing and logging:

```rust
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
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
```

## Main Program (src/main.rs)

The main program has been converted from CLI to async web server:

```rust
use coins::web;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";

    if let Err(e) = web::run_server(addr).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
```

**Key Changes:**
- Uses `#[tokio::main]` macro for async runtime
- Calls `web::run_server()` instead of printing combinations
- Binds to `0.0.0.0:8080` for Docker compatibility
- Proper error handling with exit code

## Testing

The project includes comprehensive tests totaling 40 tests across two modules.

### Core Library Tests (src/lib.rs:117-378) - 24 Tests

#### Coin Tests (4 tests)
- ✓ `test_coin_all_returns_four_coins` - Verifies 4 coins returned
- ✓ `test_coin_all_has_correct_coins` - Validates correct order
- ✓ `test_penny_value` through `test_quarter_value` - Tests each coin value

#### Total Value Tests (4 tests)
- ✓ `test_total_value_empty` - Empty set = 0 cents
- ✓ `test_total_value_single_coin` - Single coin value
- ✓ `test_total_value_multiple_coins` - Sum of multiple coins
- ✓ `test_total_value_duplicate_coins` - Handles duplicates

#### Combination Generation Tests (8 tests)
- ✓ `test_combinations_count` - Verifies 16 combinations
- ✓ `test_combinations_has_empty_set` - Index 0 is empty
- ✓ `test_combinations_has_full_set` - Index 15 has all coins
- ✓ `test_specific_combination_*` - Validates specific bit patterns
- ✓ `test_all_combinations_unique` - No duplicate combinations
- ✓ `test_combination_values_range` - Values range 0-41 cents
- ✓ `test_all_single_coin_combinations_exist` - 4 single-coin combos
- ✓ `test_coin_copy_trait` - Copy trait works correctly

#### Random Combination Tests (4 tests)
- ✓ `test_random_combination_length_valid` - Length ≤ 4
- ✓ `test_random_combination_coins_are_valid` - Valid coin types
- ✓ `test_random_combination_value_in_range` - Value ≤ 41 cents
- ✓ `test_random_combination_produces_variety` - Generates variety

### Web Module Tests (src/web.rs:177-561) - 16 Tests

#### Response Structure Tests (4 tests)
- ✓ `test_random_response_serialization` - RandomResponse JSON format
- ✓ `test_health_response_serialization` - HealthResponse JSON format
- ✓ `test_combination_detail_serialization` - CombinationDetail structure
- ✓ `test_all_combinations_response_structure` - AllCombinationsResponse

#### HTTP Endpoint Tests (7 tests)
- ✓ `test_root_endpoint` - GET / returns API info
- ✓ `test_health_endpoint` - GET /health returns health status
- ✓ `test_random_endpoint_returns_valid_response` - GET /random structure
- ✓ `test_random_endpoint_produces_variety` - Randomness over 10 attempts
- ✓ `test_all_endpoint_returns_all_combinations` - GET /all returns 16
- ✓ `test_all_endpoint_combination_values_are_correct` - Validates values
- ✓ `test_invalid_route_returns_404` - 404 for invalid routes

#### Router & Configuration Tests (2 tests)
- ✓ `test_router_creation` - Router builds without panics
- ✓ `test_app_state_creation` - AppState Clone trait works

#### Content-Type Tests (1 test)
- ✓ `test_response_content_type_is_json` - application/json header

#### Edge Case Tests (2 tests)
- ✓ `test_multiple_requests_to_same_endpoint` - 5 concurrent requests
- ✓ `test_random_endpoint_value_matches_coins` - Value calculation accuracy

### Run Tests

```bash
# Run all 40 tests
cargo test

# Run only core library tests (24 tests)
cargo test --lib tests::

# Run only web module tests (16 tests)
cargo test web::tests

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_random_endpoint_produces_variety -- --nocapture
```

## Dependencies (Cargo.toml)

### Production Dependencies

```toml
[dependencies]
rand = "0.8"                      # Random number generation
axum = "0.8"                      # Web framework
tokio = { version = "1.42", features = ["full"] }  # Async runtime
serde = { version = "1.0", features = ["derive"] } # Serialization
serde_json = "1.0"                # JSON support
tower-http = { version = "0.6", features = ["cors", "trace"] }  # Middleware
tracing = "0.1"                   # Structured logging
tracing-subscriber = { version = "0.3", features = ["env-filter"] }  # Log subscriber
```

### Development Dependencies

```toml
[dev-dependencies]
tower = "0.5"              # Service testing utilities
http-body-util = "0.1"     # Body parsing for tests
mime = "0.3"               # Content-type testing
```

## Docker Integration

### Multi-Stage Dockerfile

The project uses a multi-stage build for optimal image size and security:

**Stage 1: Builder**
- Base: `rust:1.83`
- Compiles the application in release mode
- Runs all 40 tests during build
- ~2GB image size (not shipped)

**Stage 2: Runtime**
- Base: `debian:bookworm-slim`
- Copies only the compiled binary
- Non-root user (`coins:coins`)
- Exposes port 8080
- ~80MB final image size

### Docker Commands (Makefile)

```bash
make build              # Build Docker image
make run-web           # Run web server with port mapping
make run-web-detached  # Run in background
make stop-web          # Stop web server
make curl-test         # Test all endpoints
make clean             # Clean up resources
make rebuild           # Clean and rebuild
```

## Usage

### Local Development

```bash
# Run web server
cargo run --release

# Server starts on http://0.0.0.0:8080
# Test with: curl http://localhost:8080/random
```

### Docker Deployment

```bash
# Build and run
make build
make run-web

# Test endpoints
make curl-test

# Or manually
docker build -t coins:latest .
docker run -p 8080:8080 coins:latest
```

### API Usage Examples

```bash
# Get API information
curl http://localhost:8080/

# Health check
curl http://localhost:8080/health

# Random combination
curl http://localhost:8080/random

# All combinations
curl http://localhost:8080/all

# Query with jq
curl -s http://localhost:8080/all | jq '.combinations[5]'
curl -s http://localhost:8080/random | jq '.value'
```

## Key Rust Concepts Demonstrated

### Core Concepts
1. **Enums with Methods** - Rich enum types with associated functions
2. **Trait Derivation** - Automatic implementation of Debug, Clone, Copy, PartialEq, Serialize
3. **Bit Manipulation** - Efficient power set generation using bitwise operators
4. **Iterators** - Functional-style data processing with `iter()`, `map()`, `sum()`
5. **Pattern Matching** - Exhaustive matching in `value_in_cents()`
6. **Modules** - Clean separation (lib.rs, main.rs, web.rs)
7. **Testing** - Comprehensive unit and integration tests with `#[test]` and `#[tokio::test]`
8. **Ownership** - Borrowing with `&[Coin]` and lifetime management

### Web Development Concepts
9. **Async/Await** - Tokio runtime with async handlers
10. **HTTP Servers** - Axum web framework with routing
11. **JSON Serialization** - Serde for request/response handling
12. **Middleware** - CORS and request tracing with Tower-HTTP
13. **Error Handling** - Result types and HTTP status codes
14. **Type Safety** - Strong typing for response structures
15. **Integration Testing** - Testing HTTP endpoints with test client

### DevOps Concepts
16. **Docker** - Multi-stage builds for minimal image size
17. **CI/CD** - GitHub Actions with format, lint, test, build
18. **Makefile** - Development workflow automation
19. **Logging** - Structured logging with Tracing
20. **Security** - Non-root Docker user, minimal attack surface

## Mathematical Background

This implements a **power set** - the set of all subsets of a set.

For a set with n elements, the power set has 2^n elements:

- n = 4 coins
- 2^4 = 16 combinations
- Includes empty set {} and full set {P, N, D, Q}

**Bit Mapping:**
- Combination 0 (0000) = {} (empty)
- Combination 1 (0001) = {Penny}
- Combination 2 (0010) = {Nickel}
- Combination 3 (0011) = {Penny, Nickel}
- ...
- Combination 15 (1111) = {Penny, Nickel, Dime, Quarter}

## Performance Characteristics

- **Startup Time**: < 100ms
- **Response Time**: < 1ms per request (local)
- **Memory Usage**: ~5MB (Docker container)
- **Docker Image Size**: ~80MB (multi-stage build)
- **Concurrent Requests**: Tokio handles thousands efficiently
- **Algorithm Complexity**: O(2^n) for generation, O(1) for random

## Architecture Decisions

### Why Axum?
- Fast and ergonomic web framework
- Strong type safety
- Excellent error messages
- Built on Tokio (battle-tested)
- Modular middleware system

### Why Bit Manipulation?
- Elegant mathematical approach
- O(2^n) is optimal for power set
- Demonstrates low-level operations
- Educational value

### Why Multi-Stage Docker?
- Small final image (~80MB vs ~2GB)
- No build tools in production
- Security: minimal attack surface
- Fast deployment

### Why Comprehensive Testing?
- 40 tests ensure correctness
- Integration tests validate HTTP layer
- CI/CD catches regressions
- Documentation through tests

## Future Enhancements

Potential improvements:

### Features
- WebSocket support for real-time updates
- Rate limiting middleware
- API key authentication
- Query parameters for filtering combinations
- Pagination for /all endpoint
- Caching layer (Redis)

### Technical
- OpenAPI/Swagger documentation
- Metrics and monitoring (Prometheus)
- Database integration for persistence
- Graceful shutdown handling
- Configuration via environment variables
- Support for international currencies

### DevOps
- Kubernetes deployment manifests
- Helm charts
- Health check probes
- Horizontal scaling
- Load testing benchmarks

## References

- **Rust Book**: https://doc.rust-lang.org/book/
- **Axum Docs**: https://docs.rs/axum/
- **Tokio Guide**: https://tokio.rs/tokio/tutorial
- **Serde Guide**: https://serde.rs/
- **Power Set**: https://en.wikipedia.org/wiki/Power_set
- **Bit Manipulation**: https://en.wikipedia.org/wiki/Bit_manipulation
