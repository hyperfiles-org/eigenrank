
# EigenRank NEAR Smart Contract

This repository contains the Rust-based NEAR smart contract implementation of the Eigenrank algorithm. It is designed to interact with the socialDB contract deployed on the NEAR blockchain, calculating global profile rankings based on user interactions.

## Features

- **Eigenrank Algorithm Implementation:** Implements a modified version of the Eigenrank algorithm to calculate trust scores based on social interactions.
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
   git clone https://github.com/hyperfiles-org/eigenrank
   ```
2. Navigate into the project directory:
   ```bash
   cd hyperfiles-org/eigenrank
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

### Additional Tips
Dependencies: Ensure all required build tools and dependencies are installed, such as the Rust toolchain and the wasm32-unknown-unknown target.

You can install the target with:
```bash
rustup target add wasm32-unknown-unknown
```
Optimization Tool: If your contract is large or you want to optimize for performance, consider using wasm-opt from the Binaryen suite. This tool can greatly reduce the size of your Wasm binary which can save costs when deploying to the blockchain.

Automation: You can integrate this script into CI/CD pipelines for automated builds and deployments.

### Usage
Deploy the contract to NEAR testnet or mainnet using NEAR CLI commands.

**Initialize the contract:**
```bash
near call YOUR_ACCOUNT.testnet new '{"social_db_contract_id": "social.near"}' --accountId YOUR_ACCOUNT.testnet
```

## Calculate Rankings

To calculate rankings for a user:

```bash
near call your-account.testnet calc_rank '{"seed_accounts": ["seed1.testnet"], "user_id": "user1.testnet", "seed_strategy": 0, "localtrust_strategy": 0}' --accountId your-account.testnet
```

**Overview of `calc_rank` **
The `calc_rank` function is a core component of the Eigenrank contract. It is designed to calculate trust rankings for users within a social network based on their interactions. This method interacts with the socialDB contract to fetch interaction data, process it, and compute rankings based on the Eigenrank algorithm.

**Function Signature **
```rust
pub fn calc_rank(
    &self,
    seed_accounts: Vec<String>,
    user_id: String,
    seed_strategy: u8,
    localtrust_strategy: u8
)
```

#### Usage Example
```bash
near call eigenrank.testnet calc_rank '{"seed_accounts": ["trusted1.near", "trusted2.near"], "user_id": "user123.near", "seed_strategy": 0, "localtrust_strategy": 1}' --accountId caller_account.testnet
```

#### Error Handling
- Ensure all account IDs are valid NEAR accounts.
- The strategies should correspond to implemented methods in the contract. Using an undefined strategy will result in a runtime error.

### Parameters
1. **seed_accounts (`Vec<String>`):**
   - **Description:** A list of account IDs that are used as seed nodes for the trust calculation. These accounts are considered trusted by default and influence the trust calculation for other users.
   - **Customization:** Users can customize this list based on which accounts they want to influence the trust scores in the network. More trusted nodes typically provide a stronger foundation for the trust graph.

2. **user_id (`String`):**
   - **Description:** The NEAR account ID of the user for whom the trust rank is being calculated.
   - **Customization:** This is dynamically set to the user's account ID for whom the rank needs to be computed.

3. **seed_strategy (`u8`):**
   - **Description:** A numerical identifier that specifies the algorithm or method used to integrate the seed accounts' trust values into the overall trust calculation.
   - **Customization:** Different strategies can be implemented and identified by unique `u8` values. For example, `0` might represent a basic influence method where seed accounts contribute equally, whereas `1` could represent a weighted method where different seeds have different levels of influence based on additional metrics.

4. **localtrust_strategy (`u8`):**
   - **Description:** A numerical identifier that determines how local trust scores are computed from the raw interaction data. This affects how interactions between users are converted into trust values.
   - **Customization:** Like the seed strategy, different methods can be employed, each represented by a different `u8` value. Strategy `0` might simply count interactions, while strategy `1` might consider the type and recency of interactions.

### How It Works
- The function initiates a promise to call the `get_interactions` method on the social database contract, passing the `user_id` to fetch interaction data.
- Upon successful retrieval of interaction data, a follow-up promise is made to `process_interactions`, which processes the interaction data along with the initial parameters (`seed_accounts`, `user_id`, `seed_strategy`, `localtrust_strategy`) to calculate the final trust rankings.

#### Calculating global trust scores using the power iteration method
1. Initialization:
- global_trust: A HashMap initialized to store the global trust scores between pairs of entities (follower, follows).
- next_global_trust: A clone of local_trust used to store intermediate values during the computation.
2. Iteration Loop:
- The loop runs 10 times, which represents the number of iterations to allow the scores to converge toward stability. The number of iterations controls the depth of influence propagation through the network, effectively implementing attenuation by limiting the spread of influence to a fixed number of steps.
3. Calculation Within Each Iteration:
- For each pair (follower, follows) in local_trust, calculate the contribution to the global trust score:
- Contribution Calculation: The contribution of each follower to the entity they follow is calculated as trust * current_global_trust. Here, trust is the local trust score, and current_global_trust is fetched from global_trust with a fallback value of 0.1 if not present.
- This calculation multiplies the local trust (immediate trust between two entities) by the existing global trust score (accumulated trust from previous iterations), thereby propagating and attenuating the trust score through the network.
- Accumulation: The calculated contribution is added to next_global_trust for the corresponding pair.
4. Swapping and Resetting:
- After each iteration, global_trust and next_global_trust are swapped. This makes the next round of computations use the most recently calculated values.
- next_global_trust is then reset to zero for all values to prepare for the next iteration. This reset is crucial as it clears the accumulated values, ensuring that each iteration starts fresh based on the latest calculations.

**Implementation of Attenuation**
The concept of "attenuation" in this context refers to the gradual reduction in the influence or trust propagated from one entity to another over multiple iterations or layers of relationships. By limiting the number of iterations and by the nature of the multiplication of trust scores, the influence of any given entity's trust diminishes as it propagates further away in the network. This mimics real-world trust dynamics, where trust diminishes as it passes through multiple intermediaries.

In summary, the implementation you provided leverages the power iteration method to simulate the propagation and attenuation of trust scores in a network, allowing for a controlled and realistic model of trust dynamics over a network of entities.

#### Conclusion
The `calc_rank` function is a powerful tool for calculating user trust within a network, leveraging predefined seed accounts and customizable strategies to adapt to various trust models. By understanding and correctly setting its parameters, users can effectively influence and interpret the trust dynamics within their applications.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Illia, Eugene, James, Elliot
- Karma3
- [Original authors of the Eigentrust algorithm](https://nlp.stanford.edu/pubs/eigentrust.pdf)

---