use std::{
    error::Error,
    fs::File,
    i32,
    io::{self, Read, Write},
    path::Path,
};

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};
use log::logger;
mod proxy;
use lust::get_file_size;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("xcgit")
        .version("v0.1")
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("ADDR")))
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

fn getProgress(f: String, total_size: u64) {
    let fs = get_file_size(&f);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
.progress_chars("#>-"));

    while fs < total_size {
        let fs = get_file_size(&f);
        pb.set_position(fs);
        thread::sleep(Duration::from_millis(1000));
    }
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
    let path = Path::new("lorem_ipsum.tar.gz");

    let resp = reqwest::blocking::get(&url)?;
    let file_size = resp.content_length().unwrap();

    thread::spawn(move || getProgress("lorem_ipsum.tar.gz".to_string(), file_size));
    let result = resp.bytes()?;

    let mut file = File::create(&path)?;

    file.write(&result)?;
    Ok(())
}

fn run_download(matches: &ArgMatches) -> Result<(), String> {
    let target_url = matches.value_of("ADDR").unwrap_or("example.com");
    let download_err = download(target_url.to_string());
    match download_err {
        Ok(v) => {}
        Err(e) => {
            print!("{} is error", e.to_string())
        }
    }
    let input = matches.value_of("ADDR").unwrap();
    download(input.to_string());
    Ok(())
}

fn run_clone(matches: &ArgMatches) -> Result<(), String> {
    let url = matches.value_of("addr").unwrap();
    proxy::clone(url.to_string());
    Ok(())
}
