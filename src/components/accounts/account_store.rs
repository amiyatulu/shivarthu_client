use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct AccountStore {
    pub hash: Option<String>,
}