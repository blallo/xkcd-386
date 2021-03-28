use std::fs;
use std::str::FromStr;

use serde::Deserialize;

use crate::internal::{InternalError, InternalResult};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub version: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> InternalResult<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_str(content.as_ref())
    }
}

impl FromStr for Config {
    type Err = InternalError;

    fn from_str(data: &str) -> InternalResult<Self> {
        toml::from_str(data).map_err(|e| e.into())
    }
}
