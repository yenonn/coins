# Coin Combinations - Rust Learning Project

## Overview

This Rust project demonstrates generating and displaying all possible combinations of US coins (Penny, Nickel, Dime, Quarter) using bit manipulation to create a power set. It's an educational example showcasing Rust fundamentals including enums, traits, iterators, and testing.

## Project Structure

```
coins/
├── Cargo.toml          # Project manifest
├── src/
│   ├── lib.rs          # Core library with coin logic
│   └── main.rs         # Main executable
└── claude.md           # This documentation
```

## Core Components

### Coin Enum (src/lib.rs:13-36)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
```

**Derived Traits:**

- `Debug`: Enables printing with `{:?}` format
- `Clone`: Allows creating copies explicitly
- `Copy`: Enables implicit copying (coins are small, stack-allocated)
- `PartialEq`: Allows equality comparisons with `==` and `!=`

**Methods:**

- `all()` - Returns array of all 4 coin types `[Coin; 4]`
- `value_in_cents(&self)` - Returns coin value: Penny=1, Nickel=5, Dime=10, Quarter=25

### Power Set Generation (src/lib.rs:40-70)

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

### Value Calculation (src/lib.rs:73-79)

```rust
pub fn total_value(coins: &[Coin]) -> u32 {
    coins.iter().map(|coin| coin.value_in_cents() as u32).sum()
}
```

Uses iterator chain:

- `iter()` - Creates iterator over coin slice
- `map()` - Transforms each coin to its value
- `sum()` - Aggregates all values

## Main Program (src/main.rs)

Displays all 16 combinations with their values:

```
=== All Coin Combinations ===

Combination  0: {} (empty set) - Value: 0 cents
Combination  1: {Penny} - Value: 1 cents
Combination  2: {Nickel} - Value: 5 cents
...
Combination 15: {Penny, Nickel, Dime, Quarter} - Value: 41 cents

Total combinations: 16
```

## Testing

The project includes comprehensive tests (src/lib.rs:86-285) covering:

### Coin Tests

- ✓ `test_coin_all_returns_four_coins` - Verifies 4 coins returned
- ✓ `test_coin_all_has_correct_coins` - Validates correct order
- ✓ `test_penny_value` through `test_quarter_value` - Tests each coin value

### Total Value Tests

- ✓ `test_total_value_empty` - Empty set = 0 cents
- ✓ `test_total_value_single_coin` - Single coin value
- ✓ `test_total_value_multiple_coins` - Sum of multiple coins
- ✓ `test_total_value_duplicate_coins` - Handles duplicates

### Combination Generation Tests

- ✓ `test_combinations_count` - Verifies 16 combinations
- ✓ `test_combinations_has_empty_set` - Index 0 is empty
- ✓ `test_combinations_has_full_set` - Index 15 has all coins
- ✓ `test_specific_combination_*` - Validates specific bit patterns
- ✓ `test_all_combinations_unique` - No duplicate combinations
- ✓ `test_combination_values_range` - Values range 0-41 cents

### Run Tests

```bash
cargo test
```

## Usage

### Build and Run

```bash
cargo build
cargo run
```

### Run in Release Mode

```bash
cargo run --release
```

## Key Rust Concepts Demonstrated

1. **Enums with Methods** - Rich enum types with associated functions
2. **Trait Derivation** - Automatic implementation of common traits
3. **Bit Manipulation** - Using bitwise operators for algorithm efficiency
4. **Iterators** - Functional-style data processing with `iter()`, `map()`, `sum()`
5. **Pattern Matching** - Exhaustive matching in `value_in_cents()`
6. **Modules** - Separation of library (`lib.rs`) and binary (`main.rs`)
7. **Testing** - Comprehensive unit tests with `#[test]` and `#[cfg(test)]`
8. **Ownership** - Borrowing with `&[Coin]` in `total_value()`

## Mathematical Background

This implements a **power set** - the set of all subsets of a set.

For a set with n elements, the power set has 2^n elements:

- n = 4 coins
- 2^4 = 16 combinations
- Includes empty set {} and full set {P, N, D, Q}

## Future Enhancements

Potential improvements:

- Accept user input for coin selection
- Calculate optimal change for a given amount
- Support international currencies
- Add coin frequency/quantity support
- Generate combinations with specific value targets
