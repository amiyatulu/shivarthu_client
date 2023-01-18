use crate::components::accounts::account_store::AccountPubStore;
use crate::constants::constant::NODE_URL;
use gloo::console::log;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Ss58Codec;
use subxt::config::PolkadotConfig;
use wasm_bindgen_futures;
use yew::prelude::*;
use yewdux::prelude::*;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[function_component(Balance)]
pub fn balance() -> Html {
    let (store, _) = use_store::<AccountPubStore>();
    let address_option = store.account_address.clone();
    let account_id = address_option.clone().unwrap();

    let free_balance = use_state(|| 0);
    let free_balance_clone = free_balance.clone();
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

                if let Some(balance_details) = balance_details {
                    // log!(format!("{:?}", balance_details.data.free));
                    free_balance_clone.set(balance_details.data.free)
                }
            });
        },
        account_id32,
    );
    html! {
        <>
        if let Some(_address) = address_option {
            <>


            {*free_balance}

            </>
        }
        </>
    }
}
