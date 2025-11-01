// ============================================================================
// LIBRARY: Core Coin Combination Logic
// ============================================================================
// This file contains all the core logic for coin combinations.
// The `pub` keyword makes items publicly accessible from main.rs

use rand::Rng;

// Derive traits automatically:
// - Debug: allows printing with {:?}
// - Clone: allows creating copies of the enum
// - Copy: allows copying the value instead of moving it
// - PartialEq: allows comparing coins with == and !=
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    // Associated function (like a static method) that returns all coin types
    // Returns an array of all 4 coins
    pub fn all() -> [Coin; 4] {
        [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter]
    }

    // Method that returns the value of a coin in cents
    pub fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

// Function that generates all possible subsets (power set) of coins
// Returns a Vec (dynamic array) of Vecs, where each inner Vec is one combination
pub fn generate_all_combinations() -> Vec<Vec<Coin>> {
    let coins = Coin::all();
    let total_coins = coins.len(); // 4 coins
    let total_combinations = 1 << total_coins; // 2^4 = 16 combinations (bit shift left)

    // Create a mutable vector to store all combinations
    let mut combinations = Vec::new();

    // Iterate through all numbers from 0 to 15
    for i in 0..total_combinations {
        // Create a new vector for this combination
        let mut combination = Vec::new();

        // Check each bit position (0 to 3)
        // Note: We use the index j for bit manipulation (i >> j), not just array indexing
        #[allow(clippy::needless_range_loop)]
        for j in 0..total_coins {
            // If the j-th bit is set in i, include the j-th coin
            // Example: i=5 (binary: 0101)
            //   j=0: (5 >> 0) & 1 = 1 & 1 = 1 ✓ include Penny
            //   j=1: (5 >> 1) & 1 = 2 & 1 = 0 ✗ skip Nickel
            //   j=2: (5 >> 2) & 1 = 1 & 1 = 1 ✓ include Dime
            //   j=3: (5 >> 3) & 1 = 0 & 1 = 0 ✗ skip Quarter
            if (i >> j) & 1 == 1 {
                combination.push(coins[j]);
            }
        }

        combinations.push(combination);
    }

    combinations
}

// Helper function to calculate total value of a combination
pub fn total_value(coins: &[Coin]) -> u32 {
    // Use iterator methods:
    // - iter(): creates an iterator over the slice
    // - map(): transforms each coin to its value
    // - sum(): adds up all values
    coins.iter().map(|coin| coin.value_in_cents() as u32).sum()
}

// Function that generates a single random combination of coins
// Returns a Vec containing 0-4 coins, randomly selected
pub fn generate_random_combination() -> Vec<Coin> {
    let coins = Coin::all();
    let total_coins = coins.len(); // 4 coins
    let total_combinations = 1 << total_coins; // 2^4 = 16 combinations

    // Generate a random number from 0 to 15
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..total_combinations);

    // Create a vector for this random combination
    let mut combination = Vec::new();

    // Check each bit position using the same algorithm as generate_all_combinations()
    #[allow(clippy::needless_range_loop)]
    for j in 0..total_coins {
        // If the j-th bit is set in i, include the j-th coin
        if (i >> j) & 1 == 1 {
            combination.push(coins[j]);
        }
    }

    combination
}

// ============================================================================
// TESTS MODULE
// ============================================================================
// The #[cfg(test)] attribute tells Rust to compile this module only when
// running tests (with `cargo test`), not when building the regular program
#[cfg(test)]
mod tests {
    // Import all items from the parent module (our library code)
    use super::*;

    // ========================================================================
    // Tests for Coin::all()
    // ========================================================================

    // The #[test] attribute marks this function as a test
    #[test]
    fn test_coin_all_returns_four_coins() {
        let all_coins = Coin::all();
        // assert_eq! macro checks if two values are equal
        // If not, the test fails with a helpful error message
        assert_eq!(all_coins.len(), 4, "Should have exactly 4 coins");
    }

    #[test]
    fn test_coin_all_has_correct_coins() {
        let all_coins = Coin::all();
        assert_eq!(all_coins[0], Coin::Penny);
        assert_eq!(all_coins[1], Coin::Nickel);
        assert_eq!(all_coins[2], Coin::Dime);
        assert_eq!(all_coins[3], Coin::Quarter);
    }

    // ========================================================================
    // Tests for Coin::value_in_cents()
    // ========================================================================

    #[test]
    fn test_penny_value() {
        assert_eq!(Coin::Penny.value_in_cents(), 1);
    }

    #[test]
    fn test_nickel_value() {
        assert_eq!(Coin::Nickel.value_in_cents(), 5);
    }

    #[test]
    fn test_dime_value() {
        assert_eq!(Coin::Dime.value_in_cents(), 10);
    }

    #[test]
    fn test_quarter_value() {
        assert_eq!(Coin::Quarter.value_in_cents(), 25);
    }

    // ========================================================================
    // Tests for total_value()
    // ========================================================================

    #[test]
    fn test_total_value_empty() {
        let empty: Vec<Coin> = vec![];
        assert_eq!(total_value(&empty), 0);
    }

    #[test]
    fn test_total_value_single_coin() {
        let coins = vec![Coin::Quarter];
        assert_eq!(total_value(&coins), 25);
    }

