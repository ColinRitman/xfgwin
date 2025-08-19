use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Genesis validation data for fork attack prevention
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkValidation {
    /// Fuego network ID (chain ID)
    pub fuego_network_id: u64,
    /// Network validation hash: keccak256(network_id)
    pub network_validation_hash: String, // hex string
}
/// Proof data file schema for XFG burn transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofDataFile {
    /// File metadata
    pub metadata: ProofMetadata,
    /// Cryptographic data for proof generation
    pub cryptographic_data: CryptographicData,
    /// User-facing data
    pub user_data: UserData,
    /// Security and validation
    pub security: SecurityData,
}

/// File metadata
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct ProofMetadata {
    /// Schema version
    pub version: String,
    /// Creation timestamp
    pub created_at: u64,
    /// File type identifier
    pub proof_type: String,
    /// Fuego transaction hash
    pub transaction_hash: String,
    /// File format version
    pub format_version: String,
}

/// Cryptographic data for proof generation
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct CryptographicData {
    /// 32-byte secret from XFG transaction extra field
    pub secret: String, // hex string
    /// Nullifier: keccak256(secret + "nullifier")
    pub nullifier: String, // hex string
    /// Commitment: keccak256(secret + "commitment")
    pub commitment: String, // hex string
    /// Block height where transaction occurred
    pub block_height: u64,
    /// XFG amount burned (in units)
    pub xfg_amount: u64,
    /// Transaction extra field hash
    pub tx_extra_hash: String, // hex string
}

/// User-facing data
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct UserData {
    /// Recipient Arbitrum address
    pub recipient_address: String,
    /// Recipient hash: keccak256(recipient_address)
    pub recipient_hash: String, // hex string
    /// HEAT amount to mint
    pub heat_amount: u64,
    /// Human-readable XFG amount
    pub xfg_amount_formatted: String, // "0.8 XFG"
    /// Human-readable HEAT amount
    pub heat_amount_formatted: String, // "8,000,000 HEAT"
    /// Transaction timestamp
    pub transaction_timestamp: u64,
}

/// Security and validation data
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityData {
    /// Cryptographic signature of the proof data
    pub signature: String, // hex string
    /// SHA256 checksum of all data
    pub checksum: String, // hex string
    /// Signature public key
    pub signature_pubkey: String, // hex string
    /// File integrity check
    pub integrity_hash: String, // hex string
    /// Genesis validation data
    pub network_validation: NetworkValidation,
}

impl ProofDataFile {
    /// Create a new proof data file
    pub fn new(
        transaction_hash: String,
        secret: [u8; 32],
        recipient_address: String,
        block_height: u64,
        xfg_amount: u64,
        fuego_network_id: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Calculate cryptographic hashes
        let nullifier = Self::calculate_nullifier(&secret)?;
        let commitment = Self::calculate_commitment(&secret)?;
        let recipient_hash = Self::calculate_recipient_hash(&recipient_address)?;
        let tx_extra_hash = Self::calculate_tx_extra_hash(&secret)?;
        
        // Calculate network validation hash
        let network_validation_hash = Self::calculate_network_validation_hash(
            fuego_network_id,
            &transaction_hash
        )?;

        // Format amounts for display
        let xfg_amount_formatted = format!("{:.1} XFG", xfg_amount as f64 / 10_000_000.0);
        let heat_amount = xfg_amount * 10; // 1 XFG = 10M HEAT
        let heat_amount_formatted = format!("{} HEAT", heat_amount.to_string().as_str());

        let mut proof_data = ProofDataFile {
            metadata: ProofMetadata {
                version: "1.0".to_string(),
                created_at: timestamp,
                proof_type: "XFG_BURN_TO_HEAT".to_string(),
                transaction_hash,
                format_version: "1.0".to_string(),
            },
            cryptographic_data: CryptographicData {
                secret: hex::encode(secret),
                nullifier: hex::encode(nullifier),
                commitment: hex::encode(commitment),
                block_height,
                xfg_amount,
                tx_extra_hash: hex::encode(tx_extra_hash),
            },
            user_data: UserData {
                recipient_address,
                recipient_hash: hex::encode(recipient_hash),
                heat_amount,
                xfg_amount_formatted,
                heat_amount_formatted,
                transaction_timestamp: timestamp,
            },
            security: SecurityData {
                signature: "".to_string(), // Will be calculated
                checksum: "".to_string(), // Will be calculated
                signature_pubkey: "".to_string(), // Will be set
                integrity_hash: "".to_string(), // Will be calculated
                network_validation: NetworkValidation {
                    fuego_network_id,
                    network_validation_hash: hex::encode(network_validation_hash),
                },
            },
        };

        // Calculate security data
        proof_data.calculate_security_data()?;

        Ok(proof_data)
    }

