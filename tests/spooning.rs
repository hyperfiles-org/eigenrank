mod helpers;

#[test]
fn test_spoon_feeding_contract_data() {
    let (master_account, contract) = helpers::setup_contract();

    // Simulate feeding the contract initial data or state
    let initial_data = "Initial data setup for contract";
    let tx = contract.call(
        master_account.user_account.account_id.clone(),
        "initialize_data",
        initial_data.as_bytes(),
        near_sdk_sim::DEFAULT_GAS,
        0, // attached deposit
    );

    assert!(tx.is_ok());
    // Further asserts or checks can be added here
}
