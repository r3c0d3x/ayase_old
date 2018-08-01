use futures::Future;
use futures::future;
use shamrock::constructs::*;
use shamrock::traits::*;
use mysql::{Error as MySqlError, OptsBuilder, Params};
use r2d2::ManageConnection;
use r2d2_mysql::{CreateManager, MysqlConnectionManager};

use config::Config;
use format::MySqlFormat;

pub struct MySql(Config, MysqlConnectionManager);

impl MySql {
    fn send(&self, value: <Self as Store>::Format)
        -> Box<Future<Item = (), Error = <Self as Store>::Error>>
    {
        let (stmt, params) = value.as_value();
        Box::new(future::result(self.1.connect().and_then(|mut conn| {
            conn.prep_exec(stmt, Params::Positional(params));
            Ok(())
        })))
    }
}

impl Identifiable for MySql {
    fn id() -> &'static str {
        "mysql"
    }
}

impl Store for MySql {
    type Config = Config;
    type Error = MySqlError;
    type Format = MySqlFormat;

    fn new(config: Self::Config) -> Box<Future<Item = Self, Error = Self::Error>>
    {
        let mut opts = OptsBuilder::new();

        opts.ip_or_hostname(Some(config.address.clone()))
            .user(Some(config.username.clone()))
            .pass(Some(config.password.clone()))
            .db_name(Some(config.database.clone()))
            .prefer_socket(false);

        if (config.port.is_some()) {
            opts.tcp_port(config.port.clone().unwrap());
        }

        Box::new(future::result(MysqlConnectionManager::new(opts).map(|manager| MySql(config, manager))))
    }

    fn store<L: Storable<Self>, C: Construct<L>>(&self, v: C) -> Box<Future<Item = (), Error = Self::Error>> {
        self.send(L::convert(v.into_format()))
    }
}