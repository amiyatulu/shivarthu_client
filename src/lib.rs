use crate::router::{switch, Route};
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod constants;
mod services;
mod router;
mod js_extension_binding;
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Accounts {
    pub address: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    html! (
        <>
            // <Nav/>    
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
       </>
    )
}
