use git2::Repository;
use std::{
    error::Error,
    fs::File,
    i32,
    io::{self, Read, Write},
    path::Path,
};

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn clone(url: String) {
    let url = replace(url, "".to_string());
    let repo = match Repository::clone(&url, "hellogitworld") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn replace(url: String, proxy: String) -> String {
    return proxy_gh(url);
}

pub fn proxy_gh(url: String) -> String {
    let base_url = "https://ghproxy.com/";

    let result = base_url.to_owned() + &url;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gh_example() {
        assert_eq!(
            "https://ghproxy.com/https://github.com/nlimpid/xcgit",
            proxy_gh("https://github.com/nlimpid/xcgit".to_string())
        );
    }
}
