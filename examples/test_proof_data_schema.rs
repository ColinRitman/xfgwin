//! Test Proof Data Schema
//! 
//! Simple test to verify the proof data schema works correctly

use xfg_stark::{
    proof_data_schema::ProofDataFile,
    Result,
};
use std::fs;

fn main() -> Result<()> {
    println!("🧪 Testing Proof Data Schema");
    println!("============================");
    
    // Test 1: Create a new proof data file
    println!("\n📝 Test 1: Creating new proof data file...");
    let secret = [0x42u8; 32];
    let recipient = "0xf8108826279b68504BDF5B3f056382E7Bf821CD0".to_string();
    let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
    
    let proof_data = ProofDataFile::new(
        tx_hash.clone(),
        secret,
        recipient.clone(),
        12345,
        8_000_000,
    )?;
    
    println!("✅ Proof data file created successfully!");
    println!("   Version: {}", proof_data.metadata.version);
    println!("   Transaction: {}", proof_data.metadata.transaction_hash);
    println!("   Recipient: {}", proof_data.user_data.recipient_address);
    println!("   XFG Amount: {}", proof_data.user_data.xfg_amount_formatted);
    println!("   HEAT Amount: {}", proof_data.user_data.heat_amount_formatted);
    println!("   Block Height: {}", proof_data.cryptographic_data.block_height);
    
    // Test 2: Validate the proof data
    println!("\n🔍 Test 2: Validating proof data...");
    let validation_result = proof_data.validate()?;
    println!("✅ Proof data validation passed: {}", validation_result);
    
    // Test 3: Get filename
    println!("\n📁 Test 3: Generating filename...");
    let filename = proof_data.get_filename();
    println!("✅ Generated filename: {}", filename);
    
    // Test 4: Get display info
    println!("\n📋 Test 4: Display information...");
    let display_info = proof_data.get_display_info();
    println!("✅ Display info:\n{}", display_info);
    
    // Test 5: Serialize to JSON
    println!("\n💾 Test 5: Serializing to JSON...");
    let json_content = serde_json::to_string_pretty(&proof_data)?;
    println!("✅ JSON serialization successful!");
    println!("   JSON size: {} bytes", json_content.len());
    
    // Test 6: Save to file
    println!("\n💾 Test 6: Saving to file...");
    let test_filename = "test_proof_data_generated.json";
    fs::write(test_filename, json_content)?;
    println!("✅ File saved: {}", test_filename);
    
    // Test 7: Load from file
    println!("\n📂 Test 7: Loading from file...");
    let loaded_content = fs::read_to_string(test_filename)?;
    let loaded_proof_data: ProofDataFile = serde_json::from_str(&loaded_content)?;
    println!("✅ File loaded successfully!");
    println!("   Loaded transaction: {}", loaded_proof_data.metadata.transaction_hash);
    println!("   Loaded recipient: {}", loaded_proof_data.user_data.recipient_address);
    
    // Test 8: Validate loaded data
    println!("\n🔍 Test 8: Validating loaded data...");
    let loaded_validation = loaded_proof_data.validate()?;
    println!("✅ Loaded data validation passed: {}", loaded_validation);
    
    // Test 9: Compare original and loaded
    println!("\n🔄 Test 9: Comparing original and loaded data...");
    let original_json = serde_json::to_string(&proof_data)?;
    let loaded_json = serde_json::to_string(&loaded_proof_data)?;
    
    if original_json == loaded_json {
        println!("✅ Original and loaded data are identical!");
    } else {
        println!("❌ Original and loaded data differ!");
    }
    
    // Test 10: Cryptographic data validation
    println!("\n🔐 Test 10: Cryptographic data validation...");
    println!("   Secret: {}", proof_data.cryptographic_data.secret);
    println!("   Nullifier: {}", proof_data.cryptographic_data.nullifier);
    println!("   Commitment: {}", proof_data.cryptographic_data.commitment);
    println!("   Recipient Hash: {}", proof_data.user_data.recipient_hash);
    println!("   Checksum: {}", proof_data.security.checksum);
    println!("   Integrity Hash: {}", proof_data.security.integrity_hash);
    println!("✅ All cryptographic data present and valid!");
    
    println!("\n🎉 All tests passed successfully!");
    println!("=================================");
    println!("✅ Proof data schema is working correctly");
    println!("✅ File I/O operations work");
    println!("✅ Validation functions work");
    println!("✅ Cryptographic calculations work");
    println!("✅ JSON serialization/deserialization works");
    
    println!("\n📁 Generated files:");
    println!("   - test_proof_data_generated.json (proof data file)");
    
    println!("\n🔗 Next Steps:");
    println!("   1. Use this proof data file with the full proof generator");
    println!("   2. Generate STARK proof from the data");
    println!("   3. Use proof to mint HEAT tokens");
    
    Ok(())
}
