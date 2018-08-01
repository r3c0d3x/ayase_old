#![deny(missing_docs)]
//! MySQL `diesel` support is postponed until it
//! [supports unsigned MySQL types](https://github.com/diesel-rs/diesel/pull/912).

//#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate hyper_tls;
extern crate native_tls;
extern crate mysql;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate r2d2_postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate toml;

use futures::{Future, Stream};
use futures::future;
use futures::sync::oneshot;
use futures_cpupool::{Builder as CpuPoolBuilder, CpuPool};
//use futures_state_stream::StateStream;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio_core::reactor::{Core, Interval};
use postgres::Connection;
use postgres::params::{Builder, Host};

use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use r2d2::{Config as DbPoolBuilder, Pool as DbPool};
use r2d2_postgres::{PostgresConnectionManager as ConnectionManager, TlsMode};

pub mod api;
#[allow(missing_docs)]
pub mod config;
#[allow(missing_docs)]
pub mod db;
#[allow(missing_docs)]
pub mod error;

use error::*;

quick_main!(run);

fn run() -> Result<()> {
    let config = config::Config::new()?;
    let connection = db::connect(&config)?;

    println!("{:?}", config);
    println!("connected to db");

    //let wanted_boards = vec!["g".to_owned(), "sci".to_owned()];

    let mut core = Core::new()?;
    let handle = core.handle();

    let client = Client::configure()
        .connector(HttpsConnector::new(1, &handle)?)
        .build(&handle);

    //let manager = ConnectionManager::new(Builder::new()
    //    .user("postgres", Some("fU$n#K_:#}yPiVZRq)Fz"))
    //    .database("ayase")
    //    .build(Host::Tcp("localhost".to_owned())), TlsMode::None)?;
    //
    //let db_pool = DbPool::new(DbPoolBuilder::builder()
    //    .pool_size(4)
    //    .build(), manager)?;

    let channel = oneshot::channel::<()>();

    let fail = Option::Some(channel.0);
    let lifeline = channel.1.map_err(Error::from);

    let _ = core.run(
        api::boards(&client).and_then(|res| {
            let boards_iter = res.boards
                .into_iter();

            let boards: Vec<api::types::Board>;

            match config.engine.boards {
                config::Boards::Bool(b) => if b {
                    boards = boards_iter.collect();
                } else {
                    return Box::new(future::err(Error::from_kind(ErrorKind::Msg("no boards selected".to_owned()))))
                        as Box<Future<Item = (_, _), Error = Error>>;
                },
                config::Boards::Some(ref wanted_boards) => {
                    boards = boards_iter
                        .filter(|ref board| wanted_boards.iter().any(|ref wanted| board.board == **wanted))
                        .collect();
                }
            }

            println!("{:?}", boards);

            let cpu_pool = CpuPoolBuilder::new()
                .name_prefix("ayase-pool-")
                .after_start(|| println!("spawned thread"))
                .before_stop(|| println!("killed thread"))
                .create();

            Box::new(
                lifeline.select(
                    Interval::new(Duration::from_secs(1), &handle)
                        .unwrap()
                        .map_err(Error::from)
                        .for_each(|_| {
                            println!("one second elapsed");
                            //println!("{:?}", &boards);
                            
                            future::ok(())
                        })
                ).map_err(|(err, _)| err)
            ) as Box<Future<Item = (_, _), Error = Error>>
        })
    )?;

    Ok(())
}

/*
fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let test = Connection::connect(
        Builder::new()
            .user("postgres", Some("fU$n#K_:#}yPiVZRq)Fz"))
            .database("ayase")
            .build(Host::Tcp("localhost".to_owned())), 
        TlsMode::None,
        &handle
    )
    .map_err(|err| Box::new(err) as Box<Error>)
    .and_then(|db| {
        
    })
    .and_then(|db| {
        db.prepare("select * from information_schema.tables").map_err(|err| Box::new(err) as Box<Error>)
    })
    .and_then(|(statement, db)| {
        db.query(&statement, &[]).collect().map_err(|err| Box::new(err) as Box<Error>)
    })
    .and_then(|(tables, db)| {
        println!("{:?}", tables.into_iter().map(|row| row.get::<String, &str>("table_name")).collect::<Vec<_>>());
        ok(())
    });

    core.run(test).unwrap();
}
*/