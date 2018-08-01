use serde_json::Value;
use std::fmt;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Components {
    pub layout: String,
    pub source: String,
    pub store: String,
}

#[derive(Debug)]
pub struct Config(Components, HashMap<String, HashMap<String, Value>>);

struct ConfigVisitor(PhantomData<HashMap<String, Value>>);

