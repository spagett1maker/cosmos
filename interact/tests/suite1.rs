use cosmrs::{
    bip32::{self},
    crypto::secp256k1,
};
use pdao_cosmos_interact::utils::private_to_pub_and_account;
use pdao_cosmos_interact::*;

use serde_json::json;

#[ignore]
#[tokio::test]
async fn check_connection() {
    let _config = Config::read_from_env();
    // check whether the full node is responding by a simple request
    unimplemented!();
}

#[ignore]
#[tokio::test]
async fn check_block_number() {
    let _config = Config::read_from_env();
    // check the latest block number recognized by the full node **twice** with some delay,
    // so that we can assure that the full node is properly updating its blocks
    unimplemented!();
}

/// by requesting the full node, checks whether the account given by the config has enough native token to pay gas fee
#[ignore]
#[tokio::test]
async fn check_account() {
    let _config = Config::read_from_env();

    let rest_api_endpoint = "TODO";
    let target_address = "TODO";
    let min_balance = 1234; // TODO;

    let client = reqwest::Client::new();
    let response = request(
        &client,
        &format!(
            "https://{}/cosmos/bank/v1beta1/balances/{}",
            rest_api_endpoint, target_address
        ),
        None,
    )
    .await
    .unwrap();

    let current_balance = response["balances"].as_array().unwrap()[0]["amount"]
        .as_str()
        .unwrap();

    assert!(min_balance <= current_balance.parse::<u64>().unwrap());
}

#[ignore]
#[tokio::test]
async fn test_query_get_count() {}

#[ignore]
#[tokio::test]
async fn test_query_get_auth() {}

#[ignore]
#[tokio::test]
async fn test_execute_increment_fail() {
    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();

    // This should be failed since the count is above 10
    let msg = json!({
        "increment": {"count": 20u64}
    });

    execute::send_execute(
        &key,
        "malaga-420",
        "https://rpc.malaga-420.cosmwasm.com:443",
        "umlg",
        "wasm",
        10000,
        "wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
        1411,
    )
    .await
    // deliver_tx failed: TxResult { code: Err(5), data: None, log: Log("failed to execute message; message index: 0: Unauthorized: execute wasm contract failed"), info: Info(""), gas_wanted: Gas(2000000), gas_used: Gas(136249), events: [Event { type_str: "coin_spent", attributes: [Tag { key: Key("spender"), value: Value("wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf") }, Tag { key: Key("amount"), value: Value("2000000umlg") }] }, Event { type_str: "coin_received", attributes: [Tag { key: Key("receiver"), value: Value("wasm17xpfvakm2amg962yls6f84z3kell8c5l69j4zk") }, Tag { key: Key("amount"), value: Value("2000000umlg") }] }, Event { type_str: "transfer", attributes: [Tag { key: Key("recipient"), value: Value("wasm17xpfvakm2amg962yls6f84z3kell8c5l69j4zk") }, Tag { key: Key("sender"), value: Value("wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf") }, Tag { key: Key("amount"), value: Value("2000000umlg") }] }, Event { type_str: "message", attributes: [Tag { key: Key("sender"), value: Value("wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("fee"), value: Value("2000000umlg") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("acc_seq"), value: Value("wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf/5") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("signature"), value: Value("dHGDLpzY8zHIO/E1K6MHTNWkbF5RMlYbYzTzlIqpjzgfUfk7EO7L0hC7mHoJO+9lQJhV01JJMnAWDQToe+RogA==") }] }], codespace: Codespace("wasm") }
}

#[ignore]
#[tokio::test]
async fn test_execute_increment() {
    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();

    let msg = json!({
        "increment": {"count": 5u64}
    });

    execute::send_execute(
        &key,
        "malaga-420",
        "https://rpc.malaga-420.cosmwasm.com:443",
        "umlg",
        "wasm",
        10000,
        "wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
        1411,
    )
    .await
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"amount","value":"10000umlg"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"method","value":"try_increment"}]}]}]
}

#[ignore]
#[tokio::test]
async fn test_execute_reset() {
    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();

    // This should be failed since the count is above 10
    let msg = json!({
        "reset": {"count": 50u64}
    });

    execute::send_execute(
        &key,
        "malaga-420",
        "https://rpc.malaga-420.cosmwasm.com:443",
        "umlg",
        "wasm",
        10000,
        "wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
        1411,
    )
    .await
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"amount","value":"10000umlg"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"method","value":"reset"}]}]}]
}

#[ignore]
#[tokio::test]
async fn test_store_contract() {
    // Sender publickey {"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Aggx3Gp4SJOHzZK4WDen/j5EXutf78JB87DQA5/7Z59y"}
    // Sender account id wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf
    // Mnemonic "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry"
    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();
    deploy::store_contract(
        &key,
        "../simple-counter/artifacts/simple_counter-aarch64.wasm",
        "https://rpc.malaga-420.cosmwasm.com:443",
        "malaga-420",
        1411,
        "umlg",
        None,
        2000000,
        2000000,
        "wasm",
    )
    .await;
    // [{"events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"547"}]}]}]
    // code_id = 547
}

#[ignore]
#[tokio::test]
async fn test_instantiate_contract() {
    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();
    let (_, sender_account_id) = private_to_pub_and_account(&key, "wasm");

    let msg = json!({
        "count": 100u64,
        "auth": [sender_account_id.to_string()]
    });

    deploy::instantiate_contract(
        &key,
        547,
        "https://rpc.malaga-420.cosmwasm.com:443",
        "malaga-420",
        1411,
        "umlg",
        None,
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        "wasm",
        10000,
    )
    .await;
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"amount","value":"10000umlg"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"instantiate","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"code_id","value":"547"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgInstantiateContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"sender","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"amount","value":"10000umlg"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae"},{"key":"method","value":"instantiate"},{"key":"auth","value":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"},{"key":"count","value":"100"}]}]}]
    // contract_address = wasm1rpfxxy379eq2lq8wjz0lcke9ql49p5uzx2246vx6pml7yvd954tstdaaae
}
