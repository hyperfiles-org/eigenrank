use near_sdk::serde_json;
use eigenrank::EigenTrustContract;
use near_sdk_sim::{deploy, init_simulator, to_yocto, DEFAULT_GAS};

#[test]
fn test_integration_behavior() {
    let master_account = init_simulator(None);
    let contract_account = deploy!(
        contract: EigenTrustContract,
        contract_id: "contract",
        bytes: &include_bytes!("../out/eigenrank.wasm").to_vec(),
        signer_account: master_account
    );

    // Example of calling a method
    let outcome = contract_account.call(
        master_account.user_account.account_id.clone(),
        "new",
        &serde_json::to_vec(&("example.testnet".to_string())).unwrap(),
        DEFAULT_GAS,
        0, // attached deposit
    );

    assert!(outcome.is_ok());
}
