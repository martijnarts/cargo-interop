use std::{fs, io};

use serde::Deserialize;
use thiserror::Error;

use crate::languages::java::JavaConfig;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not read from file `{filename}`")]
    ReadError {
        filename: String,
        #[source]
        source: io::Error,
    },

    #[error("could not parse TOML")]
    TomlParseError {
        #[source]
        source: toml::de::Error,
    },
}

#[derive(Debug, Deserialize)]
pub struct InteropConfig {
    java: JavaConfig,
}

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    interop: InteropConfig,
}

pub fn parse_config(filename: String) -> Result<InteropConfig, ConfigError> {
    let config = fs::read_to_string(&filename).map_err(|e| ConfigError::ReadError {
        filename,
        source: e,
    })?;

    Ok(toml::from_str::<CargoToml>(&config)
        .map_err(|e| ConfigError::TomlParseError { source: e })?
        .interop)
}
