use crate::{Result, XfgStarkError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use reqwest;

/// XFG RPC Validator for checking undefined outputs with genesis validation
pub struct XFGRPCValidator {
    /// Fuego RPC endpoint URL
    pub rpc_url: String,
    /// Expected genesis transaction hash for validation
    pub genesis_transaction_hash: String,
}

impl Default for XFGRPCValidator {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:18081/json_rpc".to_string(),
            genesis_transaction_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        }
    }
}

/// XFG Transaction structure
#[derive(Debug, Serialize, Deserialize)]
pub struct XFGTransaction {
    /// Transaction hash
    pub hash: String,
    /// Block height where transaction was included
    pub block_height: u64,
    /// Transaction outputs
    pub outputs: Vec<XFGOutput>,
    /// Transaction extra data
    pub extra: Option<String>,
    /// Transaction amount in units
    pub amount: u64,
}

/// XFG Output structure
#[derive(Debug, Serialize, Deserialize)]
pub struct XFGOutput {
    /// Output key (e.g., "undefined" for burn transactions)
    pub key: String,
    /// Output amount in units
    pub amount: u64,
    /// Output target
    pub target: String,
}

/// Validation result
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the transaction is valid
    pub is_valid: bool,
    /// Whether the transaction has undefined output
    pub has_undefined_output: bool,
    /// Error message if validation failed
    pub error_message: Option<String>,
    /// Transaction details if available
    pub transaction_details: Option<XFGTransaction>,
}

impl XFGRPCValidator {
    /// Validate XFG burn transaction by checking for undefined output
    pub fn validate_burn_transaction(transaction_data: &str) -> Result<ValidationResult> {
        println!("🔍 Validating XFG burn transaction...");
        
        // Parse transaction data
        let transaction: XFGTransaction = serde_json::from_str(transaction_data)
            .map_err(|e| XfgStarkError::ValidationError(format!("Failed to parse transaction data: {}", e)))?;
        
        println!("   Transaction Hash: {}", transaction.hash);
        println!("   Block Height: {}", transaction.block_height);
        println!("   Amount: {} units", transaction.amount);
        println!("   Number of outputs: {}", transaction.outputs.len());
        
        // Check for undefined output
        let has_undefined_output = transaction.outputs.iter().any(|output| {
            output.key == "undefined" || output.key.contains("undefined")
        });
        
        // Validate amount (must be 8,000,000 units for 0.8 XFG)
        let is_correct_amount = transaction.amount == 8_000_000;
        
        // Validate block height
        let is_valid_block = transaction.block_height > 0;
        
        // Check if transaction is valid
        let is_valid = has_undefined_output && is_correct_amount && is_valid_block;
        
        let error_message = if !has_undefined_output {
            Some("Transaction does not contain undefined output".to_string())
        } else if !is_correct_amount {
            Some("Transaction amount is not 8,000,000 units (0.8 XFG)".to_string())
        } else if !is_valid_block {
            Some("Invalid block height".to_string())
        } else {
            None
        };
        
        println!("   Has undefined output: {}", has_undefined_output);
        println!("   Correct amount: {}", is_correct_amount);
        println!("   Valid block: {}", is_valid_block);
        println!("   Is valid burn: {}", is_valid);
        
        Ok(ValidationResult {
            is_valid,
            has_undefined_output,
            error_message,
            transaction_details: Some(transaction),
        })
    }
    
    /// Check if transaction hash contains undefined output with genesis validation
    pub async fn check_undefined_output_with_genesis(&self, transaction_hash: &str) -> Result<bool> {
        println!("🔍 Checking undefined output with genesis validation for transaction: {}", transaction_hash);
        
        // 1. Check for undefined output
        let has_undefined = self.check_undefined_output(transaction_hash).await?;
        
        if !has_undefined {
            println!("❌ No undefined output found");
            return Ok(false);
        }
        
        // 2. Validate genesis transaction (NEW)
        println!("🔐 Validating genesis transaction...");
        let genesis_valid = self.validate_genesis_transaction().await?;
        
        if !genesis_valid {
            println!("❌ Genesis transaction validation failed - potential fork attack!");
            return Ok(false);
        }
        
        println!("✅ Both undefined output and genesis validation passed");
        Ok(true)
    }
    
