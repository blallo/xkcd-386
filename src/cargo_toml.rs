use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::internal::InternalResult;

pub const VERSION: &str = "0.1.0";
pub const LICENSE: &str = "WTFPL";
pub const DESCRIPTION: &str = "Someone is wrong on the internet";
pub const HOMEPAGE: &str = "https://xkcd.com/386";
pub const DOCUMENTATION: &str = "https://crates.io/policies";
pub const REPOSITORY: &str = "https://github.com/blallo/xkcd-386";
pub const README: &str = "README.md";

#[derive(Serialize, Debug)]
pub struct CargoToml {
    pub project: Project,
}

#[derive(Serialize, Debug)]
pub struct Project {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub license: String,
    pub description: String,
    pub homepage: String,
    pub documentation: String,
    pub repository: String,
    pub readme: String,
}

impl CargoToml {
    pub fn to_file(&self, path: &str) -> InternalResult<()> {
        let mut file = File::open(path)?;
        self.to_open_file(&mut file)
    }

    pub fn to_open_file(&self, file: &mut File) -> InternalResult<()> {
        let string_value: String = Self::to_string(&self)?;
        file.write_all(string_value.as_ref())?;
        Ok(())
    }

    pub fn to_string(&self) -> InternalResult<String> {
        toml::ser::to_string(&self).map_err(|e| e.into())
    }

    pub fn from_config(name: String, config: Config) -> Self {
        Self {
            project: Project {
                name,
                authors: vec![config.author],
                version: config.version.unwrap_or(VERSION.into()),
                license: config.license.unwrap_or(LICENSE.into()),
                description: config.description.unwrap_or(DESCRIPTION.into()),
                homepage: config.homepage.unwrap_or(HOMEPAGE.into()),
                documentation: config.documentation.unwrap_or(DOCUMENTATION.into()),
                repository: config.repository.unwrap_or(REPOSITORY.into()),
                readme: config.readme.unwrap_or(README.into()),
            },
        }
    }
}
