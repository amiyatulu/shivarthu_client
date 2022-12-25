use yew::prelude::*;
use crate::components::accounts::multistep_account_creation::info_warning::InfoWarning;

#[function_component(CreateAccount)]
pub fn create_account() -> Html {
  let step_state = use_state(|| 0);

  let prev_step = {
      let state = step_state.clone();
      Callback::from(move |_| state.set(*state + 1))
  };

  html! {
      <>
          <p> {"current count: "} {*step_state} </p>
          <InfoWarning continue_onclick={prev_step} />
      </>
  }
}