    /// Calculate nullifier hash
    fn calculate_nullifier(secret: &[u8; 32]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use sha3::{Keccak256, Digest};
        let mut hasher = Keccak256::new();
        hasher.update(secret);
        hasher.update(b"nullifier");
        let result = hasher.finalize();
        let mut nullifier = [0u8; 32];
        nullifier.copy_from_slice(&result);
        Ok(nullifier)
    }

    /// Calculate commitment hash
    fn calculate_commitment(secret: &[u8; 32]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use sha3::{Keccak256, Digest};
        let mut hasher = Keccak256::new();
        hasher.update(secret);
        hasher.update(b"commitment");
        let result = hasher.finalize();
        let mut commitment = [0u8; 32];
        commitment.copy_from_slice(&result);
        Ok(commitment)
    }

    /// Calculate recipient hash
    fn calculate_recipient_hash(recipient_address: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use sha3::{Keccak256, Digest};
        let mut hasher = Keccak256::new();
        hasher.update(recipient_address.as_bytes());
        let result = hasher.finalize();
        let mut recipient_hash = [0u8; 32];
        recipient_hash.copy_from_slice(&result);
        Ok(recipient_hash)
    }

    /// Calculate transaction extra hash
    fn calculate_tx_extra_hash(secret: &[u8; 32]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use sha3::{Keccak256, Digest};
        let mut hasher = Keccak256::new();
        hasher.update(secret);
        let result = hasher.finalize();
        let mut tx_extra_hash = [0u8; 32];
        tx_extra_hash.copy_from_slice(&result);
        Ok(tx_extra_hash)
    }

    /// Calculate network validation hash
    fn calculate_network_validation_hash(
        network_id: u64,
        genesis_tx: &str,
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use sha3::{Keccak256, Digest};
        let mut hasher = Keccak256::new();
        hasher.update(network_id.to_string().as_bytes());
        hasher.update(genesis_tx.as_bytes());
        let result = hasher.finalize();
        let mut network_hash = [0u8; 32];
        network_hash.copy_from_slice(&result);
        Ok(network_hash)
    }

    /// Calculate security data (signature, checksum, integrity hash)
    fn calculate_security_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use sha2::{Sha256, Digest};

        // Calculate checksum of all data (excluding security fields)
        let data_for_checksum = serde_json::to_string(&self.metadata)? + 
                               &serde_json::to_string(&self.cryptographic_data)? + 
                               &serde_json::to_string(&self.user_data)?;
        
        let mut hasher = Sha256::new();
        hasher.update(data_for_checksum.as_bytes());
        let checksum = hasher.finalize();
        self.security.checksum = hex::encode(checksum);

        // Calculate integrity hash
        let integrity_data = format!("{}{}{}{}", 
            self.metadata.transaction_hash,
            self.cryptographic_data.secret,
            self.user_data.recipient_address,
            self.security.checksum
        );
        
        let mut integrity_hasher = Sha256::new();
        integrity_hasher.update(integrity_data.as_bytes());
        let integrity_hash = integrity_hasher.finalize();
        self.security.integrity_hash = hex::encode(integrity_hash);

        // For now, use a placeholder signature (in production, this would be a real signature)
        self.security.signature = "placeholder_signature".to_string();
        self.security.signature_pubkey = "placeholder_pubkey".to_string();

