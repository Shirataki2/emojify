use std::sync::Arc;

use crate::config::Config;

pub struct Data {
    pub config: Arc<Config>,
    pub client: reqwest::Client,
}
