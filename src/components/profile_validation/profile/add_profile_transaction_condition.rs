use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::navigation::nav::Nav;
use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;

use crate::components::jstests::first_test::FirstTest;




#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipfs_response: String,
}



#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let profile_data = ipfs_response.as_bytes().to_vec();

    
    // let handler = use_sign_tx_handle();
    let first_load = use_state(|| true);

    use_effect( move || {
        

        if *first_load {
            wasm_bindgen_futures::spawn_local(async move {
                let tx = polkadot::tx().profile_validation().add_citizen(profile_data);
                // let data = (handler.api_call)(tx).await;
                // log!(data.unwrap());
            });
            

            first_load.set(false);
        }
        || {}

    });

    html! {
        <>
        <Nav />
            <div class="container">
                <h1>{"Transaction details"}</h1>
                // <p>{hash}</p>
                <FirstTest/>
            </div>
        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  ipfs_response={ipfs_response}/>
        </>
       }
    }
}
