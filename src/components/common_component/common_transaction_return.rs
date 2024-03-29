use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::components::navigation::nav::Nav;
use crate::components::common_component::common_transaction_return_hook::use_common_transaction_return;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hookdata: TransactionReturn,
}

#[function_component(CommonTransactionReturn)]
pub fn common_transaction_return(props: &Props) -> Html {
    let hookdata = props.hookdata.clone();

    let value = use_common_transaction_return(hookdata);
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
