//! Example: Memo utilities
//!
//! Run with:
//!   cargo run --example memo

use soroban_toolkit::memo::{validate_memo, MemoType};

fn main() {
    let memos = [
        MemoType::None,
        MemoType::Text("Payment for services".to_string()),
        MemoType::Id(987654321),
        MemoType::Hash("a".repeat(64)),
        MemoType::Return("b".repeat(64)),
    ];

    for memo in &memos {
        match validate_memo(memo) {
            Ok(()) => println!("{:?} → valid", memo),
            Err(e) => println!("{:?} → error: {}", memo, e),
        }
    }

    // Invalid cases
    let too_long = MemoType::Text("x".repeat(29));
    let bad_hash = MemoType::Hash("not-a-hash".to_string());

    println!("{:?} → {}", too_long, validate_memo(&too_long).unwrap_err());
    println!("{:?} → {}", bad_hash, validate_memo(&bad_hash).unwrap_err());
}
