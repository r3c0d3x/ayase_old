extern crate serde_json;
extern crate shamrock;

use serde_json::Value;
use shamrock::Format;

/// Newtype allowing us to any JSON value.
pub struct Json(Value);

impl Format for Json {
    type Value = Value;

    fn as_value(self) -> Self::Value {
        self.0
    }
}