use subxt::PolkadotConfig;
use yew::prelude::*;

use crate::constants::constant::NODE_URL;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;

use crate::components::common_component::pagination::Pagination;
use crate::components::profile_validation::profile::last_citizen_id::use_last_citizen_id;
use crate::services::common_services::polkadot;

fn range(start: u64, end: u64) -> Vec<u64> {
    (start..=end).collect()
}

#[function_component(ViewProfiles)]
pub fn view_profiles() -> Html {
    let page_size = 10;
    let current_page_state = use_state(|| 1);
    let current_page_state_clone = current_page_state.clone();
    let current_page_state_clone2 = current_page_state.clone();
    let current_page_state_clone_deps = current_page_state.clone();
    let current_page_state_clone_deps2 = current_page_state.clone();

    let last_citizen_id = use_last_citizen_id();

    // let profile_ids: UseStateHandle<Vec<u64>> = use_state(|| vec![]);

    let profile_ids = use_memo(
        move |_| {
            let first_page_index = (*current_page_state_clone_deps2 - 1) * page_size;
            let last_page_index = first_page_index + page_size;
            range(first_page_index, last_page_index)
        },
        current_page_state_clone_deps,
    );

    let on_page_change = Callback::from(move |value: u64| {
        gloo::console::log!(value);
        current_page_state_clone.set(value);
    });
    html! {
        <>
        <div class="container">
        <Pagination on_page_change={on_page_change} total_count={last_citizen_id.unwrap()} sibling_count=1 current_page={*current_page_state_clone2} page_size={page_size} class_name={Some("hello")} />
        </div>
        </>
    }
}
