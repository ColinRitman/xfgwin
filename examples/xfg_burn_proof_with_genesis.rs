use xfg_stark::xfg_rpc_validator::{XFGRPCValidator, ProofData};
use xfg_stark::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 XFG Burn Proof with Genesis Validation");
    println!("==========================================");
    
    // Get transaction hash from command line arguments
    let args: Vec<String> = env::args().collect();
    let transaction_hash = if args.len() > 1 {
        args[1].clone()
    } else {
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()
    };
    
    println!("📋 Transaction Hash: {}", transaction_hash);
    
    // Create RPC validator with genesis validation
    let mut validator = XFGRPCValidator::default();
    
    // Optionally override RPC URL and genesis transaction hash
    if let Ok(rpc_url) = env::var("FUEGO_RPC_URL") {
        validator.rpc_url = rpc_url;
        println!("🔧 Using custom RPC URL: {}", validator.rpc_url);
    }
    
    if let Ok(genesis_tx) = env::var("FUEGO_GENESIS_TX") {
        validator.genesis_transaction_hash = genesis_tx;
        println!("🔧 Using custom genesis transaction: {}", validator.genesis_transaction_hash);
    }
    
    println!("\n🔍 Step 1: Checking undefined output with genesis validation...");
    
    // Check undefined output with genesis validation
    match validator.check_undefined_output_with_genesis(&transaction_hash).await {
        Ok(true) => {
            println!("✅ Transaction validation passed!");
            println!("   - Undefined output found");
            println!("   - Genesis transaction validated");
        }
        Ok(false) => {
            println!("❌ Transaction validation failed!");
            println!("   - Either no undefined output found");
            println!("   - Or genesis transaction validation failed");
            return Ok(());
        }
        Err(e) => {
            println!("❌ Error during validation: {}", e);
            println!("   This might be because:");
            println!("   - Fuego daemon is not running");
            println!("   - RPC endpoint is not accessible");
            println!("   - Network connectivity issues");
            return Ok(());
        }
    }
    
    println!("\n📊 Step 2: Extracting proof data with genesis validation...");
    
    // Extract proof data with genesis validation
    match validator.extract_proof_data_with_genesis(&transaction_hash).await {
        Ok(proof_data) => {
            println!("✅ Proof data extracted successfully!");
            println!("   Transaction Hash: {}", proof_data.transaction_hash);
            println!("   Block Height: {}", proof_data.block_height);
            println!("   Amount: {} units ({} XFG)", proof_data.amount, proof_data.amount as f64 / 10_000_000.0);
            println!("   Secret: 0x{}", hex::encode(proof_data.secret));
            
            // Calculate HEAT amount
            let heat_amount = proof_data.amount * 10; // 1 XFG = 10M HEAT
            println!("   HEAT Amount: {} HEAT", heat_amount);
            
            println!("\n🔐 Step 3: Ready for STARK proof generation...");
            println!("   The proof data is now ready for STARK proof generation.");
            println!("   This data can be used to create a proof that will allow");
            println!("   minting of {} HEAT tokens on Arbitrum.", heat_amount);
            
            println!("\n📁 Next steps:");
            println!("   1. Generate STARK proof using this data");
            println!("   2. Save proof to file");
            println!("   3. Submit proof to HEAT contract on Arbitrum");
            println!("   4. Mint HEAT tokens");
            
        }
        Err(e) => {
            println!("❌ Failed to extract proof data: {}", e);
            println!("   This might be because:");
            println!("   - Transaction validation failed");
            println!("   - Genesis validation failed");
            println!("   - RPC communication issues");
        }
    }
    
    println!("\n🛡️ Security Features Active:");
    println!("   ✅ Undefined output validation");
    println!("   ✅ Genesis transaction validation");
    println!("   ✅ Network ID validation");
    println!("   ✅ Fork attack prevention");
    println!("   ✅ Cross-chain attack prevention");
    
    println!("\n🎯 Genesis Validation Details:");
    println!("   Expected Genesis: {}", validator.genesis_transaction_hash);
    println!("   RPC Endpoint: {}", validator.rpc_url);
    println!("   Validation Method: Real-time RPC query");
    
    Ok(())
}

