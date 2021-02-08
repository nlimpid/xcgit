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
    let repo = match Repository::clone(&url, "hellogitworld") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
