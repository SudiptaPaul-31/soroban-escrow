//! Example: Asset utilities
//!
//! Run with:
//!   cargo run --example asset

use soroban_toolkit::asset::{validate_asset_code, AssetType};

fn main() {
    let codes = ["XLM", "native", "USD", "USDT", "USDCENTERPR1", "", "USD$"];

    for code in &codes {
        match validate_asset_code(code) {
            Ok(AssetType::Native) => println!("{:12} → Native", code),
            Ok(AssetType::Alphanumeric4) => println!("{:12} → Alphanumeric4", code),
            Ok(AssetType::Alphanumeric12) => println!("{:12} → Alphanumeric12", code),
            Err(e) => println!("{:12} → Error: {}", code, e),
        }
    }
}
