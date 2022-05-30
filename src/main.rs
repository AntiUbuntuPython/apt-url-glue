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

    let packages = apt_uri.path().split(',').into_iter().collect::<Vec<_>>().join(" ");
    let mut child = Command::new("sudo")
        .args(["apt", "install", packages.as_str()])
        .spawn()?;

    match child.try_wait() {
        Ok(Some(status)) => println!("exited with: {}", status),
        Ok(None) => {
            let res = child.wait();
            exit(res.map(|a| a.code()).unwrap_or_default().unwrap_or(0))
        }
        Err(e) => return Err(Box::new(e)),
    };

    Ok(())
}
