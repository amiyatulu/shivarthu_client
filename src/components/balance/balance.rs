use crate::constants::constant::NODE_URL;
use gloo::console::log;
use sp_core::crypto::AccountId32;
use subxt::config::PolkadotConfig;
use wasm_bindgen_futures;
use yew::prelude::*;
use yewdux::prelude::*;
use sp_core::crypto::Ss58Codec;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub account_id: String,
}

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[function_component(Balance)]
pub fn balance(props: &Props) -> Html {
    let account_id = props.account_id.clone();
    log!(format!("{:?}", account_id));

    let account_id32 = AccountId32::from_string(&account_id).unwrap();
    let account_id32_clone = account_id32.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let balance_storage = polkadot::storage().system().account(account_id32_clone);
                let balance_details = client
                    .storage()
                    .fetch(&balance_storage, None)
                    .await
                    .unwrap();

                log!(format!("{:?}", balance_details));
            });
        },
        account_id32,
    );
    html! {
        <>
        </>
    }
}
