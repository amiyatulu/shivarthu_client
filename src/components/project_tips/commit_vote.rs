use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::ops::Deref;
use crate::components::project_tips::commit_vote_transaction_condition::ConditionalTransactionModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub department_required_fund_id: u64,
}

#[function_component(CommitVotePositiveExternality)]
pub fn commit_vote(props: &Props) -> Html {
    let department_required_fund_id = props.department_required_fund_id.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let hash_state: UseStateHandle<Option<[u8; 32]>> = use_state(|| None);
    let commit_vote_state: UseStateHandle<Option<String>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();
    let commit_vote_state_clone = commit_vote_state.clone();
    let commit_vote_state_onchange = commit_vote_state.clone();
    let commit_vote_state_view = commit_vote_state.clone();


    let hash_state_clone = hash_state.clone();


    let hash_state_else_clone = hash_state.clone();

    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if commit_vote_state_clone.is_some() {
            let commit_vote = format!("{}", commit_vote_state_clone.deref().clone().unwrap());
            let hash = sp_core_hashing::keccak_256(commit_vote.as_bytes());
            hash_state_clone.set(Some(hash));
            submit_done.set(true);
        }
    });

    let commit_vote_onchanged = Callback::from(move |event: KeyboardEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        commit_vote_state_onchange.set(Some(value.clone()));
       
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
    
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="commit vote" class="form-label">{"Commit Vote:"}</label>
                <input name={"commit vote"} type="text" class={"form-control"} required={true} onkeyup={commit_vote_onchanged}/>
                </div>
                if let Some(myvote) = commit_vote_state_view.deref().clone() {
                    <div>{myvote}</div>
                }
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
        let hash = hash_state_else_clone.unwrap();

        html! {
            <>
            <ConditionalTransactionModal hash={hash} department_required_fund_id={department_required_fund_id}/>
            </>
        }
    }
}
