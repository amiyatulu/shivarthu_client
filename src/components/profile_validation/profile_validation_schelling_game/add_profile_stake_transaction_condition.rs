use yew::prelude::*;
use yewdux::prelude::*;




use crate::components::navigation::nav::Nav;
use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
// use crate::components::accounts::hooks::profile_validation::add_profile_stake_hooks;
use crate::components::accounts::hooks::commons::TransactionReturnKind;
use crate::js_extension_binding;
use crate::components::accounts::hooks::custom_extrinsics_hook::use_custom_extrinsic;






#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
    pub amount_to_fund: u32,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let amount_to_fund = props.amount_to_fund.clone();

    let promise_creator = move |node_url: &String, seed: &String| {
            js_extension_binding::add_profile_stake(
                node_url.to_owned(),
                seed.to_owned(),
                profile_user_account,
                amount_to_fund,
            )
    };

    let hookdata = use_custom_extrinsic(promise_creator);

    // let hookdata = add_profile_stake_hooks::use_profile_stake(profile_user_account, amount_to_fund);

    html! {
        <>
        <Nav />
            <div class="container">
                <h1>{"Transaction details"}</h1>
                <p>
                {
                    match hookdata.kind {
                        TransactionReturnKind::Finalized => {hookdata.value}
                        TransactionReturnKind::Error => {hookdata.value}
                        TransactionReturnKind::InBlock => {hookdata.value}
                        TransactionReturnKind::Processing => {hookdata.value}
                    }
                }
                </p>
            </div>
        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let amount_to_fund = props.amount_to_fund.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  profile_user_account={profile_user_account} amount_to_fund={amount_to_fund}/>
        </>
       }
    }
}
