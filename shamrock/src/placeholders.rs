use std::error::Error;
use std::fmt;

use traits::*;

/// XXX: Replace with [this](https://github.com/rust-lang/rust/issues/35121) when it's stable.
#[derive(Debug)]
pub struct PlaceholderError;

impl fmt::Display for PlaceholderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for PlaceholderError {
    fn description(&self) -> &str {
        ""
    }
}

pub struct PlaceholderFormat;

impl Format for PlaceholderFormat {
    type Value = ();

    fn as_value(self) -> Self::Value {
        ()
    }
}

pub struct PlaceholderLayout;

impl Identifiable for PlaceholderLayout {
    fn id() -> &'static str {
        ""
    }
}

impl Layout for PlaceholderLayout {
    type Error = PlaceholderError;
    type Format = PlaceholderFormat;
}