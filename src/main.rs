extern crate clap;

use std::io::{self, BufRead, Error};
use clap::{Arg, App};

fn main() {
    let matches = App::new("rgrep")
        .version("0.1.0")
        .author("Arthur Saint-Genis <arthur@codeshuffle.org>")
        .about("grep clone written in Rust")
        .arg(Arg::with_name("text")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("text to find"))
        .get_matches();
    let text = matches.value_of("text").unwrap();

    let stdin = io::stdin();
    let mut x = 1;
    for line in stdin.lock().lines() {
        let to_search = line.unwrap();
        let result = search_in_line(&to_search, text);
        match result {
            Ok(true) => println!("{}:{}", x, to_search),
            Ok(false) => (),
            Err(_) => println!("fatal error")
        }
        x = x + 1;
    }

    println!("{}", text);
}

fn search_in_line(line: &String, to_find: &str) -> Result<bool, Error> {
    let v: Vec<_> = line.match_indices(to_find).collect();
    if v.len() > 0 {
        return Ok(true);
    } else {
        return Ok(false);
    }
}