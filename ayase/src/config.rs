use error::Result;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use toml;

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Boards {
    Bool(bool),
    Some(Vec<String>),
}

impl Default for Boards {
    fn default() -> Self {
        Boards::Bool(false)
    }
}

#[derive(Debug)]
pub struct Fourchan;
#[derive(Debug)]
pub struct Eightchan;

#[derive(Debug, Deserialize)]
pub enum Site {
    #[serde(rename = "4chan")]
    Fourchan,
    Eightchan,
}

#[serde(rename_all = "lowercase")]
#[derive(Clone, Debug, Deserialize)]
pub enum Layout {
    Fuuka,
    FoolFuuka,
    Ayase,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Deserialize)]
pub enum DbEngine {
    Mysql,
    Postgresql,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub engine: DbEngine,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: String,
    pub password: Option<String>,
    pub database: String,
    pub size: Option<u32>,
    pub layout: Layout,
}

#[derive(Debug, Deserialize)]
pub struct Engine {
    pub site: Site,
    #[serde(default)]
    pub boards: Boards,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Db,
    pub engine: Engine,
}

impl Config {
    pub fn new() -> Result<Config> {
        let mut contents = String::new();
        File::open("ayase.toml")?.read_to_string(&mut contents)?;
        Ok(toml::from_str(contents.as_ref())?)
    }
}