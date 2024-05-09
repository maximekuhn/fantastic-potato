use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use self::{app::AppConfig, server::ServerConfig};

pub mod app;
pub mod server;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub apps: HashMap<String /* app name */, AppConfig>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: '{err}'")]
    Io {
        #[from]
        err: std::io::Error,
    },

    #[error("Deserialization error: '{err}'")]
    Deserialization {
        #[from]
        err: serde_yaml::Error,
    },

    #[error("Config is not valid: '{err_msg}'")]
    ValidationError { err_msg: String },
}

impl Config {
    pub fn new_from_file(path: &str) -> Result<Self, ConfigError> {
        let file_content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&file_content)?;
        let validated_config = config
            .validate()
            .map_err(|err| ConfigError::ValidationError {
                err_msg: err.to_string(),
            })?;
        Ok(validated_config)
    }

    fn validate(self) -> Result<Self, &'static str> {
        if !(self.server.listen_addr.is_private() || self.server.listen_addr.is_loopback()) {
            return Err("server.listen_addr must be loopback or private");
        }
        Ok(self)
    }
}
