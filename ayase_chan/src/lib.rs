extern crate futures;
extern crate serde;
extern crate shamrock;

#[macro_use] extern crate serde_derive;

use futures::Future;
use shamrock::traits::*;
use std::error::Error;

pub mod constructs;

use constructs::*;

pub struct Config;

pub trait Board: Source {
    type Chan: Chan;

//    fn threads(&self) -> Box<Future<Vec<Self::>>>;
}

pub trait Chan {
    type Error: Error;
    type Layout: Layout;

    fn threads(&self) -> Box<Stream<Item = Thread<Self::Layout>, Error = Self::Error>>;
}