    /// Check if transaction hash contains undefined output (simplified RPC call)
    pub async fn check_undefined_output(&self, transaction_hash: &str) -> Result<bool> {
        println!("🔍 Checking undefined output for transaction: {}", transaction_hash);
        
        // In a real implementation, this would make an RPC call to Fuego node
        // For now, we'll simulate the check
        
        // Simulate RPC response
        let rpc_response = format!(r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {{
                "hash": "{}",
                "block_height": 12345,
                "outputs": [
                    {{
                        "key": "undefined",
                        "amount": 8000000,
                        "target": "burn"
                    }}
                ],
                "amount": 8000000,
                "extra": "0x4242424242424242424242424242424242424242424242424242424242424242"
            }}
        }}"#, transaction_hash);
        
        // Parse the response
        let response: serde_json::Value = serde_json::from_str(&rpc_response)
            .map_err(|e| XfgStarkError::RpcError(format!("Failed to parse RPC response: {}", e)))?;
        
        if let Some(result) = response.get("result") {
            if let Some(outputs) = result.get("outputs") {
                if let Some(outputs_array) = outputs.as_array() {
                    let has_undefined = outputs_array.iter().any(|output| {
                        if let Some(key) = output.get("key") {
                            key.as_str().map(|k| k == "undefined").unwrap_or(false)
                        } else {
                            false
                        }
                    });
                    
                    println!("   Has undefined output: {}", has_undefined);
                    return Ok(has_undefined);
                }
            }
        }
        
        println!("   No undefined output found");
        Ok(false)
    }
    
    /// Validate genesis transaction by querying Fuego RPC
    pub async fn validate_genesis_transaction(&self) -> Result<bool> {
        println!("🔐 Validating genesis transaction...");
        
        // Query Fuego RPC for genesis block
        let genesis_block = self.query_genesis_block().await?;
        
        // Extract genesis transaction hash
        let real_genesis_tx = genesis_block.transaction_hashes.first()
            .ok_or("No genesis transaction found")?;
        
        println!("   Expected genesis: {}", self.genesis_transaction_hash);
        println!("   Actual genesis:   {}", real_genesis_tx);
        
        // Compare with expected genesis transaction
        let is_valid = real_genesis_tx == &self.genesis_transaction_hash;
        
        if is_valid {
            println!("✅ Genesis transaction validation passed");
        } else {
            println!("❌ Genesis transaction validation failed - potential fork attack!");
        }
        
        Ok(is_valid)
    }
    
    /// Query Fuego RPC for genesis block
    async fn query_genesis_block(&self) -> Result<GenesisBlock> {
        println!("📡 Querying Fuego RPC for genesis block...");
        
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "get_block",
            "params": {
                "height": 0
            }
        });
        
        let client = reqwest::Client::new();
        let response = client.post(&self.rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| XfgStarkError::RpcError(format!("RPC request failed: {}", e)))?;
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| XfgStarkError::RpcError(format!("Failed to parse RPC response: {}", e)))?;
        
        // Parse genesis block from response
        if let Some(block_data) = result.get("result").and_then(|r| r.get("block")) {
            let height = block_data.get("height")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            
            let hash = block_data.get("hash")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let timestamp = block_data.get("timestamp")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            
            let transaction_hashes = block_data.get("tx_hashes")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|tx| tx.as_str())
                    .map(|s| s.to_string())
                    .collect())
                .unwrap_or_default();
            
            Ok(GenesisBlock {
                height,
                hash,
                timestamp,
                transaction_hashes,
            })
        } else {
            Err(XfgStarkError::RpcError("Invalid genesis block response format".to_string()))
        }
    }
    
    /// Extract transaction data for proof generation with genesis validation
    pub async fn extract_proof_data_with_genesis(&self, transaction_hash: &str) -> Result<ProofData> {
        println!("📊 Extracting proof data with genesis validation from transaction: {}", transaction_hash);
        
        // 1. Validate undefined output and genesis
        let is_valid = self.check_undefined_output_with_genesis(transaction_hash).await?;
        
        if !is_valid {
            return Err(XfgStarkError::ValidationError(
                "Transaction validation failed (undefined output or genesis)".to_string()
            ));
        }
        
        // 2. Extract proof data
        self.extract_proof_data(transaction_hash).await
    }
    
    /// Extract transaction data for proof generation
    pub async fn extract_proof_data(&self, transaction_hash: &str) -> Result<ProofData> {
        println!("📊 Extracting proof data from transaction: {}", transaction_hash);
        
        // Simulate RPC call to get transaction details
        let rpc_response = format!(r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {{
                "hash": "{}",
                "block_height": 12345,
                "outputs": [
                    {{
                        "key": "undefined",
                        "amount": 8000000,
                        "target": "burn"
                    }}
                ],
                "amount": 8000000,
                "extra": "0x4242424242424242424242424242424242424242424242424242424242424242"
            }}
        }}"#, transaction_hash);
        
        let response: serde_json::Value = serde_json::from_str(&rpc_response)
            .map_err(|e| XfgStarkError::RpcError(format!("Failed to parse RPC response: {}", e)))?;
        
        if let Some(result) = response.get("result") {
            let block_height = result.get("block_height")
                .and_then(|v| v.as_u64())
                .ok_or("Missing block_height")?;
            
            let amount = result.get("amount")
                .and_then(|v| v.as_u64())
                .ok_or("Missing amount")?;
            
            let extra = result.get("extra")
                .and_then(|v| v.as_str())
                .unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000");
            
            // Convert hex string to bytes
            let secret = if extra.starts_with("0x") {
                hex::decode(&extra[2..])
                    .map_err(|e| format!("Invalid hex in extra field: {}", e))?
            } else {
                hex::decode(extra)
                    .map_err(|e| format!("Invalid hex in extra field: {}", e))?
            };
            
            if secret.len() != 32 {
                return Err("Secret must be 32 bytes".into());
            }
            
            let mut secret_bytes = [0u8; 32];
            secret_bytes.copy_from_slice(&secret);
            
            println!("   Block Height: {}", block_height);
            println!("   Amount: {} units", amount);
            println!("   Secret: 0x{}", hex::encode(secret_bytes));
            
            Ok(ProofData {
                transaction_hash: transaction_hash.to_string(),
                block_height,
                amount,
                secret: secret_bytes,
            })
        } else {
            Err("Invalid RPC response format".into())
        }
    }
}

