//! Example: XLM formatting and transaction utilities
//!
//! Demonstrates stroops/XLM conversion, fee estimation, and hash utilities.
//!
//! Run with:
//!   cargo run --example xlm_formatting

use soroban_toolkit::transaction::{
    estimate_fee, estimate_fee_xlm, format_xlm, is_valid_tx_hash, normalize_tx_hash,
    stroops_to_xlm, xlm_to_stroops,
};

fn main() {
    // Stroops ↔ XLM conversion (1 XLM = 10,000,000 stroops)
    println!("1.0 XLM       = {} stroops", xlm_to_stroops(1.0));
    println!("10_000_000    = {} XLM", stroops_to_xlm(10_000_000));
    println!("Formatted:      {}", format_xlm(10_000_000));
    println!("0.5 XLM       = {} stroops", xlm_to_stroops(0.5));

    // Fee estimation
    let stroops = estimate_fee(100, 3);
    let xlm = estimate_fee_xlm(100, 3);
    println!(
        "\nFee (3 ops @ 100 stroops base): {} stroops / {:.7} XLM",
        stroops, xlm
    );

    // Transaction hash validation
    let valid_hash = "a".repeat(64);
    let prefixed = format!("0x{}", "b".repeat(64));

    println!(
        "\nHash '{}...' valid: {}",
        &valid_hash[..8],
        is_valid_tx_hash(&valid_hash)
    );
    println!(
        "Normalized 0x-prefixed: {}...",
        &normalize_tx_hash(&prefixed).unwrap()[..8]
    );
    println!("Hash 'short' valid: {}", is_valid_tx_hash("short"));
}
