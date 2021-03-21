use std::str::FromStr;

use crate::config::Config;
use crate::internal::{InternalError, InternalResult};

#[test]
fn deserialize_valid_full_config() -> InternalResult<()> {
    let valid_config = Config::from_str(
        r#"
        name = "Jane Doe"
        author = "j.doe@mydom.info"
        version = "1.2.3"
        license = "GPLv3"
        description = "Just an innocent joke"
        homepage = "file:///dev/null"
        documentation = "https://www.google.com"
        repository = "https://github.com/blallo/xkcd-386"
        readme = "DON_T_DO_IT_AT_HOME.md"
    "#,
    )?;

    is_ok(
        "name",
        Some(valid_config.name),
        Some("Jane Doe".to_string()),
    )?;
    is_ok(
        "author",
        Some(valid_config.author),
        Some("j.doe@mydom.info".to_string()),
    )?;
    is_ok("version", valid_config.version, Some("1.2.3".to_string()))?;
    is_ok("license", valid_config.license, Some("GPLv3".to_string()))?;
    is_ok(
        "description",
        valid_config.description,
        Some("Just an innocent joke".to_string()),
    )?;
    is_ok(
        "homepage",
        valid_config.homepage,
        Some("file:///dev/null".to_string()),
    )?;
    is_ok(
        "documentation",
        valid_config.documentation,
        Some("https://www.google.com".to_string()),
    )?;
    is_ok(
        "repository",
        valid_config.repository,
        Some("https://github.com/blallo/xkcd-386".to_string()),
    )?;
    is_ok(
        "readme",
        valid_config.readme,
        Some("DON_T_DO_IT_AT_HOME.md".to_string()),
    )
}

#[test]
fn deserialize_valid_partial_config() -> InternalResult<()> {
    let valid_config = Config::from_str(
        r#"
        name = "Jane Doe"
        author = "j.doe@mydom.info"
    "#,
    )?;

    is_ok(
        "name",
        Some(valid_config.name),
        Some("Jane Doe".to_string()),
    )?;
    is_ok(
        "author",
        Some(valid_config.author),
        Some("j.doe@mydom.info".to_string()),
    )?;
    is_ok("version", valid_config.version, None)?;
    is_ok("license", valid_config.license, None)?;
    is_ok("description", valid_config.description, None)?;
    is_ok("homepage", valid_config.homepage, None)?;
    is_ok("documentation", valid_config.documentation, None)?;
    is_ok("repository", valid_config.repository, None)?;
    is_ok("readme", valid_config.readme, None)
}

#[test]
fn deserialize_invalid_config() -> InternalResult<()> {
    match Config::from_str(
        r#"
        name = "Jane Doe"
    "#,
    ) {
        Err(InternalError::TomlDeError(_)) => Ok(()),
        e => Err(InternalError::Generic(format!("Unexpected: {:?}", e))),
    }
}

fn is_ok(field: &str, value: Option<String>, expected: Option<String>) -> InternalResult<()> {
    if value == expected {
        Ok(())
    } else {
        Err(InternalError::Generic(format!(
            "Unexpected value {:?} in field {} (expected {:?})",
            value, field, expected,
        )))
    }
}
