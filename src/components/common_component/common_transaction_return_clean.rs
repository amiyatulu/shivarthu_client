use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::components::navigation::nav::Nav;
use crate::components::common_component::common_transaction_return_hook::use_common_transaction_return;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hookdata: TransactionReturn,
}

#[function_component(CommonTransactionReturnClean)]
pub fn common_transaction_return_clean(props: &Props) -> Html {
    let hookdata = props.hookdata.clone();

    let value = use_common_transaction_return(hookdata);
    html! {
        <>
            <h1>{"Transaction details"}</h1>
            <p>
                {value}
            </p>
        </>
    }
}
