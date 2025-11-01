# Coin Combinations REST API

[![CI](https://github.com/yenonn/coins/actions/workflows/ci.yml/badge.svg)](https://github.com/yenonn/coins/actions/workflows/ci.yml)

A Rust web service that provides a REST API for generating and querying all possible combinations of US coins using bit manipulation to create a power set.

## Overview

This project demonstrates fundamental Rust concepts through a practical web API: generating all 16 possible combinations of four US coins (Penny, Nickel, Dime, Quarter) and exposing them via HTTP endpoints. It uses bit manipulation techniques to efficiently create the power set and showcases Rust's web framework ecosystem (Axum + Tokio), type system, iterators, and comprehensive testing.

## Features

- 🌐 **REST API** with 4 HTTP endpoints
- 🪙 Generate all possible coin combinations (power set)
- 💰 Calculate total value for any combination
- 🎲 Random combination generator
- 🐳 Docker support with multi-stage builds
- 🧪 Comprehensive test suite (40+ tests)
- 📊 JSON responses for all endpoints
- 📈 Code coverage tracking with cargo-llvm-cov
- 🔒 Security-focused Docker configuration
- 📚 Well-documented with extensive inline comments

## Prerequisites

- Rust 1.80+ (edition 2021)
- Cargo (comes with Rust)
- Docker (optional, for containerized deployment)

## Quick Start

### Run Locally

```bash
# Clone the repository
git clone <repository-url>
cd coins

# Run the web server
cargo run --release

# Server starts on http://0.0.0.0:8080
```

### Run with Docker

```bash
# Build the Docker image
make build

# Run the web server
make run-web

# Or use Docker directly
docker build -t coins:latest .
docker run -p 8080:8080 coins:latest
```

### Test the API

```bash
# Test all endpoints with curl
make curl-test

# Or test manually
curl http://localhost:8080/random
curl http://localhost:8080/health
curl http://localhost:8080/all
```

## API Endpoints

### GET `/`
Returns API information and available endpoints.

**Response:**
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

### GET `/health`
Health check endpoint for monitoring and load balancers.

**Response:**
```json
{
  "status": "healthy",
  "service": "coins-api",
  "version": "0.1.0"
}
```

### GET `/random`
Returns a random coin combination with its total value.

**Response:**
```json
{
  "coins": ["Penny", "Dime"],
  "value": 11
}
```

### GET `/all`
Returns all 16 possible coin combinations.

**Response:**
```json
{
  "total_combinations": 16,
  "combinations": [
    {
      "index": 0,
      "coins": [],
      "value": 0
    },
    {
      "index": 1,
      "coins": ["Penny"],
      "value": 1
    },
    ...
    {
      "index": 15,
      "coins": ["Penny", "Nickel", "Dime", "Quarter"],
      "value": 41
    }
  ]
}
```

## Project Structure

```
coins/
├── Cargo.toml          # Project manifest and dependencies
├── Dockerfile          # Multi-stage Docker build configuration
├── Makefile            # Convenient build and run commands
├── README.md           # This file
├── CLAUDE.md           # Detailed technical documentation
├── .github/
│   └── workflows/
│       └── ci.yml      # GitHub Actions CI/CD pipeline
└── src/
    ├── lib.rs          # Core library (Coin enum, algorithms, tests)
    ├── main.rs         # Web server entry point
    └── web.rs          # HTTP API implementation (Axum routes, handlers)
```

## Running Tests

```bash
# Run all tests (40 tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test web::tests

# Run tests in Docker
make test
```

## Code Coverage

This project includes code coverage tracking using `cargo-llvm-cov`, which provides accurate line and branch coverage analysis.

### Prerequisites

Install cargo-llvm-cov (one-time setup):

```bash
cargo install cargo-llvm-cov
```

### Generate Coverage Reports

```bash
# Generate lcov.info coverage file
make coverage

# Generate and open HTML coverage report in browser
make coverage-html

# Show coverage summary in terminal
make coverage-text

# Generate lcov format for CI/editors
make coverage-lcov
```

### Understanding Coverage Output

The HTML report (`make coverage-html`) provides:
- **Line coverage**: Which lines of code are executed by tests
- **Branch coverage**: Which code paths (if/else, match arms) are tested
- **Function coverage**: Which functions are called during tests
- **Region coverage**: Detailed coverage of code regions

Coverage files and directories (automatically ignored by git):
- `lcov.info` - LCOV format coverage data
- `target/llvm-cov-target/` - Coverage build artifacts
- `html/` or `target/llvm-cov/html/` - HTML coverage reports

### CI Integration

Code coverage is automatically generated in the CI pipeline:
- Coverage runs on every push and pull request
- Coverage report is uploaded to Codecov (if configured)
- Coverage artifacts are available for download from GitHub Actions

## Docker Commands

The project includes a comprehensive Makefile with convenient Docker commands:

```bash
# Build the Docker image
make build

# Run web server (foreground)
make run-web

# Run web server (background)
make run-web-detached

# Stop web server
make stop-web

# Test API endpoints
make curl-test

# Clean up Docker resources
make clean

# Rebuild from scratch
make rebuild
```

## How It Works

### Bit Manipulation Algorithm

The program uses **bit manipulation** to generate all combinations:

1. For 4 coins, there are 2^4 = 16 possible combinations
2. Each number from 0-15 represents a unique combination
3. Each bit position indicates whether to include that coin:
   - Bit 0: Penny
   - Bit 1: Nickel
   - Bit 2: Dime
   - Bit 3: Quarter

Example: `5` in binary is `0101`:
- Bit 0 (1) → Include Penny ✓
- Bit 1 (0) → Skip Nickel
- Bit 2 (1) → Include Dime ✓
- Bit 3 (0) → Skip Quarter
- Result: {Penny, Dime} = 11 cents

### Web Architecture

- **Framework**: Axum (fast, ergonomic web framework)
- **Runtime**: Tokio (async runtime)
- **Serialization**: Serde (JSON responses)
- **Middleware**: Tower-HTTP (CORS, logging)
- **Logging**: Tracing (structured logging)

## Key Concepts Demonstrated

### Rust Core Concepts
- **Enums**: Rich enum types with methods and traits
- **Traits**: Deriving Debug, Clone, Copy, PartialEq, Serialize
- **Bit Manipulation**: Efficient power set generation
- **Iterators**: Functional programming with `iter()`, `map()`, `sum()`
- **Pattern Matching**: Exhaustive matching in value calculations
- **Modules**: Clean separation (lib.rs, main.rs, web.rs)
- **Ownership**: Borrowing and references

### Web Development
- **Async/Await**: Tokio runtime and async handlers
- **REST API**: RESTful endpoint design
- **JSON**: Serialization with Serde
- **Error Handling**: Result types and proper HTTP status codes
- **Middleware**: CORS and request tracing

### DevOps & Testing
- **Unit Testing**: 24 core logic tests
- **Integration Testing**: 16 HTTP endpoint tests
- **Code Coverage**: cargo-llvm-cov for coverage tracking
- **Docker**: Multi-stage builds for minimal image size
- **CI/CD**: GitHub Actions pipeline
- **Makefile**: Development workflow automation

## API Reference

### Core Library

#### `Coin` Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Coin {
    Penny,   // 1 cent
    Nickel,  // 5 cents
    Dime,    // 10 cents
    Quarter, // 25 cents
}
```

**Methods:**
- `Coin::all() -> [Coin; 4]` - Returns array of all coin types
- `coin.value_in_cents() -> u8` - Returns the coin's value

#### Functions

- `generate_all_combinations() -> Vec<Vec<Coin>>` - Generates all 16 combinations
- `generate_random_combination() -> Vec<Coin>` - Generates a random combination
- `total_value(coins: &[Coin]) -> u32` - Calculates total value of coins

### Web Module

#### Response Types

- `RandomResponse` - Structure for `/random` endpoint
- `HealthResponse` - Structure for `/health` endpoint
- `AllCombinationsResponse` - Structure for `/all` endpoint
- `CombinationDetail` - Individual combination details

#### Functions

- `create_router() -> Router` - Creates the Axum router with all endpoints
- `run_server(addr: &str) -> Result<()>` - Starts the HTTP server

## Examples

### Using the Library

```rust
use coins::{Coin, generate_all_combinations, generate_random_combination, total_value};

// Get all combinations
let combinations = generate_all_combinations();
println!("Total: {}", combinations.len()); // 16

// Calculate value
let my_coins = vec![Coin::Quarter, Coin::Dime];
println!("Value: {} cents", total_value(&my_coins)); // 35

// Get random combination
let random = generate_random_combination();
println!("Random: {:?}", random);
```

### Using the API

```bash
# Get a random combination
curl http://localhost:8080/random
# {"coins":["Nickel","Quarter"],"value":30}

# Get all combinations
curl http://localhost:8080/all | jq '.total_combinations'
# 16

# Health check
curl http://localhost:8080/health
# {"status":"healthy","service":"coins-api","version":"0.1.0"}
```

### Using with jq

```bash
# Get only the coins from a random combination
curl -s http://localhost:8080/random | jq '.coins'

# Get the value of combination at index 5
curl -s http://localhost:8080/all | jq '.combinations[5].value'

# Count combinations with value > 20 cents
curl -s http://localhost:8080/all | jq '[.combinations[] | select(.value > 20)] | length'
```

## Development

### Local Development

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Build documentation
cargo doc --open

# Build release version
cargo build --release

# Run locally
cargo run --release
```

### Docker Development

```bash
# Run shell in builder environment
make build-shell

# View logs from running container
make logs

# Inspect Docker image
make inspect
```

## Continuous Integration

This project uses GitHub Actions for automated testing. The CI pipeline runs on:
- Pull requests to `main` or `master` branches
- Direct pushes to `main` or `master` branches

### CI Checks

The workflow performs the following checks:

1. **Code Formatting** - Ensures code follows Rust style guidelines (`cargo fmt`)
2. **Linting** - Catches common mistakes and enforces best practices (`cargo clippy`)
3. **Build** - Verifies the project compiles successfully
4. **Tests** - Runs all 40 unit and integration tests
5. **Code Coverage** - Generates coverage report with `cargo-llvm-cov` and uploads to Codecov

All checks must pass before a pull request can be merged to the main branch.

### Running CI Locally

To run the same checks locally before pushing:

```bash
# Format check
cargo fmt -- --check

# Linting
cargo clippy -- -D warnings

# Build and test
cargo build --verbose
cargo test --verbose

# Docker build
docker build -t coins:latest .
```

## Performance

- **Startup Time**: < 100ms
- **Response Time**: < 1ms per request (local)
- **Memory Usage**: ~5MB (Docker container)
- **Image Size**: ~80MB (multi-stage build)

## Security

- ✅ Non-root user in Docker container
- ✅ Multi-stage build (no build tools in final image)
- ✅ Minimal base image (Debian Bookworm Slim)
- ✅ No hardcoded secrets
- ⚠️ CORS configured as permissive (suitable for public API)

## License

This is an educational project for learning Rust.

## Contributing

This is a learning project, but suggestions and improvements are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and formatting checks
5. Submit a pull request

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [Power Set (Mathematics)](https://en.wikipedia.org/wiki/Power_set)

## Troubleshooting

### Port Already in Use

If port 8080 is already in use:
```bash
# Find process using port 8080
lsof -i :8080

# Kill the process or use a different port
# Edit src/main.rs to change the port
```

### Docker Build Fails

```bash
# Clean Docker build cache
docker builder prune

# Rebuild from scratch
make rebuild
```

### Tests Fail

```bash
# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

## See Also

- `CLAUDE.md` - Detailed technical documentation with code references
- `Dockerfile` - Multi-stage build configuration
- `Makefile` - Available commands and targets
