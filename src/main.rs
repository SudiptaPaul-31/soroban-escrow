use clap::{Parser, Subcommand};
use soroban_toolkit::address::{detect_address_type, mask_address, validate_address, AddressType};
use std::process;

#[derive(Parser)]
#[command(name = "soroban-toolkit")]
#[command(about = "Soroban utility toolkit", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Address utilities
    Address {
        #[command(subcommand)]
        action: AddressCommands,
    },
}

#[derive(Subcommand)]
enum AddressCommands {
    /// Validates a Stellar/Soroban address
    Validate {
        /// The Stellar address to validate
        address: String,
    },
    /// Masks a Stellar/Soroban address showing only first 4 and last 4 characters
    Mask {
        /// The Stellar address to mask
        address: String,
    },
    /// Detects the type of a Stellar/Soroban address (Account or Contract)
    Detect {
        /// The Stellar address to detect
        address: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Address { action } => match action {
            AddressCommands::Validate { address } => match validate_address(&address) {
                Ok(_) => {
                    println!("Address is valid: {}", address);
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
            AddressCommands::Mask { address } => match validate_address(&address) {
                Ok(_) => {
                    println!("{}", mask_address(&address));
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
            AddressCommands::Detect { address } => match detect_address_type(&address) {
                AddressType::Account => {
                    println!("Account");
                    process::exit(0);
                }
                AddressType::Contract => {
                    println!("Contract");
                    process::exit(0);
                }
                AddressType::Invalid => {
                    eprintln!("Error: Address is invalid");
                    process::exit(1);
                }
            },
        },
    }
}
