use shamrock::Format;
use postgres::Error;
use postgres::types::ToSql;

/// Newtype allowing us to represent any storable PostgreSQL value.
pub struct PostgreSqlFormat(Vec<Box<ToSql>>);

impl Format for PostgreSqlFormat {
    type Value = Vec<Box<ToSql>>;

    fn as_value(self) -> Self::Value {
        self.0
    }
}