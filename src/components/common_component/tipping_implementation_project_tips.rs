use crate::services::common_services::polkadot;

use polkadot::runtime_types::project_tips::types::TippingName;

impl TippingName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "smalltipper" => Some(TippingName::SmallTipper),
            "bigtipper" => Some(TippingName::BigTipper),
            "smallspender" => Some(TippingName::SmallSpender),
            "mediumspender" => Some(TippingName::MediumSpender),
            "bigspender" => Some(TippingName::BigSpender),
            _ => None,
        }
    }
}