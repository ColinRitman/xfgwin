 # Enhanced XFG STARK CLI

The Enhanced XFG STARK CLI provides a comprehensive tool for generating STARK proofs with integrated Eldernode verification using identical inputs for maximum security.

## 🚀 Key Features

### **Parallel Processing**
- **STARK Generation**: Generates cryptographic proofs using Winterfell framework
- **Eldernode Verification**: Verifies blockchain data with Eldernode consensus
- **Identical Inputs**: Both processes use exactly the same inputs for security

### **Real-Time Progress Tracking**
- Live status updates for both STARK and Eldernode processes
- Progress indicators showing Eldernode response counts
- Timing information for performance monitoring

### **Security-First Design**
- **Input Consistency Verification**: Ensures both processes used identical inputs
- **Attack Prevention**: Prevents data manipulation between systems
- **Atomic Verification**: Both processes must succeed for final output

## 📦 Installation

```bash
# Build the enhanced CLI
cargo build --bin xfg-stark-enhanced-cli

# Build with optimizations for production
cargo build --release --bin xfg-stark-enhanced-cli
```

## 🔧 Usage

### **Prove-and-Verify Command**

The main command that handles both STARK generation and Eldernode verification:

```bash
./target/debug/xfg-stark-enhanced-cli prove-and-verify \
  --input burn-package.json \
  --output complete-proof.json \
  --eldernode-endpoint https://eldernodes.fuego.network/api/v1/verify
```

#### **Parameters:**
- `--input`: Input data package file (JSON format)
- `--output`: Output complete proof package file
- `--eldernode-endpoint`: Eldernode verification endpoint (optional, has default)

### **Other Commands**

```bash
# Generate STARK proof only
./target/debug/xfg-stark-enhanced-cli generate \
  --input burn-package.json \
  --output stark-proof.json

# Validate data package
./target/debug/xfg-stark-enhanced-cli validate \
  --input burn-package.json

# Create template
./target/debug/xfg-stark-enhanced-cli create-template \
  --burn-amount 8000000 \
  --output template.json

# Create package from template
./target/debug/xfg-stark-enhanced-cli create-package \
  --template template.json \
  --txn-hash abcdef1234567890 \
  --recipient 0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b7 \
  --output package.json
```

## 🔄 Workflow

### **1. Data Package Preparation**
```bash
# Create a template for 0.8 XFG burn
./target/debug/xfg-stark-enhanced-cli create-template \
  --burn-amount 8000000 \
  --output template.json

# Create actual package with transaction details
./target/debug/xfg-stark-enhanced-cli create-package \
  --template template.json \
  --txn-hash YOUR_TRANSACTION_HASH \
  --recipient YOUR_ETH_ADDRESS \
  --output burn-package.json
```

### **2. Parallel Verification**
```bash
# Run STARK generation and Eldernode verification in parallel
./target/debug/xfg-stark-enhanced-cli prove-and-verify \
  --input burn-package.json \
  --output complete-proof.json
```

### **3. Progress Monitoring**
The CLI provides real-time progress updates:

```
🔐 STARK Generation: Creating prover...
📤 Eldernode Verification: Sending verification request...
⏱️  Total Time: 1.2s

🔐 STARK Generation: Generating proof...
📊 Eldernode Verification: 2/5 Eldernodes responded
⏱️  Total Time: 2.8s

🔐 STARK Generation: Proof generated successfully!
✅ Eldernode Verification: Consensus reached!
⏱️  Total Time: 4.1s
```

### **4. Output**
The complete proof package contains:
- **STARK Proof**: Cryptographic proof for L2 contract verification
- **Eldernode Consensus**: Blockchain verification consensus
- **STARK Inputs**: Inputs used for both processes
- **Metadata**: Package information and timestamps

## 🔒 Security Architecture

### **Identical Inputs Requirement**
Both STARK generation and Eldernode verification use exactly the same inputs:

```rust
struct StarkGenerationInputs {
    secret: Vec<u8>,
    burn_amount: u64,
    mint_amount: u64,
    tx_prefix_hash: [u8; 32],
    recipient_hash: Vec<u8>,
    network_id: u32,
    target_chain_id: u32,
    commitment_version: u32,
    domain_separator: String,
}
```

### **Input Consistency Verification**
After both processes complete, the system verifies they used identical inputs:

