//! Proof Data Schema for XFG STARK Proofs
//! 
//! This module defines the data structures and validation logic for XFG burn proof data files.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use sha3::{Keccak256, Digest as Sha3Digest};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use hex;

/// Main proof data file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofDataFile {
    pub metadata: ProofMetadata,
    pub cryptographic_data: CryptographicData,
    pub user_data: UserData,
    pub security: SecurityData,
}

/// Proof metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub version: String,
    pub created_at: u64,
    pub proof_type: String,
    pub transaction_hash: String,
    pub format_version: String,
}

/// Cryptographic data for the proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicData {
    pub secret: String,
    pub commitment: String,
    pub nullifier: String,
    pub xfg_amount: u64,
    pub block_height: u64,
}

/// User data for the proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub recipient_address: String,
    pub xfg_amount_formatted: String,
    pub heat_amount_formatted: String,
}

/// Security and validation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityData {
    pub checksum: String,
    pub integrity_hash: String,
    pub signature: String,
    pub signature_pubkey: String,
    pub network_validation: NetworkValidation,
}

/// Network validation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkValidation {
    pub fuego_network_id: u64,
    pub network_validation_hash: String,
}

impl ProofDataFile {
    /// Create a new proof data file
    pub fn new(
        secret: [u8; 32],
        recipient: String,
        xfg_amount: u64,
        tx_hash: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let metadata = ProofMetadata {
            version: "1.0".to_string(),
            created_at: now,
            proof_type: "xfg_burn".to_string(),
            transaction_hash: tx_hash.clone(),
            format_version: "1.0".to_string(),
        };

        let cryptographic_data = CryptographicData {
            secret: hex::encode(secret),
            commitment: "placeholder_commitment".to_string(),
            nullifier: "placeholder_nullifier".to_string(),
            xfg_amount,
            block_height: 12345678,
        };

        let user_data = UserData {
            recipient_address: recipient.clone(),
            xfg_amount_formatted: format!("{:.1} XFG", xfg_amount as f64 / 1_000_000.0),
            heat_amount_formatted: format!("{:.2} HEAT", (xfg_amount as f64 / 1_000_000.0) * 0.1),
        };

        let network_validation = NetworkValidation {
            fuego_network_id: 12345,
            network_validation_hash: "placeholder_network_hash".to_string(),
        };

        let security = SecurityData {
            checksum: "placeholder_checksum".to_string(),
            integrity_hash: "placeholder_integrity_hash".to_string(),
            signature: "placeholder_signature".to_string(),
            signature_pubkey: "placeholder_pubkey".to_string(),
            network_validation,
        };

        let mut proof_data = Self {
            metadata,
            cryptographic_data,
            user_data,
            security,
        };

        // Calculate checksum and integrity hash
        proof_data.calculate_security_hashes()?;

        Ok(proof_data)
    }

    /// Calculate security hashes and checksum
    fn calculate_security_hashes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Calculate checksum
        let checksum_data = serde_json::to_string(&self.metadata)? + 
                           &serde_json::to_string(&self.cryptographic_data)? + 
                           &serde_json::to_string(&self.user_data)?;
        
        let mut checksum_hasher = Sha256::new();
        checksum_hasher.update(checksum_data.as_bytes());
        let checksum = checksum_hasher.finalize();
        self.security.checksum = hex::encode(checksum);

        // Calculate integrity hash
        let integrity_data = format!("{}{}{}{}", 
            self.metadata.transaction_hash,
            self.cryptographic_data.secret,
            self.cryptographic_data.xfg_amount,
            self.security.network_validation.fuego_network_id
        );
        
        let mut integrity_hasher = Sha256::new();
        integrity_hasher.update(integrity_data.as_bytes());
        let integrity_hash = integrity_hasher.finalize();
        self.security.integrity_hash = hex::encode(integrity_hash);

        // Calculate network validation hash
        let network_hash = Self::calculate_network_validation_hash(
            self.security.network_validation.fuego_network_id,
            &self.metadata.transaction_hash
        )?;
        self.security.network_validation.network_validation_hash = hex::encode(network_hash);

        // Generate placeholder signature for now
        // In production, this would use a real private key
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
    
    /// Sign the proof data with a private key
    pub fn sign(&mut self, private_key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just set placeholder values
        // TODO: Implement real signing when ed25519-dalek API is properly understood
        self.security.signature = "placeholder_signature".to_string();
        self.security.signature_pubkey = "placeholder_pubkey".to_string();
        
        Ok(())
    }
    
    /// Verify the signature of the proof data
    pub fn verify_signature(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // For now, return true for placeholder signatures
        // TODO: Implement real signature verification
        Ok(true)
    }
    
    /// Create deterministic message for signing
    fn create_signature_message(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Create deterministic message for signing
        let mut hasher = sha3::Keccak256::default();
        hasher.update(&self.metadata.transaction_hash.as_bytes());
        hasher.update(&self.cryptographic_data.secret.as_bytes());
        hasher.update(&self.cryptographic_data.xfg_amount.to_le_bytes());
        hasher.update(&self.security.network_validation.fuego_network_id.to_le_bytes());
        
        Ok(hasher.finalize().to_vec())
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

    /// Calculate network validation hash
    fn calculate_network_validation_hash(
        network_id: u64,
        transaction_hash: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut hasher = Sha256::new();
        hasher.update(network_id.to_le_bytes());
        hasher.update(transaction_hash.as_bytes());
        Ok(hasher.finalize().to_vec())
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
            secret,
            recipient.clone(),
            8_000_000, // 0.8 XFG
            tx_hash.clone(),
        ).unwrap();

        assert_eq!(proof_data.metadata.version, "1.0");
        assert_eq!(proof_data.user_data.recipient_address, recipient);
        assert_eq!(proof_data.cryptographic_data.xfg_amount, 8_000_000);
        assert_eq!(proof_data.metadata.transaction_hash, tx_hash);
    }

    #[test]
    fn test_xfg_amount_validation() {
        assert!(ProofDataFile::is_valid_xfg_amount(8_000_000));
        assert!(ProofDataFile::is_valid_xfg_amount(80_000_000_000));
        assert!(!ProofDataFile::is_valid_xfg_amount(1_000_000));
        assert!(!ProofDataFile::is_valid_xfg_amount(100_000_000_000));
    }

    #[test]
    fn test_proof_data_validation() {
        let secret = [0x42u8; 32];
        let recipient = "0xf8108826279b68504BDF5B3f056382E7Bf821CD0".to_string();
        let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
        
        let proof_data = ProofDataFile::new(
            secret,
            recipient,
            8_000_000,
            tx_hash,
        ).unwrap();

        assert!(proof_data.validate().unwrap());
    }
}
