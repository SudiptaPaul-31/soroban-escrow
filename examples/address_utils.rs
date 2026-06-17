//! Example: Address utilities
//!
//! Run with:
//!   cargo run --example address_utils

use soroban_toolkit::address::{
    detect_address_type, is_account_address, is_contract_address, mask_address, validate_address,
    AddressType,
};

fn main() {
    let account = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
    let contract = "CCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";

    // Validate
    match validate_address(account) {
        Ok(addr) => println!("Valid address: {}", addr),
        Err(e) => println!("Invalid: {}", e),
    }

    // Mask
    println!("Masked account:  {}", mask_address(account));
    println!("Masked contract: {}", mask_address(contract));

    // Detect type
    for addr in [account, contract] {
        let label = match detect_address_type(addr) {
            AddressType::Account => "Account",
            AddressType::Contract => "Contract",
            AddressType::Invalid => "Invalid",
        };
        println!("{} → {}", &addr[..4], label);
    }

    // Predicate helpers
    assert!(is_account_address(account));
    assert!(is_contract_address(contract));
    println!("Predicate checks passed.");
}
