use futures_util::StreamExt;
use std::{
    error::Error,
    fs::File,
    io::{Write},
    path::Path,
};

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};
mod proxy;
use lust::get_file_size;
use std::thread;
use reqwest::Url;
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

async fn get_progress(f: String, total_size: u64){
    let fs = get_file_size(&f);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
.progress_chars("#>-"));

    while fs < total_size {
        let fs = get_file_size(&f);
        pb.set_position(fs);
        // thread::sleep(Duration::from_millis(1000));
    }
    pb.finish();
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let xcgit = Xcgit{rt: rt};

    match matches.subcommand() {
        ("get", Some(m)) => xcgit.run_download(m),
        ("clone", Some(m)) => xcgit.run_clone(m),
        _ => Ok(()),
    }
}

struct Xcgit {
    rt: tokio::runtime::Runtime
}

impl Xcgit {
    async fn download(&self, url: Url) -> Result<(), Box<dyn Error>> {
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
    
        while let Some(item) = stream.next().await {
            let aaa = &item?;
            // println!("Chunk: {:?}", aaa.len());
            // let data = item?.as_ref();
            file.write(aaa.as_ref()).unwrap();
        }
    
        Ok(())
    }
    fn run_download(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let target_url = matches.value_of("ADDR").unwrap_or("example.com");
        let  url = Url::parse(target_url)?;
        ;
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
}





