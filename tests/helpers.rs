use eigenrank::EigenTrustContract;
use near_sdk::{AccountId, Balance};
use near_sdk_sim::{deploy, init_simulator, UserAccount};

pub fn setup_contract() -> (UserAccount, EigenTrustContract) {
    let master_account = init_simulator(None);
    let contract_account = deploy!(
        contract: EigenTrustContract,
        contract_id: AccountId::new_unchecked("eigenrank.testnet".to_string()),
        bytes: &include_bytes!("../../out/eigenrank.wasm").to_vec(),
        signer_account: master_account,
    );

    (master_account, contract_account)
}

pub fn to_yocto_near(amount: &str) -> Balance {
    amount.parse::<Balance>().unwrap() * 10u128.pow(24)
}
