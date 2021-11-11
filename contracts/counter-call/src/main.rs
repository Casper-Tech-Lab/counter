#![no_main]

extern crate alloc;

use casper_types::{runtime_args::RuntimeArgs, ApiError, ContractHash, Key};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

const COUNTER_KEY: &str = "counter";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";

#[no_mangle]
pub extern "C" fn call() {
    // カウンタースマートコントラクトのContractHashを取得
    let contract_hash = {
        let counter_uref = runtime::get_key(COUNTER_KEY).unwrap_or_revert_with(ApiError::GetKey);
        if let Key::Hash(hash) = counter_uref {
            ContractHash::new(hash)
        } else {
            runtime::revert(ApiError::User(66));
        }
    };

    // エントリーポイント"counter_get"をCallし、現在の値を取得する
    let current_counter_value: u32 =
        runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    // エントリーポイント"counter_inc"をCallし、値をインクリメントする
    let _: () = runtime::call_contract(contract_hash, COUNTER_INC, RuntimeArgs::new());

    // エントリーポイント"counter_get"を再びCallし、新しい値を取得する
    let new_counter_value: u32 =
        runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    // インクリメントされた値が1でなければ、リバートする
    if new_counter_value - current_counter_value != 1u32 {
        runtime::revert(ApiError::User(67));
    }
}
