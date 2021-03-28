#![allow(clippy::writeln_empty_string)]
use curl::easy::Easy;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::BTreeMap;
use std::env;
use std::fs::{copy, create_dir, write, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::cargo_toml::CargoToml;
use crate::config::Config;
use crate::internal::{InternalError, InternalResult};

const DEFAULT_VERSION: &str = "0.1.0";
const DEFAULT_LIB: &str = r#"#[cfg(test)]
mod tests {
    #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
}"#;

pub struct Project {
    name: String,
    version: String,
    token: String,
    cargo_toml: CargoToml,
    workdir: PathBuf,
    readme: Option<PathBuf>,
    outdir: Option<PathBuf>,
}

impl Project {
    pub fn new(name: String, config: Config, token: String, workdir: PathBuf) -> Self {
        log::debug!("Creating project for: {}", name);
        let version = config
            .clone()
            .version
            .unwrap_or_else(|| DEFAULT_VERSION.into());
        let cargo_toml = CargoToml::from_config(name.clone(), config);
        Self {
            name,
            version,
            cargo_toml,
            token,
            workdir,
            readme: None,
            outdir: None,
        }
    }

    pub fn with_outdir(self, outdir: PathBuf) -> Self {
        log::debug!("For {} setting outdir to {:?}", self.name, outdir);
        Self {
            outdir: Some(outdir),
            ..self
        }
    }

    pub fn with_readme(self, readme: Option<PathBuf>) -> Self {
        log::debug!("For {} setting readme to {:?}", self.name, readme);
        Self { readme, ..self }
    }

    #[cfg(test)]
    pub fn get_cargo_toml(&self) -> &CargoToml {
        &self.cargo_toml
    }

    pub fn create_tarball(&self) -> InternalResult<File> {
        // create workdir for this package
        let workdir: PathBuf = self.workdir.join(self.name.clone());
        log::debug!("For {} creating workdir to {:?}", self.name, workdir);
        create_dir(workdir.clone())?;
        // Cargo.toml
        let cargo_toml_path = try_as_string(workdir.join("Cargo.toml"))?;
        log::debug!(
            "For {} creating Cargo.toml into {:?}",
            self.name,
            cargo_toml_path
        );
        self.cargo_toml.to_file(&cargo_toml_path)?;
        // src/lib.rs
        log::debug!(
            "For {} creating src/lib.rs to {:?}",
            self.name,
            workdir.join("src").join("lib.rs")
        );
        create_dir(workdir.join("src"))?;
        let lib_path = try_as_string(workdir.join("src").join("lib.rs"))?;
        let mut lib_file = File::create(lib_path)?;
        lib_file.write_all(DEFAULT_LIB.as_bytes())?;
        drop(lib_file);
        // README.md
        let readme_dest_path = workdir.join(self.cargo_toml.clone().project.readme);
        if let Some(readme_src_path) = self.readme.clone() {
            log::debug!(
                "For {} copying {:?} to {:?}",
                self.name,
                readme_src_path,
                readme_dest_path
            );
            copy(readme_src_path, readme_dest_path)?;
        } else {
            log::debug!(
                "For {} filling {:?} with default readme content",
                self.name,
                readme_dest_path
            );
            write(readme_dest_path, default_readme(&self.name))?;
        }
        // create tarball
        let outdir: PathBuf = self
            .outdir
            .clone()
            .map(Ok)
            .unwrap_or_else(env::current_dir)?;
        let tar_path = outdir.join(self.name.clone()).with_extension("tar.gz");
        log::debug!("For {} creating the tarball in {:?}", self.name, tar_path);
        let tar_gz = File::create(tar_path.clone())?;
        let mut gz = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(&mut gz);
        tar.append_dir_all(self.base_path(), workdir)?;
        // Reopen tarball and return the `File`
        File::open(tar_path).map_err(|e| e.into())
    }

    fn base_path(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    fn get_readme_content(&self) -> String {
        if let Some(readme_path) = self.readme.clone() {
            readme_from_file(readme_path)
        } else {
            default_readme(self.name.as_str())
        }
    }

    pub fn publish(self, dry_run: bool) -> InternalResult<()> {
        let tarball = self.create_tarball()?;
        let krate = crates_io::NewCrate {
            name: self.name.clone(),
            vers: self.version.clone(),
            deps: vec![],
            features: BTreeMap::new(),
            authors: self.cargo_toml.clone().project.authors,
            description: Some(self.cargo_toml.clone().project.description),
            documentation: Some(self.cargo_toml.clone().project.documentation),
            homepage: Some(self.cargo_toml.clone().project.homepage),
            readme: Some(self.get_readme_content()),
            readme_file: Some(self.cargo_toml.clone().project.readme),
            keywords: vec![],
            categories: vec![],
            license: Some(self.cargo_toml.clone().project.license),
            license_file: None,
            repository: Some(self.cargo_toml.clone().project.repository),
            badges: BTreeMap::new(),
            links: None,
            v: None,
        };
        log::debug!("Publishing {}", self.name);
        let mut registry: crates_io::Registry = self.into();
        if dry_run {
            log::debug!("Dry run, bailing out");
            Ok(())
        } else {
            registry
                .publish(&krate, &tarball)
                // Ignore all warnings
                .map(|_| ())
                // Map error to internal error
                .map_err(|e| InternalError::Generic(format!("Failed to publish: {:?}", e)))
        }
    }
}

impl Into<crates_io::Registry> for Project {
    fn into(self) -> crates_io::Registry {
        let mut handle = Easy::new();
        // Explicitly panic if failing
        handle
            .useragent("xkcd-386 (https://github.com/blallo/xkcd-386)")
            .unwrap();
        crates_io::Registry::new_handle("https://crates.io".to_owned(), Some(self.token), handle)
    }
}

fn try_as_string(basepath: PathBuf) -> InternalResult<String> {
    match basepath.to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err(InternalError::Generic(format!(
            "Invalid path for: {:?}",
            basepath
        ))),
    }
}

fn default_readme(name: &str) -> String {
    format!(
        r#"# {}

Automatically generated with [xkcd-386](https://github.com/blallo/xkcd-386)
"#,
        name
    )
}

fn readme_from_file(path: PathBuf) -> String {
    let mut readme_content = String::new();
    let _ = File::open(path)
        .expect("Could not open provided readme")
        .read_to_string(&mut readme_content);
    readme_content
}