        Ok(())
    }

    /// Validate the proof data file
    pub fn validate(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check version compatibility
        if self.metadata.version != "1.0" {
            return Err("Unsupported proof data version".into());
        }

        // Validate XFG amount (supports both 0.8 XFG and 8000 XFG)
        if !ProofDataFile::is_valid_xfg_amount(self.cryptographic_data.xfg_amount) {
            return Err(format!("Invalid XFG amount: {} (must be 8,000,000 or 80,000,000,000 units)", 
                self.cryptographic_data.xfg_amount).into());
        }

        // Validate recipient address format
        if !self.user_data.recipient_address.starts_with("0x") || 
           self.user_data.recipient_address.len() != 42 {
            return Err("Invalid recipient address format".into());
        }

        // Validate block height
        if self.cryptographic_data.block_height == 0 {
            return Err("Invalid block height".into());
        }

        // Validate transaction hash format
        if !self.metadata.transaction_hash.starts_with("0x") || 
           self.metadata.transaction_hash.len() != 66 {
            return Err("Invalid transaction hash format".into());
        }

        // Recalculate and verify checksum
        let expected_checksum = self.calculate_expected_checksum()?;
        if self.security.checksum != expected_checksum {
            return Err("Checksum validation failed".into());
        }

        // Validate network data
        let expected_network_hash = Self::calculate_network_validation_hash(
            self.security.network_validation.fuego_network_id,
            &self.metadata.transaction_hash
        )?;
        if self.security.network_validation.network_validation_hash != hex::encode(expected_network_hash) {
            return Err("Network validation hash mismatch".into());
        }

        Ok(true)
    }

    /// Validate XFG amount (supports both 0.8 XFG and 8000 XFG)
    pub fn is_valid_xfg_amount(amount: u64) -> bool {
        match amount {
            8_000_000 => true,           // 0.8 XFG
            80_000_000_000 => true,      // 8000 XFG
            _ => false
        }
    }

    /// Get XFG amount type description
    pub fn get_xfg_amount_type(amount: u64) -> &'static str {
        match amount {
            8_000_000 => "0.8 XFG (Standard)",
            80_000_000_000 => "8000 XFG (Large)",
            _ => "Invalid Amount"
        }
    }

    /// Calculate expected checksum for validation
    fn calculate_expected_checksum(&self) -> Result<String, Box<dyn std::error::Error>> {
        use sha2::{Sha256, Digest};
        let data_for_checksum = serde_json::to_string(&self.metadata)? + 
                               &serde_json::to_string(&self.cryptographic_data)? + 
                               &serde_json::to_string(&self.user_data)?;
        
        let mut hasher = Sha256::new();
        hasher.update(data_for_checksum.as_bytes());
        let checksum = hasher.finalize();
        Ok(hex::encode(checksum))
    }

    /// Get filename for the proof data file
    pub fn get_filename(&self) -> String {
        format!("xfg_burn_proof_{}.json", 
            self.metadata.transaction_hash[2..10].to_lowercase())
    }

    /// Get display information for user
    pub fn get_display_info(&self) -> String {
        format!(
            "XFG Burn Proof\n\
             Transaction: {}\n\
             Recipient: {}\n\
             Amount: {}\n\
             HEAT to Mint: {}\n\
             Block: {}\n\
             Created: {}",
            self.metadata.transaction_hash,
            self.user_data.recipient_address,
            self.user_data.xfg_amount_formatted,
            self.user_data.heat_amount_formatted,
            self.cryptographic_data.block_height,
            chrono::DateTime::from_timestamp(self.metadata.created_at as i64, 0)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_data_file_creation() {
        let secret = [0x42u8; 32];
        let recipient = "0xf8108826279b68504BDF5B3f056382E7Bf821CD0".to_string();
        let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
        
        let proof_data = ProofDataFile::new(
            tx_hash,
            secret,
            recipient,
            12345,
            8_000_000,
            12345, // Fuego network ID
        ).unwrap();

        assert_eq!(proof_data.metadata.version, "1.0");
        assert_eq!(proof_data.cryptographic_data.xfg_amount, 8_000_000);
        assert_eq!(proof_data.user_data.heat_amount, 80_000_000);
        assert_eq!(proof_data.security.network_validation.fuego_network_id, 12345);
        assert!(proof_data.validate().unwrap());
    }

    #[test]
    fn test_proof_data_file_creation_8000_xfg() {
        let secret = [0x42u8; 32];
        let recipient = "0xf8108826279b68504BDF5B3f056382E7Bf821CD0".to_string();
        let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
        
        let proof_data = ProofDataFile::new(
            tx_hash,
            secret,
            recipient,
            12345,
            80_000_000_000, // 8000 XFG
            12345, // Fuego network ID
        ).unwrap();

        assert_eq!(proof_data.metadata.version, "1.0");
        assert_eq!(proof_data.cryptographic_data.xfg_amount, 80_000_000_000);
        assert_eq!(proof_data.user_data.heat_amount, 800_000_000_000); // 800B HEAT
        assert_eq!(proof_data.security.network_validation.fuego_network_id, 12345);
        assert!(proof_data.validate().unwrap());
    }

    #[test]
    fn test_xfg_amount_validation() {
        assert!(ProofDataFile::is_valid_xfg_amount(8_000_000));      // 0.8 XFG
        assert!(ProofDataFile::is_valid_xfg_amount(80_000_000_000)); // 8000 XFG
        assert!(!ProofDataFile::is_valid_xfg_amount(1_000_000));     // Invalid
        assert!(!ProofDataFile::is_valid_xfg_amount(100_000_000_000)); // Invalid
    }

    #[test]
    fn test_xfg_amount_type_description() {
        assert_eq!(ProofDataFile::get_xfg_amount_type(8_000_000), "0.8 XFG (Standard)");
        assert_eq!(ProofDataFile::get_xfg_amount_type(80_000_000_000), "8000 XFG (Large)");
        assert_eq!(ProofDataFile::get_xfg_amount_type(1_000_000), "Invalid Amount");
    }

    #[test]
    fn test_filename_generation() {
        let secret = [0x42u8; 32];
        let recipient = "0xf8108826279b68504BDF5B3f056382E7Bf821CD0".to_string();
        let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
        
        let proof_data = ProofDataFile::new(
            tx_hash,
            secret,
            recipient,
            12345,
            8_000_000,
            12345,
        ).unwrap();

        let filename = proof_data.get_filename();
        assert_eq!(filename, "xfg_burn_proof_12345678.json");
    }
}
