use gloo::console::log;
use wasm_bindgen_futures;
use yew::{prelude::*};
use subxt::config::PolkadotConfig;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[function_component(Storage)]
pub fn storage() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            wasm_bindgen_futures::spawn_local(async move {
                let client =
                    subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
                        .await
                        .unwrap();
                // let key = polkadot::runtime_types::sortition_sum_game::types::SumTreeName::UniqueIdenfier1 {
                //     citizen_id: 0,
                //     name: "challengeprofile".as_bytes().to_vec(),
                // };
                // let period_storage = polkadot::storage().profile_validation().period_name(&key);
                // let period = client.storage().at_latest().await.unwrap().fetch(&period_storage).await.unwrap();
                // log!(format!("{:?}", period));
                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
            first_load.set(false);
        }
        || {}
    });

    html! {
        <h1>{"Storage"}</h1>
    }
}
