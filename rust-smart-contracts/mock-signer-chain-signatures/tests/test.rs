use ethers_core::{
    k256::elliptic_curve::point::AffineCoordinates,
    types::{Signature, U256},
    utils::hex::ToHexExt,
};

use k256::ecdsa::VerifyingKey;
use mock_signer_chain_signatures::{
    derive_eth_address, derive_public_key,
    lib::types::{SignRequest, SignatureResponse},
};
use near_sdk::{bs58, NearToken, PublicKey};
use near_workspaces::{network::Sandbox, Contract, Worker};

const CONTRACT_FILE_PATH: &str =
    "./target/wasm32-unknown-unknown/release/mock_signer_chain_signatures.wasm";

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
    let predecessor = contract.id();

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

    let sign_response: SignatureResponse = contract
        .call("sign")
        .args_json(serde_json::json!(sign_request))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact_async()
        .await
        .unwrap()
        .await
        .unwrap()
        .json()
        .unwrap();

    let public_key_response: PublicKey = contract
        .call("public_key")
        .max_gas()
        .transact_async()
        .await
        .unwrap()
        .await
        .unwrap()
        .json()
        .unwrap();

    let r = U256::from_big_endian(&sign_response.big_r.affine_point.x());
    let s = U256::from_big_endian(&sign_response.s.scalar.to_bytes());
    let v = (sign_response.recovery_id + 27) as u64;
    let signature = Signature { r, s, v };

    let recovered_address = signature.recover(sign_request.payload).unwrap();

    let root_public_key = String::from(&public_key_response);
    let root_public_key = root_public_key.split(":").nth(1).unwrap();
    let root_public_key = bs58::decode(root_public_key).into_vec().unwrap();
    let mut sec1_key = vec![0x04];
    sec1_key.extend_from_slice(&root_public_key);
    let root_public_key = VerifyingKey::from_sec1_bytes(&sec1_key).unwrap();

    let public_key = derive_public_key(&root_public_key, predecessor.to_string(), path.to_string());
    let derived_address = derive_eth_address(&public_key);

    assert_eq!(recovered_address.encode_hex(), derived_address);
}
