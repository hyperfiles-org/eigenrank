use workspaces::prelude::*;
use workspaces::network::Sandbox;

#[tokio::test]
async fn test_contract_deploy() {
    let worker = workspaces::sandbox().await.unwrap();
    let contract = worker.dev_deploy(include_bytes!("../res/eigenrank.wasm")).await.unwrap();

    let outcome = contract.call(&worker, "new")
        .args_json(("example.testnet".to_string()))
        .transact()
        .await.unwrap();

    assert!(outcome.success);
}
