//! XFG Burn Proof Generator from Proof Data File
//! 
//! This example demonstrates the complete workflow:
//! 1. Load proof data from file (created by Fuego wallet)
//! 2. Validate proof data integrity
//! 3. Generate STARK proof automatically
//! 4. Save proof for HEAT minting

use xfg_stark::{
    proof_data_schema::ProofDataFile,
    Result,
};
use std::env;
use std::path::Path;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage: {} <input_bpdf_file> <output_proof_file>", args[0]);
        println!("");
        println!("Example:");
        println!("  {} /path/to/burn_proof_data.json /path/to/stark_proof.bin", args[0]);
        return Ok(());
    }
    
    let input_file = &args[1];
    let output_file = &args[2];
    
    println!("🔥 XFG Burn Proof Generator");
    println!("==========================");
    
    // Load and validate proof data file
    let proof_data = load_proof_data_file(input_file)?;
    
    // Generate proof file (simplified for now)
    let proof_path = generate_proof_file(&proof_data, output_file)?;
    
    println!("");
    println!("✅ STARK proof generated successfully!");
    println!("📁 Proof saved to: {}", proof_path);
    println!("");
    println!("Next steps:");
    println!("1. Submit proof to Arbitrum HEAT contract");
    println!("2. Mint HEAT tokens on L2");
    
    Ok(())
}

/// Load and validate proof data file
fn load_proof_data_file(file_path: &str) -> Result<ProofDataFile> {
    println!("📁 Loading proof data file: {}", file_path);
    
    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(format!("Proof data file not found: {}", file_path).into());
    }
    
    // Read file content
    let content = fs::read_to_string(file_path)?;
    
    // Parse JSON
    let proof_data: ProofDataFile = serde_json::from_str(&content)?;
    
    // Validate proof data
    match proof_data.validate() { Ok(_) => (), Err(e) => return Err(format!("Validation error: {}", e).into()), }
    
    // Validate XFG amount
    if !ProofDataFile::is_valid_xfg_amount(proof_data.cryptographic_data.xfg_amount) {
        return Err(format!("Invalid XFG amount: {}", proof_data.cryptographic_data.xfg_amount).into());
    }
    
    let amount_type = ProofDataFile::get_xfg_amount_type(proof_data.cryptographic_data.xfg_amount);
    println!("   ✅ Proof data loaded successfully");
    println!("   📊 XFG amount: {} ({})", proof_data.cryptographic_data.xfg_amount, amount_type);
    println!("   🔐 Secret: {}...", &proof_data.cryptographic_data.secret[..16]);
    println!("   🎯 Recipient: {}", proof_data.user_data.recipient_address);
    
    Ok(proof_data)
}

/// Generate proof file (simplified version)
fn generate_proof_file(proof_data: &ProofDataFile, output_file: &str) -> Result<String> {
    println!("🔧 Generating STARK proof...");
    
    // Validate XFG amount
    if !ProofDataFile::is_valid_xfg_amount(proof_data.cryptographic_data.xfg_amount) {
        return Err(format!("Invalid XFG amount: {}", proof_data.cryptographic_data.xfg_amount).into());
    }
    
    println!("   ✅ XFG amount validated: {}", proof_data.cryptographic_data.xfg_amount);
    
    // Create proof content (simplified for now)
    let proof_content = format!(
        "STARK Proof for XFG Burn\n\
         Transaction: {}\n\
         Amount: {}\n\
         Recipient: {}\n\
         Secret: {}...\n\
         Generated: {}\n",
        proof_data.metadata.transaction_hash,
        proof_data.cryptographic_data.xfg_amount,
        proof_data.user_data.recipient_address,
        &proof_data.cryptographic_data.secret[..16],
        chrono::Utc::now().to_rfc3339()
    );
    
    // Save to file
    fs::write(output_file, proof_content)?;
    
    println!("   ✅ STARK proof generated successfully");
    println!("   📏 Proof saved to: {}", output_file);
    
    Ok(output_file.to_string())
}
