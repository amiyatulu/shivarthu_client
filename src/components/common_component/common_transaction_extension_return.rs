use crate::components::common_component::custom_extrinsics_extension_hook::ExtensionReturn;
use crate::components::navigation::nav::Nav;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hookdata: ExtensionReturn,
}

#[function_component(CommonTransactionExtensionReturn)]
pub fn common_transaction_extension_return(props: &Props) -> Html {
    let hookdata = props.hookdata.clone();

    let value = html! {
        <>
        if hookdata.error.is_some() {
            {hookdata.error.unwrap()}
        }
        <br/>
        if hookdata.extrinsic_success.is_some() {
            {hookdata.extrinsic_success.unwrap()}
        }
        <br/>
        if hookdata.extrinsic_error.is_some() {
            {hookdata.extrinsic_error.unwrap()}
        }

        </>
    };
    html! {
        <>
        <Nav />
            <div class="container">
                <h1>{"Transaction details"}</h1>
                <p>
                {value}
                </p>
            </div>
        </>
    }
}
