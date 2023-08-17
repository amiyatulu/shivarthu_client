use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::profile_validation::profile_validation_schelling_game::challenger_evidence_transaction_condition::ConditionalTransactionModal;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_storage_call::get_challenger_fees::GetChallengerFees;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_rpc::evidence_end_block::EvidenceEndBlock;

use gloo::console::log;
use json::object;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(ChallengerEvidence)]
pub fn challenger_evidence(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let evidence_markdown_state: UseStateHandle<Option<String>> = use_state(|| None);
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let ipfs_response: UseStateHandle<Option<String>> = use_state(|| None);
    let ipfs_response_clone = ipfs_response.clone();

    let evidence_markdown_state_clone = evidence_markdown_state.clone();
    let evidence_markdown_state_clone_onsubmit = evidence_markdown_state.clone();

    let spinner_clone = spinner_state.clone();

    let markdown_changed = Callback::from(move |markdown: String| {
        let markdown_value: Option<String> = if markdown.is_empty() {
            None
        } else {
            Some(markdown)
        };

        evidence_markdown_state_clone.set(markdown_value);
    });

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        let ipfs_response_onsubmit = ipfs_response.clone();
        if evidence_markdown_state_clone_onsubmit.is_some() {
            let details = format!(
                "{}",
                evidence_markdown_state_clone_onsubmit
                    .as_deref()
                    .unwrap_or_default()
            );
            let data = object! {
                  version: "1.0",
                  details: details,
            };

            let json_string = json::stringify(data);
            let spinner_clone = spinner_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    ipfs_call_json_string(DEFAULT_IPFS_PROVIDER, &json_string, "ipfs".to_owned())
                        .await;
                ipfs_response_onsubmit.set(Some(response.clone()));
                spinner_clone.set(Some(false));
                log!(response);
            });
        }
    });

    if ipfs_response_clone.is_none() {
        html! {
            <>
                <Nav/>
                <div class="container">
                <EvidenceEndBlock profile_user_account={profile_user_account.clone()} />
                <GetChallengerFees profile_user_account={profile_user_account.clone()}/>
                    <form onsubmit={onsubmit}>
                        <div class="mb-3">
                            <label for="details" class="form-label">{"Details"}</label>
                            <MarkdownField name={"details"} class={"form-control"} required={true} handle_onchange={markdown_changed}/>
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
        let ipfs_response = format!("{}", ipfs_response_clone.as_deref().unwrap_or_default());
        html! {
            <>
            <ConditionalTransactionModal ipfs_response={ipfs_response} profile_user_account={profile_user_account}/>
            </>
        }
    }
}
