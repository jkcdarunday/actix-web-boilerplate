use std::sync::RwLock;
use config::Config;

lazy_static! {
	pub static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(config::File::with_name("config.toml")).unwrap();
        settings.merge(config::Environment::with_prefix("APP")).unwrap();

        settings
    });
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T{
    CONFIG.read().unwrap().get::<T>(key).unwrap()
}
