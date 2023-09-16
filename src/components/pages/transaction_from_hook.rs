use yew::prelude::*;

use crate::components::accounts::sign_tx_hook::use_sign_tx;
use crate::services::common_services::polkadot;


#[function_component(TransactionFromHooks)]
pub fn transaction() -> Html {
    let first_load = use_state(|| true);
    let profile_data = "challengeprofile".as_bytes().to_vec();
    let tx = polkadot::tx().template_module().add_citizen(profile_data);

    let hash = use_sign_tx(tx);

    html! {
        <>
            <h1>{"Transaction"}</h1>
            <p>{"Hash: "}{hash}</p>
        </>

    }
}
