use xfg_stark::{
    xfg_rpc_validator::{XFGRPCValidator, ProofData},
    proof_data_schema::ProofDataFile,
    Result,
};
use std::env;
use std::process;
use tokio;
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔥 XFG to HEAT Complete Workflow CLI");
    println!("=====================================");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "validate" => {
            if args.len() < 3 {
                println!("❌ Error: Missing transaction hash for validation");
                print_usage();
                process::exit(1);
            }
            let tx_hash = &args[2];
            validate_transaction(tx_hash).await?;
        }
        "generate-proof" => {
            if args.len() < 4 {
                println!("❌ Error: Missing arguments for proof generation");
                println!("Usage: {} generate-proof <tx_hash> <recipient_address>", args[0]);
                process::exit(1);
            }
            let tx_hash = &args[2];
            let recipient = &args[3];
            generate_proof(tx_hash, recipient).await?;
        }
        "mint-heat" => {
            if args.len() < 3 {
                println!("❌ Error: Missing proof file for minting");
                println!("Usage: {} mint-heat <proof_file.json>", args[0]);
                process::exit(1);
            }
            let proof_file = &args[2];
            mint_heat(proof_file).await?;
        }
        "full-workflow" => {
            if args.len() < 4 {
                println!("❌ Error: Missing arguments for full workflow");
                println!("Usage: {} full-workflow <tx_hash> <recipient_address>", args[0]);
                process::exit(1);
            }
            let tx_hash = &args[2];
            let recipient = &args[3];
            full_workflow(tx_hash, recipient).await?;
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            println!("❌ Error: Unknown command '{}'", command);
            print_usage();
            process::exit(1);
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("\n📖 Usage:");
    println!("  {} validate <tx_hash>                    - Validate XFG burn transaction", env::args().nth(0).unwrap());
    println!("  {} generate-proof <tx_hash> <recipient>  - Generate proof data file", env::args().nth(0).unwrap());
    println!("  {} mint-heat <proof_file.json>           - Mint HEAT tokens on Arbitrum", env::args().nth(0).unwrap());
    println!("  {} full-workflow <tx_hash> <recipient>   - Complete XFG to HEAT workflow", env::args().nth(0).unwrap());
    println!("  {} help                                  - Show this help message", env::args().nth(0).unwrap());
    
    println!("\n🔧 Environment Variables:");
    println!("  FUEGO_RPC_URL=<url>                      - Fuego RPC endpoint (default: http://localhost:18081/json_rpc)");
    println!("  FUEGO_GENESIS_TX=<hash>                  - Expected genesis transaction hash");
    println!("  ARBITRUM_RPC_URL=<url>                   - Arbitrum RPC endpoint");
    println!("  PRIVATE_KEY=<key>                        - Private key for Arbitrum transactions");
    
    println!("\n📋 Example:");
    println!("  export FUEGO_RPC_URL=\"http://localhost:18081/json_rpc\"");
    println!("  export FUEGO_GENESIS_TX=\"0x1234567890abcdef...\"");
    println!("  export ARBITRUM_RPC_URL=\"https://sepolia-rollup.arbitrum.io/rpc\"");
    println!("  export PRIVATE_KEY=\"0x1234567890abcdef...\"");
    println!("  {} full-workflow 0x1234567890abcdef... 0xf8108826279b68504BDF5B3f056382E7Bf821CD0", env::args().nth(0).unwrap());
}

async fn validate_transaction(tx_hash: &str) -> Result<()> {
    println!("🔍 Step 1: Validating XFG burn transaction...");
    println!("   Transaction Hash: {}", tx_hash);
    
    // Create RPC validator
    let mut validator = XFGRPCValidator::default();
    
    // Override with environment variables
    if let Ok(rpc_url) = env::var("FUEGO_RPC_URL") {
        validator.rpc_url = rpc_url;
    }
    if let Ok(genesis_tx) = env::var("FUEGO_GENESIS_TX") {
        validator.genesis_transaction_hash = genesis_tx;
    }
    
    // Validate transaction with genesis check
    match validator.check_undefined_output_with_genesis(tx_hash).await {
        Ok(true) => {
            println!("✅ Transaction validation passed!");
            println!("   - Undefined output found");
            println!("   - Genesis transaction validated");
            println!("   - Ready for proof generation");
        }
        Ok(false) => {
            println!("❌ Transaction validation failed!");
            println!("   - Either no undefined output found");
            println!("   - Or genesis transaction validation failed");
            process::exit(1);
        }
        Err(e) => {
            println!("❌ Error during validation: {}", e);
            println!("   This might be because:");
            println!("   - Fuego daemon is not running");
            println!("   - RPC endpoint is not accessible");
            println!("   - Network connectivity issues");
            process::exit(1);
        }
    }
    
    Ok(())
}

