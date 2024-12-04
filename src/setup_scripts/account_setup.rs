use std::time::Duration;

use tokio::time::sleep;

use crate::contract_clients::config::Clients;
use crate::contract_clients::utils::{
    build_single_owner_account, declare_contract, deploy_account_using_priv_key, DeclarationInput, RpcAccount,
    TEMP_ACCOUNT_PRIV_KEY,
};
use crate::utils::constants::{OZ_ACCOUNT_CASM_PATH, OZ_ACCOUNT_PATH, OZ_ACCOUNT_SIERRA_PATH};
use crate::utils::{convert_to_hex, save_to_json, JsonValueType};
use crate::ConfigFile;

pub async fn account_init<'a>(clients: &'a Clients, arg_config: &'a ConfigFile) -> RpcAccount<'a> {
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // Making temp account for declaration of OZ account Cairo 1 contract
    let oz_account_class_hash = declare_contract(DeclarationInput::LegacyDeclarationInputs(
        String::from(OZ_ACCOUNT_PATH),
        arg_config.rollup_seq_url.clone(),
        clients.provider_l2(),
    ))
    .await;
    log::info!("OZ Account Class Hash Declared");
    save_to_json("oz_account_class_hash", &JsonValueType::StringType(oz_account_class_hash.to_string())).unwrap();
    sleep(Duration::from_secs(10)).await;

    log::info!("Waiting for block to be mined [/]");
    sleep(Duration::from_secs(10)).await;
    println!(">>> wait done");

    let account_address_temp =
        deploy_account_using_priv_key(TEMP_ACCOUNT_PRIV_KEY.to_string(), clients.provider_l2(), oz_account_class_hash)
            .await;
    println!(">>> account address temp deployed");
    sleep(Duration::from_secs(10)).await;
    println!(">>> wait done");

    let user_account_temp = build_single_owner_account(
        clients.provider_l2(),
        TEMP_ACCOUNT_PRIV_KEY,
        &convert_to_hex(&account_address_temp.to_string()),
        false,
    )
    .await;
    println!(">>> temp account built");
    let oz_account_caio_1_class_hash = declare_contract(DeclarationInput::DeclarationInputs(
        String::from(OZ_ACCOUNT_SIERRA_PATH),
        String::from(OZ_ACCOUNT_CASM_PATH),
        user_account_temp.clone(),
    ))
    .await;
    println!(">>> oz_account_caio_1_class_hash declared");
    save_to_json("oz_account_caio_1_class_hash", &JsonValueType::StringType(oz_account_caio_1_class_hash.to_string()))
        .unwrap();
    sleep(Duration::from_secs(10)).await;
    println!(">>> wait done");
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    // Using Account Cairo 1 contract
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    let account_address = deploy_account_using_priv_key(
        arg_config.rollup_priv_key.clone(),
        clients.provider_l2(),
        oz_account_caio_1_class_hash,
    )
    .await;
    println!(">>> cairo 1 account declared");
    save_to_json("account_address", &JsonValueType::StringType(account_address.to_string())).unwrap();
    println!(">>> account init done.");
    build_single_owner_account(
        clients.provider_l2(),
        &arg_config.rollup_priv_key,
        &convert_to_hex(&account_address.to_string()),
        false,
    )
    .await
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
}
