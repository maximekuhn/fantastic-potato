use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use crate::{
    config::{app::LoadBalancerType, Config},
    load_balancer::{random::RandomLoadBalancer, LoadBalancer},
};

#[derive(Clone)]
pub struct State {
    // wrap apps in a RwLock to be able to change them at runtime
    pub apps: Arc<RwLock<Vec<(String /* app name */, String /* app path */)>>>,

    pub load_balancers: Arc<HashMap<String /* app name */, Mutex<Box<dyn LoadBalancer>>>>,
}

impl From<Config> for State {
    fn from(config: Config) -> Self {
        let apps = config
            .apps
            .iter()
            .map(|(app_name, app_config)| (app_name.to_string(), app_config.path.to_string()))
            .collect();

        let load_balancers = config
            .apps
            .into_iter()
            .map(|(app_name, app_config)| {
                let backends = app_config.backends;
                let lb: Box<dyn LoadBalancer> = Box::new(match app_config.load_balancer {
                    LoadBalancerType::Random => RandomLoadBalancer::new(backends),
                });

                let lb = Mutex::new(lb);
                (app_name.to_string(), lb)
            })
            .collect();

        Self {
            apps: Arc::new(RwLock::new(apps)),
            load_balancers: Arc::new(load_balancers),
        }
    }
}
