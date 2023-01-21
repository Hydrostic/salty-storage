use serde::Deserialize;

use config::{Config, File};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: Server,
}
impl AppConfig {
    pub fn init(path: String) -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .set_default("server.address", "127.0.0.1")?
            .set_default("server.port", "1146")?;

        let config = config.add_source(File::with_name(&path)).build().unwrap();

        config.try_deserialize()
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}
