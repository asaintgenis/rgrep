extern crate clap;

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

    println!("{}", text);
}