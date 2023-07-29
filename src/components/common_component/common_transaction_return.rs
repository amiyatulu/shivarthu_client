use yew::prelude::*;
use crate::components::accounts::hooks::commons::{TransactionReturnKind, TransactionReturn};
use crate::components::navigation::nav::Nav;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub hookdata: TransactionReturn
}

#[function_component(CommonTransactionReturn)]
pub fn common_transaction_return(props: &Props) -> Html {
    let hookdata = props.hookdata.clone();
    html! {
        <>
        <Nav />
            <div class="container">
                <h1>{"Transaction details"}</h1>
                <p>
                {
                    match hookdata.kind {
                        TransactionReturnKind::Finalized => {hookdata.value}
                        TransactionReturnKind::Error => {hookdata.value}
                        TransactionReturnKind::InBlock => {hookdata.value}
                        TransactionReturnKind::Processing => {hookdata.value}
                    }
                }
                </p>
            </div>
        </>

    }
}

