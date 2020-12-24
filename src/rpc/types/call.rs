use super::{Address, Bytes, Hex};
use crate::prelude::*;
use serde_with::skip_serializing_none;

/// Call request
#[allow(clippy::module_name_repetitions)]
#[skip_serializing_none]
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
    pub from:      Option<Address>,
    pub to:        Option<Address>,
    pub gas:       Option<Hex<U256>>,
    pub gas_price: Option<Hex<U256>>,
    pub value:     Option<Hex<U256>>,
    pub data:      Option<Bytes>,
}
