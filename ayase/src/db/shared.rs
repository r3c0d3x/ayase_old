use std::marker::PhantomData;

use error;

pub trait SqlType: ToSql + ToValue { }

pub trait Engine {
    fn new(&Config) -> error::Result<Self>;
    fn insert(&self, &str, &[&SqlType]) -> error::Result<()>;
}

//https://player.twitch.tv/?volume=0.11&!muted&video=v161911951&collection=&
//2:45:45