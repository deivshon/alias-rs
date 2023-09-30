use core::fmt;
use std::{convert, fs, io, path::Path};

use serde::Deserialize;

use crate::shell::Shell;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub aliases: Vec<Alias>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Alias {
    pub alias: String,
    pub equals: String,
    pub shells: Option<ShellList>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum ShellList {
    Blacklist(Vec<Shell>),
    Whitelist(Vec<Shell>),
}
pub enum ConfigError {
    Io(io::Error),
    Deserialization(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error: {}", err),
            ConfigError::Deserialization(err) => {
                write!(f, "Error in JSON config: {}", err)
            }
        }
    }
}

impl convert::From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        return ConfigError::Io(err);
    }
}

impl convert::From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> ConfigError {
        return ConfigError::Deserialization(err);
    }
}

impl Config {
    // TODO enforce no spaces in alias
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let path = Path::new(path);
        let file_content = fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&file_content)?;

        return Ok(config);
    }
}
