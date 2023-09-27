use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().expect("Failed to get env"));

#[derive(Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
}
