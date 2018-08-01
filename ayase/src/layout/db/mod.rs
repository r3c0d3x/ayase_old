use ib::Layout;
use mysql::prelude::ToValue as ToMySql;
use postgres::types::ToSql as ToPostgreSql;

/// Database-safe Format
pub trait DbFormat: Format + ToMySql + ToPostgreSql { }