async fn generate_proof(tx_hash: &str, recipient: &str) -> Result<()> {
    println!("📊 Step 2: Generating proof data file...");
    println!("   Transaction Hash: {}", tx_hash);
    println!("   Recipient Address: {}", recipient);
    
    // Create RPC validator
    let mut validator = XFGRPCValidator::default();
    
    // Override with environment variables
    if let Ok(rpc_url) = env::var("FUEGO_RPC_URL") {
        validator.rpc_url = rpc_url;
    }
    if let Ok(genesis_tx) = env::var("FUEGO_GENESIS_TX") {
        validator.genesis_transaction_hash = genesis_tx;
    }
    
    // Extract proof data with genesis validation
    let proof_data = match validator.extract_proof_data_with_genesis(tx_hash).await {
        Ok(data) => data,
        Err(e) => {
            println!("❌ Failed to extract proof data: {}", e);
            process::exit(1);
        }
    };
    
    // Create proof data file
    let genesis_tx = validator.genesis_transaction_hash.clone();
    let genesis_block = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(); // Placeholder
    let genesis_timestamp = 1640995200; // Placeholder
    let fuego_network_id = 12345; // Placeholder
    
    let proof_file = match ProofDataFile::new(
        proof_data.transaction_hash,
        proof_data.secret,
        recipient.to_string(),
        proof_data.block_height,
        proof_data.amount,
        genesis_tx,
        genesis_block,
        genesis_timestamp,
        fuego_network_id,
    ) {
        Ok(file) => file,
        Err(e) => {
            println!("❌ Failed to create proof data file: {}", e);
            process::exit(1);
        }
    };
    
    // Save proof data file
    let filename = proof_file.get_filename();
    let json_data = serde_json::to_string_pretty(&proof_file)?;
    std::fs::write(&filename, json_data)?;
    
    println!("✅ Proof data file generated successfully!");
    println!("   Filename: {}", filename);
    println!("   XFG Amount: {}", proof_file.user_data.xfg_amount_formatted);
    println!("   HEAT Amount: {}", proof_file.user_data.heat_amount_formatted);
    println!("   Block Height: {}", proof_file.cryptographic_data.block_height);
    println!("   Genesis Validated: ✅");
    
    Ok(())
}

async fn mint_heat(proof_file: &str) -> Result<()> {
    println!("🔥 Step 3: Minting HEAT tokens on Arbitrum...");
    println!("   Proof File: {}", proof_file);
    
    // Check if proof file exists
    if !std::path::Path::new(proof_file).exists() {
        println!("❌ Error: Proof file '{}' not found", proof_file);
        process::exit(1);
    }
    
    // Load proof data
    let proof_data: ProofDataFile = {
        let content = std::fs::read_to_string(proof_file)?;
        serde_json::from_str(&content)?
    };
    
    // Validate proof data
    match proof_data.validate() {
        Ok(true) => {
            println!("✅ Proof data validation passed!");
        }
        Ok(false) => {
            println!("❌ Proof data validation failed!");
            process::exit(1);
        }
        Err(e) => {
            println!("❌ Error validating proof data: {}", e);
            process::exit(1);
        }
    }
    
    // Check environment variables
    let arbitrum_rpc = env::var("ARBITRUM_RPC_URL")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());
    let private_key = env::var("PRIVATE_KEY")
        .unwrap_or_else(|_| {
            println!("❌ Error: PRIVATE_KEY environment variable not set");
            println!("   Please set your private key: export PRIVATE_KEY=\"0x1234567890abcdef...\"");
            process::exit(1);
        });
    
    println!("   Arbitrum RPC: {}", arbitrum_rpc);
    println!("   Recipient: {}", proof_data.user_data.recipient_address);
    println!("   HEAT Amount: {}", proof_data.user_data.heat_amount_formatted);
    
    // TODO: Implement actual smart contract interaction
    // For now, we'll simulate the minting process
    println!("🔄 Simulating HEAT minting...");
    println!("   This would call the HEATBurnProofVerifier.claimHEAT() function");
    println!("   with the following parameters:");
    println!("   - Secret: 0x{}", proof_data.cryptographic_data.secret);
    println!("   - Nullifier: 0x{}", proof_data.cryptographic_data.nullifier);
    println!("   - Commitment: 0x{}", proof_data.cryptographic_data.commitment);
    println!("   - Recipient Hash: 0x{}", proof_data.user_data.recipient_hash);
    println!("   - Genesis TX: {}", proof_data.security.genesis_validation.genesis_transaction_hash);
    
    println!("✅ HEAT minting simulation completed!");
    println!("   {} HEAT tokens would be minted to {}", 
        proof_data.user_data.heat_amount_formatted, 
        proof_data.user_data.recipient_address);
    
    Ok(())
}

async fn full_workflow(tx_hash: &str, recipient: &str) -> Result<()> {
    println!("🚀 Starting complete XFG to HEAT workflow...");
    println!("=============================================");
    
    // Step 1: Validate transaction
    validate_transaction(tx_hash).await?;
    
    // Step 2: Generate proof
    generate_proof(tx_hash, recipient).await?;
    
    // Step 3: Mint HEAT
    let filename = format!("xfg_burn_proof_{}.json", tx_hash[2..10].to_lowercase());
    mint_heat(&filename).await?;
    
    println!("\n🎉 Complete workflow finished successfully!");
    println!("=============================================");
    println!("✅ XFG burn transaction validated");
    println!("✅ Genesis transaction validated");
    println!("✅ Proof data file generated");
    println!("✅ HEAT tokens ready for minting");
    println!("\n📁 Next steps:");
    println!("   1. Deploy contracts to Arbitrum Sepolia");
    println!("   2. Set real genesis transaction hash");
    println!("   3. Run actual minting transaction");
    println!("   4. Verify HEAT balance on Arbitrum");
    
    Ok(())
}
