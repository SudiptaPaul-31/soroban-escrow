use clap::{Parser, Subcommand};
use std::process;

use soroban_toolkit::address::validate_address;
use soroban_toolkit::encoding::{
    from_base64, from_base64_url, from_hex, to_base64, to_base64_url, to_hex,
};
use soroban_toolkit::hash::{double_sha256, sha256_hex, sha512_hex};
use soroban_toolkit::transaction::{format_xlm, stroops_to_xlm, xlm_to_stroops};

#[derive(Parser)]
#[command(name = "soroban-toolkit")]
#[command(about = "Soroban utility toolkit CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validates a Stellar/Soroban address
    ValidateAddress {
        /// The Stellar address to validate
        address: String,
    },
    /// Hashes the input data using the specified algorithm
    Hash {
        /// The input data to hash
        data: String,
        /// The hashing algorithm to use (sha256, sha512, double-sha256)
        #[arg(short, long, default_value = "sha256", value_parser = ["sha256", "sha512", "double-sha256"])]
        algo: String,
    },
    /// Encodes the input data using the specified format
    Encode {
        /// The input data to encode
        data: String,
        /// The encoding format to use (hex, base64, base64url)
        #[arg(short, long, default_value = "hex", value_parser = ["hex", "base64", "base64url"])]
        format: String,
    },
    /// Decodes the input data using the specified format
    Decode {
        /// The encoded string to decode
        data: String,
        /// The encoding format of the input data (hex, base64, base64url)
        #[arg(short, long, default_value = "hex", value_parser = ["hex", "base64", "base64url"])]
        format: String,
    },
    /// Conversions and formatting for XLM and Stroops
    Xlm {
        #[command(subcommand)]
        action: XlmCommands,
    },
}

#[derive(Subcommand)]
enum XlmCommands {
    /// Converts stroops to XLM
    ToXlm {
        /// The amount in stroops
        stroops: u64,
    },
    /// Converts XLM to stroops
    ToStroops {
        /// The amount in XLM
        xlm: f64,
    },
    /// Formats stroops as a readable XLM string
    Format {
        /// The amount in stroops
        stroops: u64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ValidateAddress { address } => match validate_address(&address) {
            Ok(_) => {
                println!("Address is valid: {}", address);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Commands::Hash { data, algo } => {
            let hashed = match algo.as_str() {
                "sha256" => sha256_hex(data.as_bytes()),
                "sha512" => sha512_hex(data.as_bytes()),
                "double-sha256" => double_sha256(data.as_bytes()),
                _ => unreachable!(),
            };
            println!("{}", hashed);
            process::exit(0);
        }
        Commands::Encode { data, format } => {
            let encoded = match format.as_str() {
                "hex" => to_hex(data.as_bytes()),
                "base64" => to_base64(data.as_bytes()),
                "base64url" => to_base64_url(data.as_bytes()),
                _ => unreachable!(),
            };
            println!("{}", encoded);
            process::exit(0);
        }
        Commands::Decode { data, format } => {
            let decoded_res = match format.as_str() {
                "hex" => from_hex(&data).map_err(|e| e.to_string()),
                "base64" => from_base64(&data).map_err(|e| e.to_string()),
                "base64url" => from_base64_url(&data).map_err(|e| e.to_string()),
                _ => unreachable!(),
            };

            match decoded_res {
                Ok(bytes) => {
                    let s = String::from_utf8_lossy(&bytes);
                    print!("{}", s);
                    if !s.ends_with('\n') {
                        println!();
                    }
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Xlm { action } => match action {
            XlmCommands::ToXlm { stroops } => {
                println!("{}", stroops_to_xlm(stroops));
                process::exit(0);
            }
            XlmCommands::ToStroops { xlm } => {
                println!("{}", xlm_to_stroops(xlm));
                process::exit(0);
            }
            XlmCommands::Format { stroops } => {
                println!("{}", format_xlm(stroops));
                process::exit(0);
            }
        },
    }
}
