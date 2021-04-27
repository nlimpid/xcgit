use futures_util::StreamExt;
use std::{error::Error, fs::File, io::Write, path::Path};

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};
mod proxy;
use lust::get_file_size;
use reqwest::Url;
use std::thread;
use std::time::Duration;
extern crate crypto;
use crypto::{digest::Digest, sha1::Sha1};

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
        .subcommand(
            SubCommand::with_name("verify").arg(
                Arg::with_name("addr")
                    .help("verify the data")
                    .takes_value(true),
            ),
        )
        .get_matches();

    run(matches);
}

async fn get_progress(f: String, total_size: u64) {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
.progress_chars("#>-"));

    loop {
        let fs = get_file_size(&f);
        pb.set_position(fs);
        if fs >= total_size {
            break;
        }
    }
    pb.finish();
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let xcgit = Xcgit { rt: rt };

    match matches.subcommand() {
        ("get", Some(m)) => xcgit.run_download(m),
        ("clone", Some(m)) => xcgit.run_clone(m),
        ("verify", Some(m)) => xcgit.run_verify(m),
        _ => Ok(()),
    }
}

struct Xcgit {
    rt: tokio::runtime::Runtime,
}

impl Xcgit {
    async fn download(&self, url: Url) -> Result<(Vec<u8>), Box<dyn Error>> {
        let resp = reqwest::get(url.clone()).await?;

        // let download_filename = s.unwrap().clone().last();
        let file_size = resp.content_length();
        let mut stream = resp.bytes_stream();
        let download_filename = url.path_segments().unwrap().into_iter().last().unwrap();
        let path = Path::new(download_filename);
        let progress_fu = get_progress(download_filename.to_string(), file_size.unwrap());
        let task = self.rt.spawn(progress_fu);
        // thread::spawn(move || get_progress(download_filename.to_string(), file_size.unwrap()));

        let mut file = File::create(&path)?;
        let mut vec: Vec<u8> = Vec::new();

        while let Some(item) = stream.next().await {
            let aaa = &item?;
            // println!("Chunk: {:?}", aaa.len());
            // let data = item?.as_ref();
            file.write(aaa.as_ref()).unwrap();
            vec.extend_from_slice(aaa.as_ref());
        }
        let c = vec.as_slice().to_owned();

        Ok((c))
    }
    fn run_download(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let target_url = matches.value_of("ADDR").unwrap_or("example.com");
        let url = Url::parse(target_url)?;
        let download_err = self.download(url.clone());
        let d_err = self.rt.block_on(download_err);
        match d_err {
            Ok(_) => {}
            Err(e) => {
                print!("{} is error", e.to_string())
            }
        }
        Ok(())
    }

    fn run_clone(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let url = matches.value_of("addr").unwrap_or("hello");
        proxy::clone(url.to_string());
        Ok(())
    }

    fn run_verify(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let target_url = matches.value_of("addr").unwrap();
        let url = Url::parse(target_url);
        let mut hasher = Sha1::new();

        let value = self.rt.block_on(self.download(url.unwrap()));
        match value {
            Ok(v) => {
                hasher.input(v.as_slice());
                let hex = hasher.result_str();
                println!("md5 is {0}", hex);
            }
            Err(e) => println!("box err"),
        }
        Ok(())
    }
}
