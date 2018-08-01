use serde::de::DeserializeOwned;
use std::error::Error;
use shamrock::{Layout, Source};
use super::Manager;

use placeholders::PlaceholderFormat;

impl<C: DeserializeOwned, E: Error, L: Layout> Manager<Box<Source<Config = C, Error = E, Layout = L>>> {
    pub fn insert<S: 'static + Source<Config = C, Error = E, Layout = L>>(&mut self, source: S) {
        self.0.insert(S::id(), Box::new(source));
    }
}

pub type SourceManager<C: DeserializeOwned, E: Error, L: Layout> = Manager<Box<Source<Config = C, Error = E, Layout = L>>>;