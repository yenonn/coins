// ============================================================================
// MAIN PROGRAM: Coin Combinations Web API
// ============================================================================
// This file starts the web server for the coin combinations API

use coins::web;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";

    if let Err(e) = web::run_server(addr).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
