# 📦 XFG STARK Proof Data Package System

This document describes the data package system for generating STARK proofs in the XFG → HEAT burn and mint process.

## 🎯 **Overview**

The data package system provides a **user-friendly way** to package all the information needed for STARK proof generation into structured JSON files. Users can:

- **Create templates** for different burn types
- **Fill in their data** (transaction hash, recipient, secret)
- **Validate the package** before proof generation
- **Generate STARK proofs** using the CLI tool
- **Submit proofs** to the HEAT mint contract

## 🏗️ **Architecture**

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Template      │    │   Data Package   │    │   STARK Proof   │
│   (JSON)        │───▶│   (JSON)         │───▶│   (JSON/Binary) │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                       │                       │
        │                       │                       │
        ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Standard Burn  │    │ User's Burn Data │    │  Ready for      │
│  0.8 XFG        │    │ + Secret + Txn   │    │  Contract       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 📁 **File Structure**

```
xfgwin/
├── src/
│   ├── proof_data_schema.rs          # Data structures and validation
│   └── bin/
│       └── xfg-stark-cli.rs          # CLI tool for proof generation
├── examples/
│   └── data_packages/
│       ├── standard_burn_template.json    # 0.8 XFG template
│       ├── large_burn_template.json       # 800 XFG template
│       └── example_standard_burn.json     # Example completed package
├── docs/
│   └── STARK_PROOF_USER_GUIDE.md         # User guide
├── scripts/
│   ├── build-cli.sh                      # Build script
│   └── example-workflow.sh               # Example workflow
└── README_DATA_PACKAGES.md               # This file
```

## 🔧 **Core Components**

### **1. Data Schema (`proof_data_schema.rs`)**

Defines the structure for STARK proof data packages:

- **`StarkProofDataPackage`**: Complete data package
- **`ProofMetadata`**: Version, timestamp, network info
- **`BurnTransaction`**: Transaction details and amounts
- **`RecipientInfo`**: Ethereum address and optional metadata
- **`SecretInfo`**: User's secret and security options
- **`ValidationResult`**: Package validation results

### **2. CLI Tool (`xfg-stark-cli`)**

Command-line interface for the entire workflow:

```bash
# Create template
xfg-stark-cli create-template standard -o template.json

# Create package
xfg-stark-cli create-package --template template.json --burn-amount 0.8 --txn-hash 0x123... --recipient 0x456... --secret my-secret --output package.json

# Validate package
xfg-stark-cli validate -i package.json

# Generate proof
xfg-stark-cli generate -i package.json -o proof.json
```

### **3. Templates**

Pre-configured templates for common burn types:

- **Standard Burn (0.8 XFG)**: Regular HEAT accumulation
- **Large Burn (800 XFG)**: Bulk HEAT minting

## 📊 **Data Package Structure**

### **Required Fields**

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `burn_amount_xfg` | String | Burn amount in XFG | `"0.8"` or `"800.0"` |
| `transaction_hash` | String | Fuego burn transaction hash | `"0x7D0725F8E03021B99560ADD456C596FEA7D8DF23529E23765E56923B73236E4D"` |
| `ethereum_address` | String | HEAT recipient address | `"0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6"` |
| `secret_key` | String | User's private secret | `"my-secret-key-123"` |

### **Optional Fields**

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `block_height` | u64 | Block where burn occurred | `1234567` |
| `timestamp` | u64 | Burn transaction timestamp | `1705312200` |
| `ens_name` | String | ENS name for recipient | `"alice.eth"` |
| `label` | String | Human-readable label | `"Alice's HEAT wallet"` |
| `salt` | String | Additional security | `"random-salt-67890"` |
| `hint` | String | Secret recovery hint | `"Remember: my favorite color + birth year"` |

## 🔐 **Security Features**

### **Input Validation**
- **Burn amount validation**: Only 0.8 or 800.0 XFG allowed
- **Transaction hash format**: Must start with 0x
- **Ethereum address format**: Must be valid 0x-prefixed hex
- **Secret key length**: Minimum 8 characters required

### **Data Integrity**
- **Atomic unit conversion**: Automatic XFG ↔ atomic units
- **Hash validation**: Recipient address hashing
- **Format verification**: JSON schema validation
- **Network isolation**: Separate mainnet/testnet packages

## 🚀 **Usage Workflow**

### **1. Setup**
```bash
# Build the CLI tool
./scripts/build-cli.sh

# Install globally (optional)
sudo cp target/release/xfg-stark-cli /usr/local/bin/
```

### **2. Create Template**
```bash
# Create standard burn template
xfg-stark-cli create-template standard -o standard_template.json
```

