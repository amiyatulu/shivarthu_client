use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_storage_call::get_period::GetPeriod;
use crate::components::profile_validation::profile_validation_schelling_game::add_profile_jurors_transaction_condition::ConditionalTransactionModal;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_rpc::staking_end_block::StakingEndBlock;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(AddApplyJurors)]
pub fn add_apply_jurors(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let add_juror_stake_state: UseStateHandle<Option<u32>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();

    let add_juror_stake_state_clone = add_juror_stake_state.clone();

    let add_juror_stake_state_clone2 = add_juror_stake_state.clone();

    let add_juror_stake_state_else_clone = add_juror_stake_state.clone();

    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if add_juror_stake_state_clone2.is_some() {
            submit_done.set(true);
        }
    });

    let juror_stake_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u32>()
            .expect("Invalid input");
        add_juror_stake_state_clone.set(Some(value));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
            <GetPeriod profile_user_account={profile_user_account.clone()}/>
            <StakingEndBlock profile_user_account={profile_user_account.clone()}/>
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="juror-stake" class="form-label">{"Juror Stake:"}</label>
                <input name={"juror-stake"} type="number" class={"form-control"} required={true} onchange={juror_stake_onchanged}/>
                </div>
                if let Some(_value) = *spinner_state {
                    <input type="submit" value="Submit" disabled={true} />
                    <img src="img/rolling.gif" alt="loading" width="40"/>
                } else {
                    <input type="submit" value="Submit" />
                }

                </form>
            </div>
            </>
        }
    } else {
        let amount_to_fund = (*add_juror_stake_state_else_clone).unwrap();

        html! {
            <>
            <ConditionalTransactionModal amount_to_fund={amount_to_fund} profile_user_account={profile_user_account}/>
            </>
        }
    }
}
