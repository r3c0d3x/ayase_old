use serde::de::DeserializeOwned;
use serde_json;
use std::fs::File;
use std::io::{stderr, Write};
use std::path::Path;

use error::Result;

pub fn from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T> {
    Ok(serde_json::from_reader(File::open(path)?)?)
}

pub fn assert<T>(res: Result<T>) {
    if let Err(ref e) = res {
        let stderr = &mut stderr();
        let errmsg = "error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        panic!();
    }
}