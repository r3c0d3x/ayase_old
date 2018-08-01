use shamrock::Format;
use mysql::{Error, Value};

/// Newtype allowing us to represent any storable MySQL value.
#[derive(Clone, Debug)]
pub struct MySqlFormat(String, Vec<Value>);

impl Format for MySqlFormat {
    type Value = (String, Vec<Value>);

    fn as_value(self) -> Self::Value {
        (self.0, self.1)
    }
}