use futures::Future;
use futures::future;
use shamrock::{Format, Layout, Store, Storable};
use shamrock::constructs::*;
use mysql::{Error as MySqlError, OptsBuilder, Params};
use r2d2::ManageConnection;
use r2d2_mysql::{TlsMode, PostgresConnectionManager};

use config::Config;
use format::PostgreSqlFormat;

pub struct MySql(Config, MysqlConnectionManager);