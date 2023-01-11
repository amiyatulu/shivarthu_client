use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component(ClearLocalStorage)]
pub fn clear_local_storage() -> Html {
    let first_load = use_state(|| true);
    let first_load_clone = first_load.clone();
    use_effect(move || {
        if *first_load_clone {
            LocalStorage::clear();
            first_load_clone.set(false);
        }
        || {}
    });

    html! {
        if *first_load == false {
            <h1>{"Successfully signed out"}</h1>
        }

    }
}
