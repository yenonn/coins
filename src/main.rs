// ============================================================================
// MAIN PROGRAM: Coin Combinations Display
// ============================================================================
// This file uses the coin library to generate and display all combinations

// Import all public items from our library (lib.rs)
// `coins` is the crate name from Cargo.toml
use coins::{generate_all_combinations, generate_random_combination, total_value};

fn main() {
    println!("=== All Coin Combinations ===\n");

    let all_combinations = generate_all_combinations();

    // Enumerate gives us (index, item) pairs
    for (index, combination) in all_combinations.iter().enumerate() {
        print!("Combination {:2}: ", index);

        if combination.is_empty() {
            println!("{{}} (empty set) - Value: 0 cents");
        } else {
            // Print the coins in this combination
            print!("{{");
            for (i, coin) in combination.iter().enumerate() {
                print!("{:?}", coin);
                if i < combination.len() - 1 {
                    print!(", ");
                }
            }
            println!("}} - Value: {} cents", total_value(combination));
        }
    }

    println!("\nTotal combinations: {}", all_combinations.len());

    // ========================================================================
    // Display random combinations
    // ========================================================================
    println!("\n=== Random Coin Combinations ===\n");

    // Generate and display 5 random combinations
    for i in 1..=5 {
        let random_combination = generate_random_combination();
        print!("Random {:2}: ", i);

        if random_combination.is_empty() {
            println!("{{}} (empty set) - Value: 0 cents");
        } else {
            // Print the coins in this random combination
            print!("{{");
            for (j, coin) in random_combination.iter().enumerate() {
                print!("{:?}", coin);
                if j < random_combination.len() - 1 {
                    print!(", ");
                }
            }
            println!("}} - Value: {} cents", total_value(&random_combination));
        }
    }
}
