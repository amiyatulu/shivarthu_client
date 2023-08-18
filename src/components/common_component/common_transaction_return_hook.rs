use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use yew::prelude::*;

#[hook]
pub fn use_common_transaction_return(hookdata: TransactionReturn) -> Html {

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
    value
}
