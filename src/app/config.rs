use std::sync::RwLock;
use config::{Config, File, FileFormat, Environment};

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new({
        let builder = Config::builder()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .add_source(Environment::with_prefix("APP"));

        match builder.build() {
            Ok(config) => config,
            Err(e) => {
                panic!("Config Error: {}", e);
            }
        }
    });
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T{
    CONFIG.read().unwrap().get::<T>(key).unwrap()
}
