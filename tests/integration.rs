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

use cosmwasm_std::{ coins, from_binary, Binary, Response };
use cosmwasm_vm::testing::{
    mock_env, mock_info, mock_instance, instantiate, execute, query, mock_instance_with_gas_limit,
};
use std::time::Instant;

use rand::msg::{ExecuteMsg, InstantiateMsg, LatestResponse, QueryMsg};
static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/rand.wasm");

fn pubkey_loe_mainnet() -> Binary {
    vec![
        134, 143, 0, 94, 184, 230, 228, 202, 10, 71, 200, 167, 124, 234, 165, 48, 154, 71, 151,
        138, 124, 113, 188, 92, 206, 150, 54, 107, 93, 122, 86, 153, 55, 197, 41, 238, 218, 102,
        199, 41, 55, 132, 169, 64, 40, 1, 175, 49,
    ]
    .into()
}

const BOUNTY_DENOM: &str = "ucosm";

#[test]
fn proper_initialization() {
    // let mut instance = mock_instance(WASM, &[]);

    // let msg = InstantiateMsg {
    //     pubkey: pubkey_loe_mainnet(),
    //     bounty_denom: BOUNTY_DENOM.into(),
    // };
    // let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    // let res = call_instantiate(&mut instance, mock_env(), info, msg).unwrap();
    // assert_eq!(res.messages.len(), 0);
}

#[test]
fn verify_valid() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000);

    let msg = InstantiateMsg {
        pubkey: pubkey_loe_mainnet(),
        bounty_denom: BOUNTY_DENOM.into(),
    }; 
    let info = mock_info("creator", &[]);
    let _res: Response = instantiate(&mut deps, mock_env(), info.clone(), msg).unwrap();

    let time_before = Instant::now();
    let gas_before = deps.get_gas_left();

    let msg = ExecuteMsg::Add {
        round: 72785,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };

    let _res: Response = execute(&mut deps, mock_env(), info, msg).unwrap();
    let gas_used = gas_before - deps.get_gas_left();
    println!("Gas used: {}", gas_used);
    println!("Time elapsed: {:.2?}", time_before.elapsed());

    let latest: LatestResponse =
        from_binary(&query(&mut deps, mock_env(), QueryMsg::Latest {}).unwrap()).unwrap();
    assert_eq!(latest.round, 42);
    assert_eq!(
        latest.randomness,
        hex::decode("a9f12c5869d05e084d1741957130e1d0bf78a8ca9a8deb97c47cac29aae433c6"
    ).unwrap()
);

}

// #[test]
// fn verify_invalid() {
//     let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000);

//     let msg = InstantiateMsg {
//         pubkey: pubkey_loe_mainnet(),
//         bounty_denom: BOUNTY_DENOM.into(),
//     };
//     let info = mock_info("creator", &[]);
//     call_instantiate(&mut deps, &mock_env(), &info, &to_binary(&msg).unwrap().to_array().unwrap()).unwrap();

//     let gas_before = deps.get_gas_left();

//     let info = mock_info("anyone", &[]);
//     let msg = ExecuteMsg::Add {
//         // curl -sS https://drand.cloudflare.com/public/72785
//         round: 42,
//         previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
//         signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
//     };
//     call_execute(&mut deps, &mock_env(), &info, &to_binary(&msg).unwrap().to_array().unwrap());
//     // let error_msg = result.into_result().unwrap_err();
//     // assert_eq!(error_msg, "Signature verification failed");

//     let gas_used = gas_before - deps.get_gas_left();
//     println!("Gas used: {}", gas_used);
// }