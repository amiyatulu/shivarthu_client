use gloo::console::log;
use wasm_bindgen_futures;
use yew::prelude::*;
use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    PolkadotConfig,
};
use crate::services::common_services::polkadot;


#[function_component(Transaction)]
pub fn transaction() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            wasm_bindgen_futures::spawn_local(async move {
                let client =
                    subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
                        .await
                        .unwrap();
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let profile_data = "challengeprofile".as_bytes().to_vec();
                let tx = polkadot::tx().template_module().add_citizen(profile_data);
                let hash = client.tx().sign_and_submit_default(&tx, &signer).await.unwrap();
                log!(format!("{:?}", hash));
            });
            first_load.set(false);
        }
        || {}
    });

    html! {
        <h1>{"Transaction"}</h1>
    }
}
