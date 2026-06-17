//! Example: Transaction utilities
//!
//! Run with:
//!   cargo run --example transaction

use soroban_toolkit::transaction::{
    estimate_fee, estimate_fee_xlm, format_xlm, is_valid_tx_hash, normalize_tx_hash,
    stroops_to_xlm, xlm_to_stroops,
};

fn main() {
    // Stroops / XLM conversion
    println!("1 XLM = {} stroops", xlm_to_stroops(1.0));
    println!("10_000_000 stroops = {} XLM", stroops_to_xlm(10_000_000));
    println!("Formatted: {}", format_xlm(10_000_000));

    // Fee estimation
    let stroops = estimate_fee(100, 3);
    let xlm = estimate_fee_xlm(100, 3);
    println!("Fee (3 ops @ 100): {} stroops / {:.7} XLM", stroops, xlm);

    // Hash validation
    let valid_hash = "a".repeat(64);
    let prefixed = format!("0x{}", "b".repeat(64));

    println!("Valid hash: {}", is_valid_tx_hash(&valid_hash));
    println!("Normalized: {}", normalize_tx_hash(&prefixed).unwrap());
    println!("Invalid hash: {}", is_valid_tx_hash("short"));
}
