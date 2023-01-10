use yewdux::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub (super) struct AccountStore {
    pub hash: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct AccountPubStore {
    pub account_address: Option<String>,
}



// Does not get stored in local storage
#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub (super) struct PhraseStore {
    pub mnemonic_phrase: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct PhaseExists {
    pub phase_exists_in_state: bool,
}