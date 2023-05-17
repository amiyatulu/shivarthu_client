use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhraseStore;

pub enum TransactionReturnKind {
    Error,
    Finalized,
    InBlock,
}

pub struct TransactionReturn {
    pub kind: TransactionReturnKind,
    pub value: String,
}

#[hook]
pub fn use_balance_tranfer() -> TransactionReturn {
    let (store, _) = use_store::<PhraseStore>();

    let store_clone = store.clone();

    let phrase_option = &store_clone.mnemonic_phrase;

    if let Some(seed) = phrase_option {
        TransactionReturn {
            kind: TransactionReturnKind::Error,
            value: format!("{}", seed),
        }
    } else {
        TransactionReturn {
            kind: TransactionReturnKind::Error,
            value: "Error seed not present".to_owned(),
        }
    }
}