    #[test]
    fn test_total_value_multiple_coins() {
        let coins = vec![Coin::Quarter, Coin::Dime, Coin::Nickel, Coin::Penny];
        // 25 + 10 + 5 + 1 = 41
        assert_eq!(total_value(&coins), 41);
    }

    #[test]
    fn test_total_value_duplicate_coins() {
        let coins = vec![Coin::Nickel, Coin::Nickel];
        assert_eq!(total_value(&coins), 10);
    }

    // ========================================================================
    // Tests for generate_all_combinations()
    // ========================================================================

    #[test]
    fn test_combinations_count() {
        let combinations = generate_all_combinations();
        // For 4 coins, we should have 2^4 = 16 combinations
        assert_eq!(combinations.len(), 16, "Should generate 16 combinations");
    }

    #[test]
    fn test_combinations_has_empty_set() {
        let combinations = generate_all_combinations();
        // First combination (index 0) should be empty
        assert!(
            combinations[0].is_empty(),
            "First combination should be empty set"
        );
    }

    #[test]
    fn test_combinations_has_full_set() {
        let combinations = generate_all_combinations();
        // Last combination (index 15) should have all 4 coins
        assert_eq!(
            combinations[15].len(),
            4,
            "Last combination should have all 4 coins"
        );
    }

    #[test]
    fn test_specific_combination_penny_only() {
        let combinations = generate_all_combinations();
        // Combination 1 (binary: 0001) should be just Penny
        let expected = vec![Coin::Penny];
        assert_eq!(combinations[1], expected);
    }

    #[test]
    fn test_specific_combination_nickel_only() {
        let combinations = generate_all_combinations();
        // Combination 2 (binary: 0010) should be just Nickel
        let expected = vec![Coin::Nickel];
        assert_eq!(combinations[2], expected);
    }

    #[test]
    fn test_specific_combination_penny_and_dime() {
        let combinations = generate_all_combinations();
        // Combination 5 (binary: 0101) should be Penny and Dime
        let expected = vec![Coin::Penny, Coin::Dime];
        assert_eq!(combinations[5], expected);
    }

    #[test]
    fn test_all_combinations_unique() {
        let combinations = generate_all_combinations();
        // Check that no two combinations are the same
        for i in 0..combinations.len() {
            for j in (i + 1)..combinations.len() {
                // Compare by converting to debug strings (simple approach)
                assert_ne!(
                    format!("{:?}", combinations[i]),
                    format!("{:?}", combinations[j]),
                    "Combinations {} and {} should be different",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_combination_values_range() {
        let combinations = generate_all_combinations();
        // Check that values range from 0 to 41 cents
        let min_value = combinations.iter().map(|c| total_value(c)).min().unwrap();
        let max_value = combinations.iter().map(|c| total_value(c)).max().unwrap();

        assert_eq!(min_value, 0, "Minimum value should be 0 (empty set)");
        assert_eq!(max_value, 41, "Maximum value should be 41 (all coins)");
    }

    // ========================================================================
    // Integration/Edge Case Tests
    // ========================================================================

    #[test]
    fn test_all_single_coin_combinations_exist() {
        let combinations = generate_all_combinations();
        // Check that all single-coin combinations exist
        let single_coin_combos: Vec<_> = combinations.iter().filter(|c| c.len() == 1).collect();

        assert_eq!(
            single_coin_combos.len(),
            4,
            "Should have 4 single-coin combinations"
        );
    }

    #[test]
    fn test_coin_copy_trait() {
        // Test that Copy trait works correctly
        let coin1 = Coin::Penny;
        let coin2 = coin1; // This copies, not moves
        let coin3 = coin1; // We can still use coin1

        assert_eq!(coin1, coin2);
        assert_eq!(coin2, coin3);
    }

    // ========================================================================
    // Tests for generate_random_combination()
    // ========================================================================

    #[test]
    fn test_random_combination_length_valid() {
        // Run multiple times to test randomness
        for _ in 0..20 {
            let combination = generate_random_combination();
            // Length should be between 0 and 4 (inclusive)
            assert!(
                combination.len() <= 4,
                "Random combination should have at most 4 coins"
            );
        }
    }

    #[test]
    fn test_random_combination_coins_are_valid() {
        // Run multiple times to test randomness
        for _ in 0..20 {
            let combination = generate_random_combination();
            // All coins in the combination should be valid coin types
            for coin in &combination {
                assert!(
                    matches!(coin, Coin::Penny | Coin::Nickel | Coin::Dime | Coin::Quarter),
                    "All coins should be valid coin types"
                );
            }
        }
    }

    #[test]
    fn test_random_combination_value_in_range() {
        // Run multiple times to test randomness
        for _ in 0..20 {
            let combination = generate_random_combination();
            let value = total_value(&combination);
            // Value should be between 0 (empty) and 41 (all coins)
            assert!(
                value <= 41,
                "Random combination value should be at most 41 cents"
            );
        }
    }

    #[test]
    fn test_random_combination_produces_variety() {
        // Generate multiple combinations and check they're not all identical
        let mut combinations = Vec::new();
        for _ in 0..50 {
            let combination = generate_random_combination();
            combinations.push(format!("{:?}", combination));
        }

        // Remove duplicates by sorting and deduping
        combinations.sort();
        combinations.dedup();

        // With 50 attempts, we should get at least 2 different combinations
        // (statistically very likely given 16 possible combinations)
        assert!(
            combinations.len() >= 2,
            "Should generate variety in random combinations"
        );
    }
}
