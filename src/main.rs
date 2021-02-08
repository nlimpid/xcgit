use std::{
    error::Error,
    fs::File,
    i32,
    io::{self, Read, Write},
    path::Path,
};

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use log::logger;
mod proxy;

fn main() {
    let matches = App::new("xcgit")
        .version("v0.1")
        .subcommand(
            SubCommand::with_name("get").arg(
                Arg::with_name("addr")
                    .long("addr")
                    .value_name("addr")
                    .help("Sets a custom config file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("clone").arg(
                Arg::with_name("addr")
                    .help("git command clone")
                    .takes_value(true),
            ),
        )
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) -> Result<(), String> {
    // ...
    let logger = log::logger();
    match matches.subcommand() {
        ("get", Some(m)) => run_download(m),
        ("clone", Some(m)) => run_clone(m),
        _ => Ok(()),
    }
}

fn download(url: String) -> Result<(), Box<dyn Error>> {
    let result = reqwest::blocking::get(&url)?.bytes()?;
    let path = Path::new("lorem_ipsum.tar.gz");

    let mut file = File::create(&path)?;

    file.write(&result)?;
    Ok(())
}

fn run_download(matches: &ArgMatches) -> Result<(), String> {
    let target_url = matches.value_of("addr").unwrap_or("example.com");
    let download_err = download(target_url.to_string());
    match download_err {
        Ok(v) => {}
        Err(e) => {
            print!("{} is error", e.to_string())
        }
    }
    let input = matches.value_of("input-file").unwrap();
    download(input.to_string());
    Ok(())
}

fn run_clone(matches: &ArgMatches) -> Result<(), String> {
    let url = matches.value_of("addr").unwrap();
    proxy::clone(url.to_string());
    Ok(())
}
