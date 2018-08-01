use placeholders::{PlaceholderFormat, PlaceholderLayout};
use shamrock::Construct;
use super::Manager;

impl Manager<Box<Construct<PlaceholderLayout>>> {
    pub fn insert<C: 'static + Construct<PlaceholderLayout>>(&mut self) {
        self.0.insert(C::id(), Box::new(C::new(PlaceholderFormat)));
    }
}

pub type ConstructManager = Manager<Box<Construct<PlaceholderLayout>>>;