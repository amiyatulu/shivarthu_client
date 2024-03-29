use subxt::PolkadotConfig;
use yew::prelude::*;

use crate::constants::constant::NODE_URL;
use wasm_bindgen_futures;
use crate::services::common_services::polkadot;

#[function_component(FetchStorageProfile)]
pub fn fetch_strorage_profile() -> Html {
    let last_citizen: UseStateHandle<Option<u64>> = use_state(|| None);
    let last_citizen_clone = last_citizen.clone();
    let last_citizen_return = last_citizen.clone();

    use_effect_with((),
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();

                // let last_citizen_storage =
                //     polkadot::storage().profile_validation().citizen_profile_root();

                // let mut last_citizen = client
                //     .storage()
                //     .at_latest()
                //     .await
                //     .unwrap()
                //     .iter(last_citizen_storage)
                //     .await
                //     .unwrap();

                //     while let Some((key, value)) = last_citizen.next().await.unwrap() {
                //         println!("Key: 0x{}", hex::encode(&key));
                //         println!("Value: {:?}", value);
                //     }

                
            });
        },
    );

   html!{
    <>
    </>
   }
}
