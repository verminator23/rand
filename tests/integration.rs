//! This integration test tries to run and call the generated wasm.
//! It depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.
//!
//! You can easily convert unit tests to integration tests.
//! 1. First copy them over verbatum,
//! 2. Then change
//!      let mut deps = mock_dependencies(20, &[]);
//!    to
//!      let mut deps = mock_instance(WASM, &[]);
//! 3. If you access raw storage, where ever you see something like:
//!      deps.storage.get(CONFIG_KEY).expect("no data stored");
//!    replace it with:
//!      deps.with_storage(|store| {
//!          let data = store.get(CONFIG_KEY).expect("no data stored");
//!          //...
//!      });
//! 4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)

use cosmwasm_std::{coins, from_binary, HandleResponse, InitResponse};
use cosmwasm_vm::testing::{
    handle, init, mock_env, mock_info, mock_instance, mock_instance_with_gas_limit, query,
};

use rand::msg::{HandleMsg, InitMsg, LatestResponse, QueryMsg};

static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/rand.wasm");
// static WASM: &[u8] = include_bytes!("../artifacts/rand.wasm");

#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = InitMsg { round: 17 };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let res: InitResponse = init(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // it worked, let's query the state
    let res = query(&mut deps, mock_env(), QueryMsg::Latest {}).unwrap();
    let value: LatestResponse = from_binary(&res).unwrap();
    assert_eq!(value.round, 17);
}

#[test]
fn verify_valid() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000);

    let msg = InitMsg { round: 17 };
    let info = mock_info("creator", &[]);
    let _res: InitResponse = init(&mut deps, mock_env(), info, msg).unwrap();

    let gas_before = deps.get_gas_left();

    let info = mock_info("anyone", &[]);
    let msg = HandleMsg::Add {
        // curl -sS https://drand.cloudflare.com/public/72785
        round: 72785,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };
    let res: HandleResponse = handle(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(res.data.unwrap().as_slice(), [0x01]);

    let gas_used = gas_before - deps.get_gas_left();
    println!("Gas used: {}", gas_used);
}

#[test]
fn verify_invalid() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000);

    let msg = InitMsg { round: 17 };
    let info = mock_info("creator", &[]);
    let _res: InitResponse = init(&mut deps, mock_env(), info, msg).unwrap();

    let gas_before = deps.get_gas_left();

    let info = mock_info("anyone", &[]);
    let msg = HandleMsg::Add {
        // curl -sS https://drand.cloudflare.com/public/72785
        round: 42,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };
    let res: HandleResponse = handle(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(res.data.unwrap().as_slice(), [0x00]);

    let gas_used = gas_before - deps.get_gas_left();
    println!("Gas used: {}", gas_used);
}
