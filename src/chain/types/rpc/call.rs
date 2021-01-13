use super::super::{Address, Transaction};
use crate::prelude::*;

/// Call request
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallRequest {
    pub from:        Address,
    pub transaction: Transaction,
}
