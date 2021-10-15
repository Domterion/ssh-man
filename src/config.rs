use serde::Deserialize;

use core::fmt;
use std::{collections::HashMap, fs, net::Ipv4Addr, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: HashMap<String, ServerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: Ipv4Addr,
    pub username: String,
    pub description: Option<String>,
    pub password: Option<String>,
    pub key_file: Option<PathBuf>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, server) in &self.servers {
            write!(f, "[{}] {}", name, server);
        }
        write!(f, "")
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@{}\n\tDescription: {}\n\tPassword: {:?}\n\tKey File: {:?}",
            self.username,
            self.ip,
            self.description
                .to_owned()
                .unwrap_or("No description".to_string()),
            self.password,
            self.key_file
        )
    }
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
