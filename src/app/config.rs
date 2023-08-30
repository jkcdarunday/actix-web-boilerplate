use config::Config;
use dotenv::dotenv;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();

    Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
});

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIG.get::<T>(key).unwrap()
}
