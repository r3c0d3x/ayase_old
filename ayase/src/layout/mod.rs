use mysql::{Params, Stmt};
use mysql::value::FromValue;
use postgres::types::ToSql;

use shared::{Board, Thread};

pub mod asagi;

use db::Engine;

pub trait Layout<E: Engine> {
    fn insert_post<T: Thread>()
    fn insert_thread()
}