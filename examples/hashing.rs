//! Example: Hashing utilities
//!
//! Run with:
//!   cargo run --example hashing

use soroban_toolkit::hash::{double_sha256, secure_compare, sha256_bytes, sha256_hex, sha512_hex};

fn main() {
    let data = b"hello";

    let sha256 = sha256_hex(data);
    println!("SHA-256:        {}", sha256);

    let sha512 = sha512_hex(data);
    println!("SHA-512:        {}", sha512);

    let double = double_sha256(data);
    println!("Double SHA-256: {}", double);

    // sha256_bytes for chaining
    let bytes = sha256_bytes(data);
    println!("SHA-256 bytes:  {} bytes", bytes.len());

    // Timing-safe comparison
    let a = sha256_bytes(b"secret");
    let b = sha256_bytes(b"secret");
    let c = sha256_bytes(b"other");
    assert!(secure_compare(&a, &b));
    assert!(!secure_compare(&a, &c));
    println!("Secure compare: ok");
}
