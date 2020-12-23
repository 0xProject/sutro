use crate::prelude::*;
use serde_hex::{SerHex, StrictPfx};

/// Ethereum addresses with Serialization to 0x prefixed hex string.
///
/// # To do
///
/// * Add check sum
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct Address(#[serde(with = "SerHex::<StrictPfx>")] [u8; 20]);

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::{assert_eq, *};
    use serde_json::{json, to_value};

    #[test]
    fn test_serialize_default() {
        assert_eq!(
            to_value(&Address::default()).unwrap(),
            json!("0x0000000000000000000000000000000000000000")
        );
    }
}
