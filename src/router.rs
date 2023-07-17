use crate::components::pages::storage::Storage;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::rpc_call::Rpc;
// use crate::components::pages::transaction::Transaction;
use crate::components::pages::fileupload::FileUpload;
use crate::components::accounts::add_accounts::AddAccounts;
use crate::components::accounts::account_home::AccountHome;
use crate::components::accounts::create_account::CreateAccount;
// use crate::components::pages::transaction_from_hook::TransactionFromHooks;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
// use crate::components::pages::conditional_transaction_modal::ConditionalTransactionModal;
use crate::components::accounts::clear_local_storage::ClearLocalStorage;
use crate::components::markdown::markdown_component::MarkdownComponent;
use crate::components::profile_validation::profile::add_profile::AddProfile;
use crate::components::ipfs::form_ipfs_upload::FormIpfsUpload;
use crate::components::jstests::first_test::FirstTest;
use crate::components::profile_validation::profile::view_profile::ViewProfile;
use crate::components::profile_validation::profile_validation_schelling_game::challenger_evidence::ChallengerEvidence;
use crate::components::ai::chat_huggingface::ChatHuggingFace;
use crate::components::profile_validation::profile_validation_schelling_game::add_profile_stake::AddProfileStake;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_rpc::staking_period_end_block::StakingPeriodEndBlock;


#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/storage")]
    Storage,
    #[at("/rpc")]
    Rpc,
    // #[at("/transaction")]
    // Transaction,
    #[at("/upload")]
    FileUpload,
    #[at("/seed")]
    AddAccounts,
    #[at("/")]
    AccountHome,
    #[at("/create-account")]
    CreateAccount,
    // #[at("/hook-tx")]
    // TransactionFromHooks,
    #[at("/password")]
    SetPhraseFromPass,
    // #[at("/conditional-transaction")]
    // ConditionalTransactionModal,
    #[at("/signout")]
    ClearLocalStorage,
    #[at("/markdown")]
    MarkdownComponent,
    #[at("/add-profile")]
    AddProfile,
    #[at("/ipfs-form")]
    FormIpfsUpload,  
    #[at("/chat-huggingface")]
    ChatHuggingFace,
    #[at("/first-test")]
    FirstTest,
    #[at("/view-profile")]
    ViewProfile,
    #[at("/challenger-evidence")]
    ChallengerEvidence,
    #[at("/add-profile-stake/:profile_user_account")]
    AddProfileStake { profile_user_account: String},
    #[at("/staking-end-period")]
    StakingPeriodEndBlock,

}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Storage => html! { <Storage />},
        Route::Rpc => html! {<Rpc/>},
        // Route::Transaction => html! {<Transaction/>},
        Route::FileUpload => html! {<FileUpload/>},
        Route::AddAccounts => html! {<AddAccounts/>},
        Route::AccountHome => html! {<AccountHome/>},
        Route::CreateAccount => html! {<CreateAccount/>},
        // Route::TransactionFromHooks => html! {<TransactionFromHooks/>},
        Route::SetPhraseFromPass => html!{<SetPhraseFromPass/>},
        // Route::ConditionalTransactionModal => html!{<ConditionalTransactionModal/>},
        Route::ClearLocalStorage => html! {<ClearLocalStorage/>},
        Route::MarkdownComponent => html! {<MarkdownComponent/>},
        Route::AddProfile => html! {<AddProfile/>},
        Route::FormIpfsUpload => html! {<FormIpfsUpload/>},
        Route::FirstTest => html! {<FirstTest/>},
        Route::ViewProfile => html! {<ViewProfile/>},
        Route::ChallengerEvidence => html! {<ChallengerEvidence/>},
        Route::ChatHuggingFace => html!{<ChatHuggingFace/>},
        Route::AddProfileStake { profile_user_account } => html!{<AddProfileStake profile_user_account={profile_user_account}/>},
        Route::StakingPeriodEndBlock => html!{<StakingPeriodEndBlock/>},
    }
}