use lazy_static::lazy_static;
use config::{Environment, Config, File, ConfigError};
use serde::Deserialize;
use std::borrow::Borrow;

#[derive(Debug, Deserialize)]
pub struct StatServiceSettings {
    pub max_users_per_ip: u32,
    pub check_in_interval: u64
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub stat: StatServiceSettings
}

lazy_static! {
static ref SETTINGS_INSTANCE : Settings = Settings::new().unwrap();
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Settings"))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }

    pub fn get() -> &'static Settings {
        return SETTINGS_INSTANCE.borrow();
    }
}