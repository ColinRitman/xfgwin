use clap::{App, Arg};
use xfg_stark::{
    proof_data_schema::{StarkProofDataPackage, CompleteProofPackage, StarkProof, EldernodeVerification, ProofDataTemplate},
    burn_mint_prover::XfgBurnMintProver,
    burn_mint_verifier::{XfgBurnMintVerifier, VerificationResult},
    XfgStarkError,
    Result,
};

fn main() -> Result<()> {
    let matches = App::new("xfg-stark-cli")
        .version("1.0")
        .about("CLI tool for managing STARK proof data packages")
        .subcommand(
            App::new("generate")
                .about("Generate a STARK proof from a data package")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Input data package file")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output proof file")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            App::new("validate")
                .about("Validate a data package")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Input data package file")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            App::new("create-template")
                .about("Create a template data package")
                .arg(
                    Arg::new("burn-amount")
                        .short('a')
                        .long("burn-amount")
                        .value_name("AMOUNT")
                        .help("Burn amount in XFG")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output template file")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            App::new("create-package")
                .about("Create a data package from a template")
                .arg(
                    Arg::new("template")
                        .short('t')
                        .long("template")
                        .value_name("FILE")
                        .help("Template file")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("txn-hash")
                        .short('x')
                        .long("txn-hash")
                        .value_name("HASH")
                        .help("Fuego transaction hash (no 0x prefix)")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("recipient")
                        .short('r')
                        .long("recipient")
                        .value_name("ADDRESS")
                        .help("Recipient Ethereum address")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output package file")
                        .required(true)
                        .takes_value(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", args)) => {
            let input_file = args.get_one::<String>("input").unwrap();
            let output_file = args.get_one::<String>("output").unwrap();
            generate_proof(input_file, output_file)?;
        }
        Some(("validate", args)) => {
            let input_file = args.get_one::<String>("input").unwrap();
            validate_package(input_file)?;
        }
        Some(("create-template", args)) => {
            let _burn_amount = args.get_one::<f64>("burn-amount").unwrap();
            let output_file = args.get_one::<String>("output").unwrap();
            create_template(output_file)?;
        }
        Some(("create-package", args)) => {
            let _template_file = args.get_one::<String>("template").unwrap();
            let txn_hash = args.get_one::<String>("txn-hash").unwrap();
            let recipient = args.get_one::<String>("recipient").unwrap();
            let output_file = args.get_one::<String>("output").unwrap();
            create_package(txn_hash, recipient, output_file)?;
        }
        _ => {
            eprintln!("Unknown subcommand. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Generate STARK proof from data package using real prover
fn generate_proof(input_file: &str, output_file: &str) -> Result<()> {
    println!("🔍 Loading data package from: {}", input_file);

    // Load and validate data package
    let package = StarkProofDataPackage::load_from_file(input_file)
        .map_err(|e| XfgStarkError::ParseError(e.to_string()))?;

    let validation = package.validate();

    if !validation.is_valid {
        eprintln!("❌ Data package validation failed:");
        for error in &validation.errors {
            eprintln!("   - {}", error);
        }
        std::process::exit(1);
    }

    if !validation.warnings.is_empty() {
        println!("⚠️  Warnings:");
        for warning in &validation.warnings {
            println!("   - {}", warning);
        }
    }

    println!("✅ Data package validated successfully");
    println!("📊 Burn amount: {} XFG ({} atomic units)",
             package.burn_transaction.burn_amount_xfg,
             package.burn_transaction.burn_amount_atomic);
    println!("🎯 Mint amount: {} HEAT", package.get_mint_amount_heat());

    // Create real prover
    println!("🔐 Creating STARK prover...");
    let prover = XfgBurnMintProver::new(128);

    // Convert transaction hash from hex string to u64
    let txn_hash_u64 = hex_to_u64(&package.burn_transaction.transaction_hash)
        .map_err(|e| XfgStarkError::ParseError(format!("Invalid transaction hash: {}", e)))?;

    // Convert Ethereum address to bytes
    let recipient_bytes = hex_to_bytes(&package.recipient.ethereum_address)
        .map_err(|e| XfgStarkError::ParseError(format!("Invalid recipient address: {}", e)))?;

    // Convert secret to bytes
    let secret_bytes = package.secret.secret_key.as_bytes();

    // Generate real STARK proof
    println!("⚡ Generating STARK proof...");
    let winterfell_proof = prover.prove_burn_mint(
        package.burn_transaction.burn_amount_atomic,
        package.get_mint_amount_atomic(),
        txn_hash_u64,
        &recipient_bytes,
        secret_bytes,
    ).map_err(|e| XfgStarkError::CryptoError(format!("Proof generation failed: {}", e)))?;

    println!("✅ STARK proof generated successfully");

    // Convert Winterfell proof to our format
    let proof_data = winterfell_proof.to_bytes();
    println!("📏 Proof size: {} bytes", proof_data.len());

    let proof = StarkProof {
        proof_data: proof_data.clone(),
        public_inputs: xfg_stark::proof_data_schema::StarkPublicInputs {
            burn_amount: package.burn_transaction.burn_amount_atomic,
            mint_amount: package.get_mint_amount_atomic(),
            txn_hash: package.burn_transaction.transaction_hash.clone(),
            recipient_hash: package.recipient.ethereum_address.clone(),
            state: 0,
        },
        metadata: xfg_stark::proof_data_schema::ProofMetadata {
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            description: format!("STARK proof for {} XFG burn", package.burn_transaction.burn_amount_xfg),
            network: package.metadata.network.clone(),
        },
    };

    // Save proof
    let json = serde_json::to_string_pretty(&proof)
        .map_err(|e| XfgStarkError::JsonError(e))?;

    std::fs::write(output_file, json)
        .map_err(|e| XfgStarkError::IoError(e))?;

    println!("�� Proof saved to: {}", output_file);
    println!("🚀 Ready for submission to HEAT mint contract!");

    Ok(())
}

/// Validate data package with enhanced Fuego blockchain validation
fn validate_package(input_file: &str) -> Result<()> {
    println!("🔍 Loading data package from: {}", input_file);

    let package = StarkProofDataPackage::load_from_file(input_file)
        .map_err(|e| XfgStarkError::ParseError(e.to_string()))?;

    println!("�� Package Information:");
    println!("   Version: {}", package.metadata.version);
    println!("   Network: {}", package.metadata.network);
    println!("   Created: {}", package.metadata.created_at);
    println!("   Description: {}", package.metadata.description);

    println!("\n🔥 Burn Transaction:");
    println!("   Hash: {}", package.burn_transaction.transaction_hash);
    println!("   Amount: {} XFG ({} atomic units)",
             package.burn_transaction.burn_amount_xfg,
             package.burn_transaction.burn_amount_atomic);
    println!("   Block Height: {}", package.burn_transaction.block_height);
    println!("   Timestamp: {}", package.burn_transaction.timestamp);

    println!("\n👤 Recipient:");
    println!("   Address: {}", package.recipient.ethereum_address);
    if let Some(ref ens) = package.recipient.ens_name {
        println!("   ENS: {}", ens);
    }
    if let Some(ref label) = package.recipient.label {
        println!("   Label: {}", label);
    }

    println!("\n🔐 Secret:");
    println!("   Key: {}...", &package.secret.secret_key[..8.min(package.secret.secret_key.len())]);
    if let Some(ref salt) = package.secret.salt {
        println!("   Salt: {}", salt);
    }
    if let Some(ref hint) = package.secret.hint {
        println!("   Hint: {}", hint);
    }

    println!("\n📊 Validation Results:");

    let validation = package.validate();
    if validation.is_valid {
        println!("   ✅ Package is valid");
    } else {
        println!("   ❌ Package has errors:");
        for error in &validation.errors {
            println!("      - {}", error);
        }
        for warning in &validation.warnings {
            println!("      - {}", warning);
        }
    }

    // Additional Fuego blockchain validation
    println!("\n🔗 Fuego Blockchain Validation:");
    validate_fuego_transaction(&package)?;

    Ok(())
}

/// Validate Fuego blockchain transaction details
fn validate_fuego_transaction(package: &StarkProofDataPackage) -> Result<()> {
    // Validate transaction hash format (Fuego native format - no 0x prefix)
    if package.burn_transaction.transaction_hash.starts_with("0x") {
        println!("   ❌ Transaction hash should not have 0x prefix for Fuego");
        return Err(XfgStarkError::ParseError("Invalid Fuego transaction hash format".to_string()));
    }

    // Validate transaction hash length (Fuego uses 32-byte hashes, 64 hex chars)
    if package.burn_transaction.transaction_hash.len() != 64 {
        println!("   ❌ Transaction hash should be 64 hex characters for Fuego");
        return Err(XfgStarkError::ParseError("Invalid Fuego transaction hash length".to_string()));
    }

    // Validate block height is after XFG burn implementation (800,000+)
    if package.burn_transaction.block_height < 800_000 {
        println!("   ❌ Block height {} is before XFG burn implementation (800,000)", package.burn_transaction.block_height);
        return Err(XfgStarkError::ParseError("Block height must be after XFG burn implementation (800,000+)".to_string()));
    }

    // Validate network ID format
    if package.burn_transaction.network_id.is_empty() {
        println!("   ❌ Network ID is required");
        return Err(XfgStarkError::ParseError("Network ID cannot be empty".to_string()));
    }

    println!("   ✅ Fuego blockchain validation passed");
    Ok(())
}

fn create_template(output_file: &str) -> Result<()> {
    let template = ProofDataTemplate::standard_burn();

    let json = serde_json::to_string_pretty(&template)
        .map_err(|e| XfgStarkError::JsonError(e))?;

    std::fs::write(output_file, json)
        .map_err(|e| XfgStarkError::IoError(e))?;

    println!("📝 Template created: {}", output_file);
    println!("📋 Template: {}", template.name);
    println!("📖 Description: {}", template.description);

    Ok(())
}

fn create_package(
    txn_hash: &str,
    recipient: &str,
    output_file: &str,
) -> Result<()> {
    // Parse burn amount
    let burn_amount_f64: f64 = 0.8; // Default to standard burn

    // Validate burn amount
    if burn_amount_f64 != 0.8 && burn_amount_f64 != 800.0 {
        eprintln!("❌ Burn amount must be exactly 0.8 or 800.0 XFG");
        std::process::exit(1);
    }

    // Create package
    let package = StarkProofDataPackage::new(
        burn_amount_f64,
        txn_hash.to_string(),
        recipient.to_string(),
        "dummy_secret_key".to_string(),
        "fuego-mainnet".to_string(),
    );

    // Save package
    package.save_to_file(output_file)?;

    println!("📦 Data package created: {}", output_file);
    println!("🔥 Burn amount: {} XFG", burn_amount_f64);
    println!("🎯 Mint amount: {} HEAT", package.get_mint_amount_heat());
    println!("�� Transaction: {}", txn_hash);
    println!("👤 Recipient: {}", recipient);
    println!("🌐 Network: fuego-mainnet");

    println!("\n💡 Next steps:");
    println!("   1. Edit {} to add block height and timestamp", output_file);
    println!("   2. Run: xfg-stark-cli validate -i {}", output_file);
    println!("   3. Run: xfg-stark-cli generate -i {} -o proof.json", output_file);

    Ok(())
}

// Helper functions for hex conversion
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

fn hex_to_u64(hex: &str) -> Result<u64, XfgStarkError> {
    let bytes = hex_to_bytes(hex)
        .map_err(|e| XfgStarkError::ParseError(format!("Invalid hex string: {}", e)))?;
    
    if bytes.len() < 8 {
        return Err(XfgStarkError::ParseError("Hex string too short for u64".to_string()));
    }
    
    let mut u64_bytes = [0u8; 8];
    u64_bytes.copy_from_slice(&bytes[0..8]);
    Ok(u64::from_le_bytes(u64_bytes))
}
