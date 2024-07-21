use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    NearToken,
};
use near_workspaces::{network::Sandbox, Contract, Worker};

const CONTRACT_FILE_PATH: &str =
    "./target/wasm32-unknown-unknown/debug/mock_signer_chain_signatures.wasm";

async fn init() -> (Worker<Sandbox>, Contract) {
    let worker = near_workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(CONTRACT_FILE_PATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();
    (worker, contract)
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct SignRequest {
    pub payload: [u8; 32],
    pub path: String,
    pub key_version: u32,
}

#[tokio::test]
async fn test_contract_sign_request() {
    let (_, contract) = init().await;
    let predecessor_id = contract.id();
    let path = "test";

    let sign_request = SignRequest {
        payload: [0; 32],
        path: path.to_string(),
        key_version: 0,
    };

    let result = contract
        .call("sign")
        .args_json(serde_json::json!({"request": sign_request}))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact_async()
        .await
        .unwrap();

    println!("{:?}", result);
}