### **3. Create Data Package**
```bash
# Fill in your data
xfg-stark-cli create-package \
  --template standard_template.json \
  --burn-amount 0.8 \
  --txn-hash 0x7D0725F8E03021B99560ADD456C596FEA7D8DF23529E23765E56923B73236E4D \
  --recipient 0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6 \
  --secret "my-super-secret-key-12345" \
  --network fuego-mainnet \
  --output my_burn_package.json
```

### **4. Edit Package**
Open `my_burn_package.json` and add:
- Block height where burn occurred
- Timestamp of burn transaction
- Optional metadata (ENS names, labels, etc.)

### **5. Validate Package**
```bash
xfg-stark-cli validate -i my_burn_package.json
```

### **6. Generate STARK Proof**
```bash
xfg-stark-cli generate -i my_burn_package.json -o proof.json
```

### **7. Submit to Contract**
Use `proof.json` with the HEAT mint contract, along with Eldernode validation proof.

## 🧪 **Testing**

### **Run Example Workflow**
```bash
# Execute complete example workflow
./scripts/example-workflow.sh
```

### **Run Unit Tests**
```bash
# Test data schema and validation
cargo test proof_data_schema

# Test CLI tool
cargo test --bin xfg-stark-cli
```

### **Test CLI Tool**
```bash
# Show help
xfg-stark-cli --help

# Test template creation
xfg-stark-cli create-template standard -o test_template.json

# Test package creation
xfg-stark-cli create-package --template test_template.json --burn-amount 0.8 --txn-hash 0x123 --recipient 0x456 --secret test-secret --output test_package.json

# Test validation
xfg-stark-cli validate -i test_package.json
```

## 🔧 **Development**

### **Adding New Fields**
1. Update `StarkProofDataPackage` struct in `proof_data_schema.rs`
2. Add validation logic in `validate()` method
3. Update CLI tool to handle new fields
4. Add tests for new functionality

### **Adding New Templates**
1. Create new template in `ProofDataTemplate` implementation
2. Add template file in `examples/data_packages/`
3. Update CLI tool to support new template type
4. Add documentation and examples

### **Customizing Validation**
1. Modify validation rules in `validate()` method
2. Add new validation functions as needed
3. Update error messages and warnings
4. Test with various input scenarios

## 📚 **Documentation**

- **[User Guide](docs/STARK_PROOF_USER_GUIDE.md)**: Complete user documentation
- **[API Reference](src/proof_data_schema.rs)**: Data structure definitions
- **[CLI Reference](src/bin/xfg-stark-cli.rs)**: Command-line tool usage
- **[Examples](examples/data_packages/)**: Sample data packages and templates

## 🤝 **Contributing**

### **Code Style**
- Follow Rust conventions
- Add comprehensive tests
- Update documentation
- Use meaningful commit messages

### **Testing**
- Test with valid and invalid inputs
- Test edge cases and error conditions
- Test CLI tool functionality
- Test data package validation

### **Documentation**
- Keep user guide up to date
- Add examples for new features
- Document security considerations
- Provide troubleshooting guides

## 🔗 **Integration**

### **With STARK Proof System**
- Data packages feed directly into `XfgBurnMintProver`
- Validation ensures data integrity before proof generation
- CLI tool handles all Winterfell integration details

### **With HEAT Contract**
- Generated proofs are ready for contract submission
- Multiple output formats (JSON, binary, hex) for flexibility
- Metadata includes all necessary contract parameters

### **With Eldernode System**
- Data packages include network and transaction information
- Eldernodes can validate on-chain data independently
- Separate validation and STARK proof processes

## 🚨 **Security Considerations**

### **Secret Management**
- **Never store secrets** in plain text files
- **Use secure storage** (password managers, hardware wallets)
- **Rotate secrets** regularly
- **Use salt** for additional security

### **Data Validation**
- **Always validate** packages before proof generation
- **Verify transaction details** independently
- **Check recipient addresses** carefully
- **Test on testnet** before mainnet

### **Network Security**
- **Use HTTPS** for template downloads
- **Verify file integrity** with checksums
- **Isolate testnet** from mainnet data
- **Monitor for suspicious activity**

## 📞 **Support**

For issues and questions:

1. **Check the user guide** for common solutions
2. **Review validation output** for specific errors
3. **Test with example data** to isolate problems
4. **Check the logs** for detailed error information
5. **Review security best practices** for your use case

## 📈 **Future Enhancements**

### **Planned Features**
- **Web interface** for package creation
- **Batch processing** for multiple burns
- **Advanced validation** rules
- **Integration with wallets** and exchanges

### **Potential Improvements**
- **Template versioning** and updates
- **Automated testing** and validation
- **Performance optimization** for large packages
- **Multi-language support** for CLI tool

---

**🎯 Goal**: Make STARK proof generation accessible to all users while maintaining security and reliability standards.
