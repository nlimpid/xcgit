use std::{
    error::Error,
    fs::File,
    i32,
    io::{self, Read, Write},
    path::Path,
};

fn main() {
    let target_url:String = "https://github.com/github/gh-ost/releases/download/v1.1.0/gh-ost-binary-linux-20200828140552.tar.gz".to_string();
    download(target_url);
}

fn download(url: String) -> Result<(), Box<dyn Error>> {
    let result = reqwest::blocking::get(&url)?.bytes()?;
    let path = Path::new("lorem_ipsum.tar.gz");

    let mut file = File::create(&path)?;

    file.write(&result)?;
    Ok(())
}
