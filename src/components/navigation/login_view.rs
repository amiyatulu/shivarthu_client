use crate::components::accounts::account_store::AccountPubStore;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(LoginView)]
pub fn sign_out_view() -> Html {
    let (store, _) = use_store::<AccountPubStore>();

    let address_option = store.account_address.as_deref();
    html! {
        if let Some(address) = address_option {
        <>
        <li class="nav-item">
        <Link<Route> to={Route::AddProfile} classes="nav-link">{"Add Profile"}</Link<Route>>
        </li>

        <li class="nav-item">
        <Link<Route> to={Route::ViewProfileAddress {profile_user_account: address.to_string()} } classes="nav-link">{"View Profile"}</Link<Route>>
        </li>

        <li class="nav-item">
        <Link<Route> to={Route::AddProfileStake {profile_user_account: address.to_string()}} classes="nav-link">{"Add Profile Stake"}</Link<Route>>
        </li>

        </>
      }

    }
}
