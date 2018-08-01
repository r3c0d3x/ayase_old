#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate toml;

use std::io::stdin;

pub mod asagi;
pub mod ayase;
mod error;

#[cfg(test)] mod test;

fn main() {
    let settings: asagi::SettingsContainer = serde_json::from_reader(stdin()).unwrap();
    println!("{:?}", settings);
}