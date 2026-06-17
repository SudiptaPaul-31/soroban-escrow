//! Example: Address validation
//!
//! Demonstrates validating, masking, and detecting the type of Stellar/Soroban addresses.
//!
//! Run with:
//!   cargo run --example address_validation

use soroban_toolkit::address::{
    detect_address_type, is_account_address, is_contract_address, mask_address, validate_address,
    AddressType,
};

fn main() {
    let account = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
    let contract = "CCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
    let invalid = "GSHORT";

    // Validate addresses
    match validate_address(account) {
        Ok(addr) => println!("Valid:   {}", addr),
        Err(e) => println!("Invalid: {}", e),
    }
    match validate_address(invalid) {
        Ok(addr) => println!("Valid:   {}", addr),
        Err(e) => println!("Invalid: {}", e),
    }

    // Mask addresses (show only first 4 and last 4 chars)
    println!("Masked account:  {}", mask_address(account));
    println!("Masked contract: {}", mask_address(contract));

    // Detect address type
    for addr in [account, contract, invalid] {
        let label = match detect_address_type(addr) {
            AddressType::Account => "Account",
            AddressType::Contract => "Contract",
            AddressType::Invalid => "Invalid",
        };
        println!("{} → {}", &addr[..6], label);
    }

    // Predicate helpers
    assert!(is_account_address(account));
    assert!(is_contract_address(contract));
    println!("Predicate checks passed.");
}