```rust
fn verify_input_consistency(
    original_inputs: &StarkGenerationInputs,
    stark_proof: &StarkProof,
    eldernode_consensus: &EldernodeConsensus
) -> Result<()> {
    // Extract inputs from both processes
    let stark_inputs = extract_inputs_from_stark_proof(stark_proof)?;
    let eldernode_inputs = extract_inputs_from_eldernode_consensus(eldernode_consensus)?;
    
    // Verify all inputs are identical
    if !inputs_are_identical(original_inputs, &stark_inputs, &eldernode_inputs) {
        return Err(SecurityError("Input consistency violation detected"));
    }
    
    Ok(())
}
```

## 🛡️ Attack Prevention

### **Data Manipulation Attacks**
- **Prevention**: Identical inputs prevent sending different data to each system
- **Detection**: Input consistency verification catches any discrepancies
- **Response**: Process fails if inputs don't match

### **Replay Attacks**
- **Prevention**: Transaction hash binding prevents proof reuse
- **Detection**: Each proof is tied to specific transaction
- **Response**: Invalid proofs are rejected

### **Fake Verification**
- **Prevention**: Eldernode consensus requires multiple signatures
- **Detection**: Consensus threshold verification
- **Response**: Insufficient consensus causes failure

## 📊 Performance

### **Parallel Processing Benefits**
- **Time Savings**: Both processes run simultaneously
- **Resource Efficiency**: Optimal CPU and network utilization
- **User Experience**: Real-time progress updates

### **Typical Performance**
- **STARK Generation**: 2-5 seconds (depends on complexity)
- **Eldernode Verification**: 1-3 seconds (network dependent)
- **Total Time**: 3-6 seconds (parallel execution)

## 🔧 Configuration

### **Eldernode Endpoints**
Default endpoint: `https://eldernodes.fuego.network/api/v1/verify`

Custom endpoints can be specified:
```bash
--eldernode-endpoint https://your-eldernode.com/api/v1/verify
```

### **Network IDs**
- **Fuego Testnet**: 4
- **Fuego Mainnet**: 1
- **Arbitrum One**: 42161
- **Arbitrum Sepolia**: 421614

## 🚨 Error Handling

### **Common Errors**
- **Invalid Transaction Hash**: Check hex format and length
- **Invalid Recipient Address**: Ensure valid Ethereum address format
- **Network Errors**: Check Eldernode endpoint connectivity
- **Input Consistency Violation**: Security attack detected

### **Recovery**
- **Retry Logic**: Automatic retry for transient network errors
- **Fallback Endpoints**: Multiple Eldernode endpoints supported
- **Detailed Logging**: Comprehensive error reporting

## 📝 Example Output

```json
{
  "stark_proof": {
    "proof_data": "0x...",
    "public_inputs": {
      "burn_amount": 8000000,
      "mint_amount": 8000000000000000000000000,
      "txn_hash": "0x1234567890abcdef...",
      "recipient_hash": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b7",
      "state": 0
    },
    "metadata": {
      "version": "1.0.0",
      "created_at": "2024-01-15T10:30:00Z",
      "description": "STARK proof for 0.8 XFG burn",
      "network": "fuego-testnet"
    }
  },
  "eldernode_verification": {
    "eldernode_ids": ["elder1", "elder2", "elder3"],
    "signatures": ["sig1", "sig2", "sig3"],
    "message_hash": "consensus_hash",
    "timestamp": "2024-01-15T10:30:00Z",
    "consensus_threshold": 3,
    "total_eldernodes": 5
  },
  "stark_inputs": {
    "secret": "0x...",
    "burn_amount": 8000000,
    "mint_amount": 8000000000000000000000000,
    "tx_prefix_hash": "0x...",
    "recipient_hash": "0x...",
    "network_id": 4,
    "target_chain_id": 42161,
    "commitment_version": 1,
    "domain_separator": "heat-commitment-v1"
  },
  "metadata": {
    "version": "1.0.0",
    "network": "fuego-testnet",
    "created_at": "2024-01-15T10:30:00Z",
    "description": "Complete proof package for XFG burn"
  }
}
```

## 🎯 Next Steps

1. **Deploy to Production**: Use release build for production environments
2. **Configure Eldernodes**: Set up production Eldernode endpoints
3. **Monitor Performance**: Track verification times and success rates
4. **Security Audits**: Regular security reviews of the verification process

## 🤝 Contributing

For contributions to the enhanced CLI:
1. Follow the existing code style
2. Add comprehensive tests for new features
3. Update documentation for any changes
4. Ensure security best practices are maintained
