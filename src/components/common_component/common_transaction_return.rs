use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::components::navigation::nav::Nav;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hookdata: TransactionReturn,
}

#[function_component(CommonTransactionReturn)]
pub fn common_transaction_return(props: &Props) -> Html {
    let hookdata = props.hookdata.clone();

    let value = match hookdata.kind {
        TransactionReturnKind::Finalized => {
            if let Some(error) = hookdata.dispatch_error {
                html! {
                    <>
                    {error} <br/> {hookdata.value}
                    </>
                }
            } else {
                html! {
                    <>
                    {hookdata.value}
                    </>
                }
            }
        }
        TransactionReturnKind::Error => html! {<> {hookdata.value} </>},
        TransactionReturnKind::InBlock => html! {<> {hookdata.value} </>},
        TransactionReturnKind::Processing => html! {<> {hookdata.value} </>},
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
