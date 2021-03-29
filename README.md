# xkcd-386

> No policing is a policy choice

## Why?

Read this: [https://troubles.noblogs.org/post/2021/03/29/why-so-much-ado](https://troubles.noblogs.org/post/2021/03/29/why-so-much-ado)

## How?

#### Compile

```
$ cargo build --release
```

#### Prerequisites

You will need a github account, a valid token for crates.io and the email
address used with the github account needs to be verified on crates.io via a
verification email. You can follow [this guide][gh].

[gh]: https://doc.rust-lang.org/cargo/reference/publishing.html

#### Usage

You need to create a file (defaults to `$PWD/.xkcd_386.toml`) where to put at
least this two pieces of information

```
name = "a random name"
author = "your name <preferably.with.an@email.info>"
```

The cli supports the following parameters:

```
$ xkcd-386 --help
xkcd-386 0.1.0
blallo@autistici.org
Someone is wrong on the internet

USAGE:
    xkcd-386 [FLAGS] [OPTIONS]

FLAGS:
    -r, --dry-run    Run all the needed steps but do not really submit to the registry
    -h, --help       Prints help information
    -m, --mangle     When reading from file, generate names as variations of those read from file
    -V, --version    Prints version information

OPTIONS:
    -c, --config <PATH_TO_CONFIG>      Path to the config (defaults to $PWD/.xkcd_386.toml)
    -l, --limit <HOW_MANY>             How many projects to publish (omitting it means no limit)
    -o, --outdir <PATH_TO_OUTDIR>      Path where to output the tar.gz of the published packages
                                       (defaults to $PWD)
    -R, --readme <PATH_TO_README>      Use this file as readme instead of the default
    -s, --source <SOURCE>              How to create the project names. Might be:
                                           - <file>: a path to a file form which to read line by line
                                           - "random": generate a random string
    -t, --token <GITHUB_TOKEN>         The token from github to authenticate to crates.io
                                       (may also be passed in CARGO_REGISTRY_TOKEN env variable)
    -w, --workdir <PATH_TO_WORKDIR>    Path where to create the packages to be published
```

A simple run with randomly generated crate names should be:

```
$ xkcd-386 --token <YOUR_TOKEN> --source random --limit <HOW_MANY_CRATES_TO_GENERATE>
```
