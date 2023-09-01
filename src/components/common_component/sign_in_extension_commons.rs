use anyhow::anyhow;
use futures::FutureExt;

use subxt::{OnlineClient, PolkadotConfig};

use subxt::ext::codec::{Decode, Encode};
use subxt::tx::SubmittableExtrinsic;
use subxt::tx::TxPayload;
use subxt::utils::{AccountId32, MultiSignature};

use crate::services::common_services::{extension_signature_for_extrinsic, get_accounts, polkadot, Account};
use web_sys::HtmlInputElement;
use yew::prelude::*;


pub enum SigningStage {
    Error(String),
    CreatingOnlineClient,
    EnterMessage,
    RequestingAccounts,
    SelectAccount(Vec<Account>),
    Signing(Account),
    SigningSuccess {
        signer_account: Account,
        signature: MultiSignature,
        signed_extrinsic_hex: String,
        submitting_stage: SubmittingStage,
    },
}

pub enum SubmittingStage {
    Initial {
        signed_extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    },
    Submitting,
    Success {
        remark_event: polkadot::system::events::ExtrinsicSuccess,
    },
    Error(anyhow::Error),
}


// #[function_component(SigningExamplesComponent)]
// pub fn signing_examples_component() -> Html {
//     let stage = use_state(|| SigningStage::CreatingOnlineClient);
//     let stage_message_html = stage.clone();
//     let message = use_state(|| "".to_string());


//     let message_html: Html = match *stage_message_html {
//         SigningStage::Error(_)
//         | SigningStage::EnterMessage
//         | SigningStage::CreatingOnlineClient => html!(<></>),
//         _ => {
//             html!(
//                 <>
//                 </>
//             )
           
//         }
//     };
//     html! {
//         <>
//         </>
//     }

// }