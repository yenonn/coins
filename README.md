# Coin Combinations

[![CI](https://github.com/yenonn/coins/actions/workflows/ci.yml/badge.svg)](https://github.com/yenonn/coins/actions/workflows/ci.yml)

A Rust program that generates and displays all possible combinations of US coins using bit manipulation to create a power set.

## Overview

This project demonstrates fundamental Rust concepts through a practical example: generating all 16 possible combinations of four US coins (Penny, Nickel, Dime, Quarter). It uses bit manipulation techniques to efficiently create the power set and showcases Rust's type system, iterators, and testing capabilities.

## Features

- Generate all possible coin combinations (power set)
- Calculate total value for any combination
- Comprehensive unit test suite (20+ tests)
- Clean separation of library and binary code
- Well-documented with extensive inline comments

## Prerequisites

- Rust 1.80+ (edition 2024)
- Cargo (comes with Rust)

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd coins

# Build the project
cargo build

# Run the program
cargo run
```

## Usage

Run the program to see all 16 coin combinations:

```bash
cargo run
```

Output:
```
=== All Coin Combinations ===

Combination  0: {} (empty set) - Value: 0 cents
Combination  1: {Penny} - Value: 1 cents
Combination  2: {Nickel} - Value: 5 cents
Combination  3: {Penny, Nickel} - Value: 6 cents
...
Combination 15: {Penny, Nickel, Dime, Quarter} - Value: 41 cents

Total combinations: 16
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_combinations_count
```

## Project Structure

```
coins/
├── Cargo.toml          # Project manifest and dependencies
├── src/
│   ├── lib.rs          # Core library (Coin enum, algorithms, tests)
│   └── main.rs         # Main executable
├── .gitignore          # Git ignore rules
├── README.md           # This file
└── claude.md           # Detailed technical documentation
```

## How It Works

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
- Result: {Penny, Dime}

## Key Concepts Demonstrated

- **Enums**: Rich enum types with methods
- **Traits**: Deriving Debug, Clone, Copy, PartialEq
- **Bit Manipulation**: Efficient power set generation
- **Iterators**: Functional programming with `iter()`, `map()`, `sum()`
- **Pattern Matching**: Exhaustive matching in value calculations
- **Modules**: Library (lib.rs) and binary (main.rs) separation
- **Testing**: Comprehensive unit tests with `#[test]`
- **Ownership**: Borrowing and references

## API Reference

### `Coin` Enum

```rust
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

### Functions

- `generate_all_combinations() -> Vec<Vec<Coin>>` - Generates all 16 combinations
- `total_value(coins: &[Coin]) -> u32` - Calculates total value of coins

## Examples

```rust
use coins::{Coin, generate_all_combinations, total_value};

// Get all combinations
let combinations = generate_all_combinations();
println!("Total: {}", combinations.len()); // 16

// Calculate value
let my_coins = vec![Coin::Quarter, Coin::Dime];
println!("Value: {} cents", total_value(&my_coins)); // 35
```

## Development

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Build documentation
cargo doc --open

# Build release version
cargo build --release
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
4. **Tests** - Runs all unit tests to ensure functionality
5. **Coverage** - Generates code coverage reports (optional)

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
```

## License

This is an educational project for learning Rust.

## Contributing

This is a learning project, but suggestions and improvements are welcome!

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Power Set (Mathematics)](https://en.wikipedia.org/wiki/Power_set)

## See Also

- `claude.md` - Detailed technical documentation with code references
