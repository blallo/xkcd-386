use std::fs;

use serde::Deserialize;

use crate::internal::InternalResult;

#[derive(Deserialize, Debug)]
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

    pub fn from_str(data: &str) -> InternalResult<Self> {
        toml::from_str(data).map_err(|e| e.into())
    }
}