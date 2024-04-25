
# EigenTrust NEAR Smart Contract

This repository contains the Rust-based NEAR smart contract implementation of the Eigentrust algorithm. It is designed to interact with the socialDB contract deployed on the NEAR blockchain, calculating global profile rankings based on user interactions.

## Features

- **Eigentrust Algorithm Implementation:** Implements a modified version of the Eigentrust algorithm to calculate trust scores based on social interactions.
- **Integration with socialDB:** Fetches interaction data from the socialDB contract.
- **Storage of Rankings:** Rankings are computed and then stored back in the socialDB contract under each user's profile.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust and Cargo: [Installation Guide](https://www.rust-lang.org/tools/install)
- NEAR CLI: Install it globally via npm:
  ```bash
  npm install -g near-cli
  ```
- wasm32 target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/eigentrust-near.git
   ```
2. Navigate into the project directory:
   ```bash
   cd eigentrust-near
   ```

### Building the Contract

Run the build script to compile the smart contract to WebAssembly (Wasm):

```bash
./build.sh
```

The script cleans previous builds, compiles the contract, and provides the Wasm file ready for deployment.

### Testing

To run the unit tests:

```bash
cargo test
```

## Deployment

To deploy the contract to the NEAR testnet:

1. Ensure you are logged in to NEAR CLI:
   ```bash
   near login
   ```
2. Deploy the contract using:
   ```bash
   near deploy --accountId your-account.testnet --wasmFile path/to/your_contract.wasm
   ```

Replace `your-account.testnet` with your actual NEAR testnet account name.

### Usage
Deploy the contract to NEAR testnet or mainnet using NEAR CLI commands.

**Deploy using:**
```bash
near deploy --wasmFile target/wasm32-unknown-unknown/release/eigentrust_near.wasm --accountId YOUR_ACCOUNT.testnet
```

**Initialize the contract:**
```bash
near call YOUR_ACCOUNT.testnet new '{"social_db_contract_id": "social.near"}' --accountId YOUR_ACCOUNT.testnet
```
### Additional Tips
Dependencies: Ensure all required build tools and dependencies are installed, such as the Rust toolchain and the wasm32-unknown-unknown target.

You can install the target with:
```bash
rustup target add wasm32-unknown-unknown
```
Optimization Tool: If your contract is large or you want to optimize for performance, consider using wasm-opt from the Binaryen suite. This tool can greatly reduce the size of your Wasm binary which can save costs when deploying to the blockchain.

Automation: You can integrate this script into CI/CD pipelines for automated builds and deployments.

## Calculate Rankings

To calculate rankings for a user:

```bash
near call your-account.testnet calc_rank '{"seed_accounts": ["seed1.testnet"], "user_id": "user1.testnet", "seed_strategy": 0, "localtrust_strategy": 0}' --accountId your-account.testnet
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Illia, Eugene, James, Elliot
- Karma3
- Original authors of the Eigentrust algorithm

---

### Notes:
- **Customization**: Replace placeholders like `https://github.com/your-username/eigentrust-near.git` with actual URLs.
- **Additional Files**: Include a `CONTRIBUTING.md` and `LICENSE.md` in your repository.
- **Community and Support**: Encourage community interaction by including links to issues, discussions, and support channels.