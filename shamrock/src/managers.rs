use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::IntoIterator;

use placeholders::*;
use traits::*;

pub struct Manager<T>(HashMap<&'static str, T>);
pub struct Iter<'a, T: 'a>(HashMapIter<'a, &'static str, T>);

//pub struct ConstructManager(HashMap<&'static str, Box<Construct<PlaceholderLayout>>>);
//pub struct ConstructIter<'a>(Iter<'a, &'static str, Box<Construct<PlaceholderLayout>>>);
//
//impl ConstructManager {
//    pub fn new() -> ConstructManager {
//        ConstructManager(HashMap::new())
//    }
//
//    pub fn insert<C: 'static + Construct<PlaceholderLayout>>(&mut self) {
//        self.0.insert(C::id(), Box::new(C::new(PlaceholderFormat)));
//    }
//
//    pub fn iter(&self) -> ConstructIter {
//        ConstructIter(self.0.iter())
//    }
//}
//
//impl<'a> Iterator for ConstructIter<'a> {
//    type Item = &'static str;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        self.0.next().map(|(&r, _)| r)
//    }
//}

impl<T> Manager<T> {
    pub fn new() -> Manager<T> {
        Manager(HashMap::new())
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.0.iter())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&r, _)| r)
    }
}

impl Manager<Box<Construct<PlaceholderLayout>>> {
    pub fn insert<C: 'static + Construct<PlaceholderLayout>>(&mut self) {
        self.0.insert(C::id(), Box::new(C::new(PlaceholderFormat)));
    }
}

pub type ConstructManager = Manager<Box<Construct<PlaceholderLayout>>>;