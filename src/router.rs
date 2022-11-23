use crate::components::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
    }
}