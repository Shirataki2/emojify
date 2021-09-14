use poise::serenity_prelude as serenity;
use std::{collections::HashSet, fs, io, path::Path, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub application: ApplicationConfig,
    pub api: ApiConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    pub application_id: serenity::ApplicationId,
    pub prefix: String,
    pub token: String,
    pub owners: HashSet<serenity::UserId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfig {
    pub url: String,
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Arc<Config>, io::Error> {
    let conf = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&conf)?;
    Ok(Arc::new(config))
}
