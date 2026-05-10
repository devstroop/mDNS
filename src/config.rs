use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub mdns: MdnsConfig,
    #[serde(default)]
    pub zones: Vec<Zone>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    53
}

fn default_api_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MdnsConfig {
    #[serde(default = "default_mdns_enabled")]
    pub enabled: bool,
    #[serde(default = "default_mdns_ttl")]
    pub ttl: u32,
}

fn default_mdns_enabled() -> bool {
    true
}

fn default_mdns_ttl() -> u32 {
    300
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Zone {
    pub name: String,
    #[serde(default = "default_zone_type")]
    pub zone_type: String,
    #[serde(default)]
    pub records: Vec<Record>,
}

fn default_zone_type() -> String {
    "primary".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    pub name: String,
    #[serde(default = "default_record_type")]
    pub record_type: String,
    pub value: String,
    #[serde(default = "default_ttl")]
    pub ttl: u32,
}

fn default_record_type() -> String {
    "A".to_string()
}

fn default_ttl() -> u32 {
    300
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
