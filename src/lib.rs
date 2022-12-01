use crate::router::{switch, Route};
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod router;
use serde::{self, Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Accounts {
    pub address: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {

        if *first_load {
            //code
            first_load.set(false);
        }
        || {}
    });

    html! (
       <BrowserRouter>
           <Switch<Route> render={switch} />
       </BrowserRouter>
    )
}
