
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::profile_validation::profile_validation_schelling_game::change_period_transaction_condition::ConditionalTransactionModal as ProfileValidationModal;

#[derive(Clone, Routable, PartialEq)]
pub enum ValidationRoute {
    #[at("/validation/profile-change-period/:profile_user_account")]
    ProfileChangePeriod { profile_user_account: String },
}

pub fn switch_validation(route: ValidationRoute) -> Html {
    match route {
        ValidationRoute::ProfileChangePeriod {
            profile_user_account,
        } => html! {<ProfileValidationModal profile_user_account={profile_user_account}/> },
    }
}
