use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

use crate::cargo_toml::{self, CargoToml};
use crate::config::Config;
#[test]
fn hydrate_cargo_toml_from_config() {
    let name = "xckd-386".to_string();
    let author = "Jane Doe".to_string();
    let version = "1.3.1-2".to_string();
    let license = "GPLv3".to_string();
    let description = "Trololol".to_string();
    let homepage = "https://not.here.org".to_string();
    let documentation = "https://www.google.com".to_string();
    let repository = "git://git.sr.ht/randomDude/this.git".to_string();
    let readme = "HOW_COMES.md".to_string();
    let config = Config {
        name: name.clone(),
        author: author.clone(),
        version: Some(version.clone()),
        license: Some(license.clone()),
        description: Some(description.clone()),
        homepage: Some(homepage.clone()),
        documentation: Some(documentation.clone()),
        repository: Some(repository.clone()),
        readme: Some(readme.clone()),
    };
    let cargo_toml: CargoToml = config.into();
    assert_eq!(cargo_toml.name, name);
    assert_eq!(cargo_toml.authors, vec![author]);
    assert_eq!(cargo_toml.version, version);
    assert_eq!(cargo_toml.license, license);
    assert_eq!(cargo_toml.description, description);
    assert_eq!(cargo_toml.homepage, homepage);
    assert_eq!(cargo_toml.documentation, documentation);
    assert_eq!(cargo_toml.repository, repository);
    assert_eq!(cargo_toml.readme, readme);
}

#[test]
fn hydrate_cargo_toml_from_config_with_defaults() {
    let name = "xckd-386".to_string();
    let author = "Jane Doe".to_string();
    let config = Config {
        name: name.clone(),
        author: author.clone(),
        version: None,
        license: None,
        description: None,
        homepage: None,
        documentation: None,
        repository: None,
        readme: None,
    };
    let cargo_toml: CargoToml = config.into();
    assert_eq!(cargo_toml.name, name);
    assert_eq!(cargo_toml.authors, vec![author]);
    assert_eq!(cargo_toml.version, cargo_toml::VERSION.to_string());
    assert_eq!(cargo_toml.license, cargo_toml::LICENSE.to_string());
    assert_eq!(cargo_toml.description, cargo_toml::DESCRIPTION.to_string());
    assert_eq!(cargo_toml.homepage, cargo_toml::HOMEPAGE.to_string());
    assert_eq!(
        cargo_toml.documentation,
        cargo_toml::DOCUMENTATION.to_string()
    );
    assert_eq!(cargo_toml.repository, cargo_toml::REPOSITORY.to_string());
    assert_eq!(cargo_toml.readme, cargo_toml::README.to_string());
}

#[test]
fn serialize_to_file() {
    let expected = r#"name = "xkcd-386"
authors = ["Jane Doe"]
version = "0.1.0"
license = "WTFPL"
description = "Someone is wrong on the internet"
homepage = "https://xkcd.com/386"
documentation = "https://crates.io/policies"
repository = "https://github.com/blallo/xkcd-386"
readme = "README.md"
"#;
    let cargo_toml: CargoToml = Config {
        name: "xkcd-386".to_owned(),
        author: "Jane Doe".to_owned(),
        version: None,
        license: None,
        description: None,
        homepage: None,
        documentation: None,
        repository: None,
        readme: None,
    }
    .into();
    let dir = tempdir().unwrap();
    let filepath = dir.path().join("Cargo.toml");
    let mut file = File::create(filepath.clone()).unwrap();
    cargo_toml.to_open_file(&mut file).unwrap();
    drop(file);
    let mut opened_file = File::open(filepath.to_str().unwrap()).unwrap();
    let mut result = String::new();
    opened_file.read_to_string(&mut result).unwrap();
    assert_eq!(expected.to_owned(), result);
}
