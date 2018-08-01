#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: Option<u16>,
}