use subxt::PolkadotConfig;
use yew::prelude::*;

use crate::constants::constant::NODE_URL;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;

use crate::components::profile_validation::profile::last_citizen_id::use_last_citizen_id;
use crate::components::common_component::pagination::Pagination;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}


#[function_component(ViewProfiles)]
pub fn view_profiles() -> Html {

    let page_size = 10;
    let current_page_state = use_state(||1);
    let current_page_state_clone = current_page_state.clone();
    let current_page_state_clone2 = current_page_state.clone();


    let on_page_change = Callback::from(move |value: u64| {
        gloo::console::log!(value);
        current_page_state_clone.set(value);
    });
    html! {
        <>
        <div class="container">
        <Pagination on_page_change={on_page_change} total_count=15 sibling_count=1 current_page={*current_page_state_clone2} page_size={page_size} class_name={Some("hello")} />
        </div>
        </>
    }
}
