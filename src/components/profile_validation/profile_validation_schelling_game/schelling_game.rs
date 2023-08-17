use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_hook::period_hook::polkadot::runtime_types::schelling_game_shared::types::Period;

use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_hook::period_hook::use_get_period;
use crate::components::profile_validation::profile_validation_schelling_game::challenger_evidence::ChallengerEvidence;
use crate::components::profile_validation::profile_validation_schelling_game::add_apply_jurors::AddApplyJurors;
use crate::components::profile_validation::profile_validation_schelling_game::draw_jurors::DrawJurors;
use crate::components::profile_validation::profile_validation_schelling_game::commit_vote::CommitVote;
use crate::components::profile_validation::profile_validation_schelling_game::reveal_vote::RevealVote;

#[function_component(SchellingGame)]
pub fn schelling_game(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let period_name = use_get_period(profile_user_account.clone());

    let myview = if let Some(period) = period_name {
        let myview = match period {
            Period::Evidence => html! {
                <>
             <ChallengerEvidence profile_user_account={profile_user_account.clone()}/>
                </>
            },
            Period::Staking => {
                html! {<AddApplyJurors profile_user_account={profile_user_account.clone()}/>}
            }
            Period::Drawing => html! {
                <>
                <DrawJurors profile_user_account={profile_user_account.clone()} />
                </>
            },
            Period::Commit => html! {
                <>
                <CommitVote profile_user_account={profile_user_account.clone()} />
                </>
            },
            Period::Vote => html! {
                <>
                <RevealVote profile_user_account={profile_user_account} />
                </>
            },
            Period::Appeal => html! {
                <>
                </>
            },
            Period::Execution => html! {
                <>
                </>
            },
        };
        myview
    } else {
        html! {
            <>
            <div class="container">
                <p>{"No period"}</p>
            </div>
            </>
        }
    };

    html! {
        <>
        {myview}
        </>
    }
}
