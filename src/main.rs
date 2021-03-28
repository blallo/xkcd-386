use clap::{App, Arg};
use std::convert::TryFrom;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use xkcd_386::config::Config;
use xkcd_386::project::Project;
use xkcd_386::source::Source;

fn main() {
    env_logger::Builder::new()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    let cli = App::new("xkcd-386")
        .version("0.1.0")
        .author("blallo@autistici.org")
        .about("Someone is wrong on the internet")
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("GITHUB_TOKEN")
                .help(
                    r#"The token from github to authenticate to crates.io
(may also be passed in CARGO_REGISTRY_TOKEN env variable)"#,
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("PATH_TO_CONFIG")
                .help("Path to the config (defaults to $PWD/.xkcd_386.toml)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("workdir")
                .short("w")
                .long("workdir")
                .value_name("PATH_TO_WORKDIR")
                .help("Path where to create the packages to be published")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outdir")
                .short("o")
                .long("outdir")
                .value_name("PATH_TO_OUTDIR")
                .help(
                    r#"Path where to output the tar.gz of the published packages
(defaults to $PWD)"#,
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .value_name("HOW_MANY")
                .help("How many projects to publish (omitting it means no limit)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("SOURCE")
                .help(
                    r#"How to create the project names. Might be:
    - <file>: a path to a file form which to read line by line
    - "random": generate a random string"#,
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mangle")
                .short("m")
                .long("mangle")
                .help(
                    "When reading from file, generate names as variations of those read from file",
                )
                .takes_value(false),
        )
        .arg(
            Arg::with_name("dry_run")
                .short("r")
                .long("dry-run")
                .help("Run all the needed steps but do not really submit to the registry")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("readme")
                .short("R")
                .long("readme")
                .value_name("PATH_TO_README")
                .help("Use this file as readme instead of the default")
                .takes_value(true),
        )
        .get_matches();

    let token: String = cli
        .value_of("token")
        .map(|token| token.to_string())
        .unwrap_or_else(|| env::var("CARGO_REGISTRY_TOKEN").expect("You must provide a token"));
    let config: Config = Config::from_file(
        cli.value_of("config")
            .map(|c| c.to_string())
            .unwrap_or_else(|| {
                env::current_dir()
                    .expect("Could not find current directory")
                    .join(".xkcd_386.toml")
                    .to_str()
                    .expect("Could not find config")
                    .to_string()
            })
            .as_str(),
    )
    .expect("Could not find config");
    let workdir: PathBuf = PathBuf::from_str(cli.value_of("workdir").unwrap_or("/tmp"))
        .expect("Could not assign workdir");
    let outdir: PathBuf = PathBuf::from_str(
        cli.value_of("outdir")
            .map(|o| o.to_string())
            .unwrap_or_else(|| {
                env::current_dir()
                    .expect("Could not find current directory")
                    .to_str()
                    .expect("Could not find current directory")
                    .to_string()
            })
            .as_str(),
    )
    .expect("Could not assign outdir");
    let limit: Option<u32> = cli
        .value_of("limit")
        .map(|l| l.parse::<u32>().expect("Unexpected value for limit"));
    let readme: Option<PathBuf> = cli
        .value_of("readme")
        .map(|r| PathBuf::from_str(r).expect("Could not assign readme"));
    let dry_run: bool = cli.is_present("dry_run");
    let mangle: bool = cli.is_present("mangle");
    let mut source = Source::try_from(cli.value_of("source").unwrap_or("random"))
        .unwrap()
        .with_limit(limit);

    if mangle {
        source = source.with_mangling();
    }

    log::debug!(
        r#"cli:
    - dry-run: {:?}
    - mangle: {:?}
    - config: {:?}
    - limit: {:?}
    - outdir: {:?}
    - readme: {:?}
    - source: {:?}
    - token: {:?}
    - workdir: {:?}"#,
        dry_run,
        mangle,
        config,
        limit,
        outdir,
        readme,
        source,
        token,
        workdir,
    );

    let _ = source
        .into_iter()
        .map(|r| {
            Project::new(r.clone(), config.clone(), token.clone(), workdir.clone())
                .with_outdir(outdir.clone())
                .with_readme(readme.clone())
                .publish(dry_run)
                .map(|()| log::info!("{} published", r))
                .unwrap_or_else(|e| log::error!("For {}: {:?}", r, e));
        })
        .collect::<()>();
}
