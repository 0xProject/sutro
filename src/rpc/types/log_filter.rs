use super::{Address, BlockNumber, ValueOrArray};
use crate::prelude::*;
use serde_with::skip_serializing_none;

/// Log Filter
#[skip_serializing_none]
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFilter {
    pub from_block: Option<BlockNumber>,
    pub to_block:   Option<BlockNumber>,
    pub block_hash: Option<U256>,
    pub address:    Option<ValueOrArray<Address>>,
    pub topics:     Option<Vec<ValueOrArray<U256>>>,
    pub limit:      Option<usize>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_value, json};

    #[test]
    fn test_decode() {
        let _de: LogFilter = from_value(json!({
          "address": null,
          "blockHash": "0x6869791f0a34781b29882982cc39e882768cf2c96995c2a110c577c53bc932d5",
          "topics": [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            [
              "0x17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31",
              "0xe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c",
              "0x7fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65",
              "0x02c310a9a43963ff31a754a4099cc435ed498049687539d72d7818d9b093415c",
              "0x82af639571738f4ebd4268fb0363d8957ebe1bbb9e78dba5ebd69eed39b154f0"
            ],
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"
          ]
        }))
        .unwrap();
    }
}
