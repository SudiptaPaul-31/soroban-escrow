//! Example: Encoding utilities
//!
//! Run with:
//!   cargo run --example encoding

use soroban_toolkit::encoding::{
    chunk_bytes, from_base64, from_base64_url, from_hex, pretty_print_json, reassemble_chunks,
    to_base64, to_base64_url, to_hex,
};

fn main() {
    let data = b"soroban-toolkit";

    // Hex roundtrip
    let hex = to_hex(data);
    let decoded = from_hex(&hex).unwrap();
    println!("Hex:     {}", hex);
    assert_eq!(decoded, data);

    // Base64 roundtrip
    let b64 = to_base64(data);
    let decoded = from_base64(&b64).unwrap();
    println!("Base64:  {}", b64);
    assert_eq!(decoded, data);

    // URL-safe base64
    let b64url = to_base64_url(data);
    let decoded = from_base64_url(&b64url).unwrap();
    println!("Base64u: {}", b64url);
    assert_eq!(decoded, data);

    // Chunk and reassemble
    let chunks = chunk_bytes(data, 5);
    let reassembled = reassemble_chunks(&chunks);
    println!("Chunks:  {:?}", chunks.len());
    assert_eq!(reassembled, data);

    // Pretty-print JSON
    let json = r#"{"network":"testnet","fee":100}"#;
    let pretty = pretty_print_json(json).unwrap();
    println!("JSON:\n{}", pretty);
}
