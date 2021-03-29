use flate2::read::GzDecoder;
use tar::Archive;
use tempfile::tempdir;

use crate::cargo_toml::CargoToml;
use crate::config::Config;
use crate::project::Project;

#[test]
fn create_project_tarball() {
    let name = "test-rs".to_string();
    let token = "12345".to_string();
    let workdir = tempdir().unwrap();
    let outdir = tempdir().unwrap();
    let config = Config {
        name: name.clone(),
        author: "Jane Doe".to_owned(),
        ..Default::default()
    };
    let proj = Project::new(name.clone(), config.clone(), token, workdir.into_path())
        .with_outdir(outdir.into_path());
    let cargo_toml = proj.get_cargo_toml();
    let expected_cargo_toml = CargoToml::from_config(name, config);
    assert_eq!(expected_cargo_toml, *cargo_toml);

    let tarball = proj.create_tarball().unwrap();

    // Read the tarball
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);
    let mut entries: Vec<String> = archive
        .entries()
        .unwrap()
        // This is vomit-inducing
        .map(|e| e.unwrap().path().unwrap().to_str().unwrap().to_string())
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        // Listing all the entries, including directories
        vec![
            "test-rs-0.1.0/",
            "test-rs-0.1.0/Cargo.toml",
            "test-rs-0.1.0/README.md",
            "test-rs-0.1.0/src",
            "test-rs-0.1.0/src/lib.rs"
        ]
    )
}