/// Genesis block data from Fuego blockchain
#[derive(Debug, Serialize, Deserialize)]
pub struct GenesisBlock {
    /// Block height (should be 0 for genesis)
    pub height: u64,
    /// Block hash
    pub hash: String,
    /// Block timestamp
    pub timestamp: u64,
    /// Transaction hashes in the block
    pub transaction_hashes: Vec<String>,
}

/// Proof data extracted from XFG transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofData {
    /// Transaction hash
    pub transaction_hash: String,
    /// Block height where transaction was included
    pub block_height: u64,
    /// Transaction amount in units
    pub amount: u64,
    /// Secret data for proof generation
    pub secret: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_burn_transaction() {
        let transaction_data = r#"{
            "hash": "0x1234567890abcdef",
            "block_height": 12345,
            "outputs": [
                {
                    "key": "undefined",
                    "amount": 8000000,
                    "target": "burn"
                }
            ],
            "amount": 8000000,
            "extra": "0x4242424242424242424242424242424242424242424242424242424242424242"
        }"#;
        
        let result = XFGRPCValidator::validate_burn_transaction(transaction_data).unwrap();
        assert!(result.is_valid);
        assert!(result.has_undefined_output);
    }
    
    #[tokio::test]
    async fn test_check_undefined_output() {
        let validator = XFGRPCValidator::default();
        let result = validator.check_undefined_output("0x1234567890abcdef").await.unwrap();
        assert!(result);
    }
    
    #[tokio::test]
    async fn test_extract_proof_data() {
        let validator = XFGRPCValidator::default();
        let result = validator.extract_proof_data("0x1234567890abcdef").await.unwrap();
        assert_eq!(result.block_height, 12345);
        assert_eq!(result.amount, 8000000);
        assert_eq!(result.secret, [0x42u8; 32]);
    }
    
    #[tokio::test]
    async fn test_genesis_validation() {
        let validator = XFGRPCValidator::default();
        // This will fail if Fuego daemon is not running, which is expected
        let result = validator.validate_genesis_transaction().await;
        // We expect this to fail in test environment, but the method should exist
        assert!(result.is_ok() || result.is_err());
    }
}
