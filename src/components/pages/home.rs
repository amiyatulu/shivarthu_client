use gloo::console::log;
use wasm_bindgen_futures;
use yew::prelude::*;

use sp_keyring::AccountKeyring;
use subxt::config::PolkadotConfig;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[function_component(Home)]
pub fn home() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            wasm_bindgen_futures::spawn_local(async move {
                let client =
                    subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
                        .await
                        .unwrap();
                let key = polkadot::runtime_types::sortition_sum_game::types::SumTreeName::UniqueIdenfier1 {
                    citizen_id: 0,
                    name: "challengeprofile".as_bytes().to_vec(),
                };
                let period_storage = polkadot::storage().template_module().period_name(&key);
                let period = client.storage().fetch(&period_storage, None).await.unwrap();
                log!(format!("{:?}", period));
            });
            first_load.set(false);
        }
        || {}
    });

    html! {
        <h1>{"Home"}</h1>
    }
}
