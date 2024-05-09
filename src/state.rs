use std::sync::Arc;

use tokio::sync::RwLock;

use crate::config::Config;

#[derive(Clone)]
pub struct State {
    // wrap apps in a RwLock to be able to change them at runtime
    pub apps: Arc<RwLock<Vec<(String /* app name */, String /* app path */)>>>,
}

impl From<Config> for State {
    fn from(config: Config) -> Self {
        let apps = config
            .apps
            .into_iter()
            .map(|(app_name, app_config)| (app_name, app_config.path))
            .collect();
        Self {
            apps: Arc::new(RwLock::new(apps)),
        }
    }
}
