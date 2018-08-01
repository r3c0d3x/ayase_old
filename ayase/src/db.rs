use mysql::OptsBuilder as MysqlBuilder;
use postgres::params::{Builder as PostgresBuilder, Host as PostgresHost};
use r2d2::{Config as PoolConfigBuilder, Pool};
use r2d2_mysql::{CreateManager, MysqlConnectionManager};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use config::{Config, DbEngine, Layout};
use error::Result;

#[derive(Debug)]
pub struct Db {
    pub pool: DbPool,
    pub layout: Layout,
}

//impl Db {
//    pub fn get_post() -> 
//}

#[derive(Debug)]
pub enum DbPool {
    MySql(Pool<MysqlConnectionManager>),
    Postgresql(Pool<PostgresConnectionManager>),
}

pub fn connect(config: &Config) -> Result<Db> {
    Ok(Db {
        pool: match config.database.engine {
            DbEngine::Mysql => DbPool::MySql(Pool::new({
                let mut builder = PoolConfigBuilder::builder();

                if config.database.size.is_some() {
                    builder = builder.pool_size(config.database.size.unwrap());
                }

                builder.build()
            }, MysqlConnectionManager::new({
                let mut builder = MysqlBuilder::new();

                builder.ip_or_hostname(config.database.host.clone())
                    .user(Some(config.database.username.clone()))
                    .pass(config.database.password.clone())
                    .db_name(Some(config.database.database.clone()))
                    .prefer_socket(false);

                if config.database.port.is_some() {
                    builder.tcp_port(config.database.port.unwrap());
                }

                builder
            })?)?),
            DbEngine::Postgresql => DbPool::Postgresql(Pool::new({
                let mut builder = PoolConfigBuilder::builder();

                if config.database.size.is_some() {
                    builder = builder.pool_size(config.database.size.unwrap());
                }

                builder.build()
            }, PostgresConnectionManager::new({
                let mut builder = PostgresBuilder::new();
                
                builder.user(config.database.username.as_ref(), config.database.password.as_ref().map(|v| v.as_ref() as &str))
                    .database(config.database.database.as_ref());

                if config.database.port.is_some() {
                    builder.port(config.database.port.unwrap());
                }

                builder.build(PostgresHost::Tcp((match config.database.host {
                    Some(ref host) => host,
                    _ => "127.0.0.1",
                }).to_owned()))
            }, TlsMode::None)?)?),
        },
        layout: config.database.layout.clone(),
    })
}