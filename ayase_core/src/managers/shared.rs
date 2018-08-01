use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::IntoIterator;

use shamrock::placeholders::*;
use shamrock::traits::*;

pub struct Manager<T>(pub HashMap<&'static str, T>);
pub struct Iter<'a, T: 'a>(HashMapIter<'a, &'static str, T>);

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