use mock_signer_chain_signatures::lib::types::{SignRequest, SignatureResponse};
use near_sdk::NearToken;
use near_workspaces::{network::Sandbox, Contract, Worker};

const CONTRACT_FILE_PATH: &str =
    "./target/wasm32-unknown-unknown/debug/mock_signer_chain_signatures.wasm";

async fn init() -> (Worker<Sandbox>, Contract) {
    let worker = near_workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(CONTRACT_FILE_PATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();
    (worker, contract)
}

#[tokio::test]
async fn test_contract_sign_request() {
    let (_, contract) = init().await;
    let path = "test";

    let _ = contract
        .call("new")
        .max_gas()
        .transact_async()
        .await
        .unwrap()
        .await
        .unwrap();

    let sign_request = SignRequest {
        payload: [0; 32],
        path: path.to_string(),
        key_version: 0,
    };

    let status = contract
        .call("sign")
        .args_json(serde_json::json!(sign_request))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact_async()
        .await
        .unwrap();

    let result = status.await.unwrap();
    let execution = result.into_result().unwrap();
    let response: SignatureResponse = execution.json().unwrap();

    println!("{:?}", response);
}
