extern crate serde;
extern crate shamrock;
extern crate void;

use shamrock::placeholders::{PlaceholderFormat, PlaceholderLayout};
use shamrock::traits::*;

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::iter::IntoIterator;

pub mod managers;
pub mod placeholders;

pub struct ConstructManager(HashMap<&'static str, Box<Construct<PlaceholderLayout>>>);
pub struct ConstructIter<'a>(Iter<'a, &'static str, Box<Construct<PlaceholderLayout>>>);

impl ConstructManager {
    pub fn new() -> ConstructManager {
        ConstructManager(HashMap::new())
    }

    pub fn insert<C: 'static + Construct<PlaceholderLayout>>(&mut self) {
        self.0.insert(C::id(), Box::new(C::new(PlaceholderFormat)));
    }

    pub fn iter(&self) -> ConstructIter {
        ConstructIter(self.0.iter())
    }
}

impl<'a> Iterator for ConstructIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&r, _)| r)
    }
}

//impl Iterator for ConstructIter {
//    type Item = &'static str;
//
//    
//}
//
//impl IntoIterator for ConstructManager {
//    type Item = &'static str;
//    type IntoIter = ConstructIter;
//
//    fn into_iter(self) -> Self::IntoIter {
//        ConstructIter(self.0.into_iter())
//    }
//}