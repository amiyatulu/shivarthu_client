use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::navigation::nav::Nav;

const STYLE_FILE: &str = include_str!("account_home.css");

#[styled_component(AccountHome)]
pub fn account_home() -> Html {

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!{
        <>
            <Nav/> 
                <div class={classes!("container", stylesheet)}>
                    <div class="card shadow-lg p-3 mb-5 bg-body rounded home">
                        <div class="d-grid gap-3">
                            <button type="button" class="btn btn-warning">{"Sign In"}</button>
                            <button type="button" class="btn btn-warning">{"Generate Account"}</button>                            
                        </div>
                    </div>
                </div>
        </>
    }
}