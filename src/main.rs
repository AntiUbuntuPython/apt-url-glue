use std::error::Error;
use std::process::{Command, exit};
use clap::{Parser};
use url::Url;

#[derive(Parser)]
struct Args {
    apt_uri: Url
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { apt_uri } = Args::parse();
    if apt_uri.scheme() != "apt" {
        eprintln!("the schema is not apt. aborting.");
        exit(1)
    }

    let packages = apt_uri.path().split(' ').into_iter().collect::<Vec<_>>().join(" ");
    let p = Command::new("sudo")
        .args(["apt", "install", packages.as_str()])
        .spawn()?
        .wait()?;

    p.code().map_or_else(|| exit(0), |code| exit(code))
}
