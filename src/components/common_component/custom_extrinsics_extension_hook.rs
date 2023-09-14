use crate::constants::constant::NODE_URL;
use crate::services::common_services::{
    extension_signature_for_extrinsic, polkadot, Account,
};
use anyhow::anyhow;
use std::ops::Deref;
use subxt::ext::codec::{Decode, Encode};
use subxt::tx::SubmittableExtrinsic;
use subxt::utils::{AccountId32, MultiSignature};
use subxt::{OnlineClient, PolkadotConfig};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct ExtensionReturn {
    pub error: Option<String>,
    pub extrinsic_success: Option<String>,
    pub extrinsic_error: Option<String>,
}

#[hook]
pub fn use_sign_tx_extension<T>(tx: T, account_address: String, account_source: String) -> ExtensionReturn
where
    T: subxt::tx::TxPayload + 'static,
{
    let error_state: UseStateHandle<Option<anyhow::Error>> = use_state(|| None);
    let extrinsic_success: UseStateHandle<Option<String>> = use_state(|| None);
    let extrinsic_error: UseStateHandle<Option<String>> = use_state(|| None);
    let error_state_clone = error_state.clone();
    let error_state_return_clone = error_state.clone();
    let extrinsic_success_clone = extrinsic_success.clone();
    let extrinsic_error_clone = extrinsic_error.clone();
    let account_id: AccountId32 = account_address.parse().unwrap();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let error_state_clone = error_state_clone.clone();

                let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();

                let account_nonce_option = match api.tx().account_nonce(&account_id).await {
                    Ok(account_nonce) => Some(account_nonce),
                    Err(_) => {
                        error_state_clone.set(Some(anyhow!("Fetching account nonce failed")));
                        None
                    }
                };

                let mut call_data_option = None;

                if account_nonce_option.is_some() {
                    call_data_option = match api.tx().call_data(&tx) {
                        Ok(call_data) => Some(call_data),
                        Err(_) => {
                            error_state_clone.set(Some(anyhow!("could not encode call data")));
                            None
                        }
                    }
                }

                let mut signature_option = None;
                if account_nonce_option.is_some() && call_data_option.is_some() {
                    signature_option = match extension_signature_for_extrinsic(
                        &call_data_option.unwrap(),
                        &api,
                        account_nonce_option.unwrap(),
                        account_source,
                        account_address,
                    )
                    .await
                    {
                        Ok(signature) => Some(signature),
                        Err(_) => {
                            error_state_clone.set(Some(anyhow!("Signing via extension failed")));
                            None
                        }
                    }
                }

                let mut multi_signature_option = None;

                if signature_option.is_some() {
                    let signature = signature_option.unwrap();
                    multi_signature_option = match MultiSignature::decode(&mut &signature[..]) {
                        Ok(multi_signature) => Some(multi_signature),
                        Err(_) => {
                            error_state_clone.set(Some(anyhow!("MultiSignature Decoding")));
                            None
                        }
                    }
                }

                let mut partial_signed_option = None;
                if multi_signature_option.is_some() && account_nonce_option.is_some() {
                    partial_signed_option = match api.tx().create_partial_signed_with_nonce(
                        &tx,
                        account_nonce_option.unwrap(),
                        Default::default(),
                    ) {
                        Ok(partial_signed) => Some(partial_signed),
                        Err(_) => {
                            error_state_clone
                                .set(Some(anyhow!("PartialExtrinsic creation failed")));
                            None
                        }
                    }
                }

                let mut signed_extrinsic_option = None;
                if partial_signed_option.is_some() && multi_signature_option.is_some() {
                    let multi_signature = multi_signature_option.unwrap();
                    let partial_signed = partial_signed_option.unwrap();
                    let signed_extrinsic = partial_signed
                        .sign_with_address_and_signature(&account_id.into(), &multi_signature);
                    signed_extrinsic_option = Some(signed_extrinsic);
                }

                // if signed_extrinsic_option.is_some() {
                //     let signed_extrinsic = signed_extrinsic_option.unwrap();
                //     let dry_res = signed_extrinsic.validate().await;
                //     web_sys::console::log_1(&format!("Validation Result: {:?}", dry_res).into());
                // }
                if signed_extrinsic_option.is_some() {
                    let signed_extrinsic = signed_extrinsic_option.unwrap();

                    match submit_wait_finalized_and_get_extrinsic_success_event(signed_extrinsic)
                        .await
                    {
                        Ok(remark_event) => {
                            extrinsic_success_clone.set(Some(format!("{:?}", remark_event)));
                        }
                        Err(err) => {
                            extrinsic_error_clone.set(Some(err.to_string()));
                        }
                    }
                }
            });
        },
        (),
    );

    ExtensionReturn {
        error: error_state_return_clone.deref().as_ref().map(|err| err.to_string()),
        extrinsic_success: extrinsic_success.deref().clone(),
        extrinsic_error: extrinsic_error.deref().clone(),
    }
}

async fn submit_wait_finalized_and_get_extrinsic_success_event(
    extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
) -> Result<polkadot::system::events::ExtrinsicSuccess, anyhow::Error> {
    let events = extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    let events_str = format!("{:?}", &events);
    web_sys::console::log_1(&events_str.into());
    for event in events.find::<polkadot::system::events::ExtrinsicSuccess>() {
        web_sys::console::log_1(&format!("{:?}", event).into());
    }

    let success = events.find_first::<polkadot::system::events::ExtrinsicSuccess>()?;
    success.ok_or(anyhow!("ExtrinsicSuccess not found in events"))
}
