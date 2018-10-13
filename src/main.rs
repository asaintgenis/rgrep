extern crate clap;
extern crate colored;

use std::io::{self, BufRead};
use clap::{Arg, App};
use colored::*;

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
        let is_found = search_in_line(&to_search, text);
        match is_found {
            true => print_found_line(&x, &to_search, text),
            false => ()
        }
        x = x + 1;
    }
}

fn search_in_line(line: &String, to_find: &str) -> bool {
    return line.contains(to_find);
}

fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.red().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}

#[cfg(test)]
mod tests {

    use search_in_line;

    #[test]
    fn test_search_in_line_found() {
        let line = "test found data".to_string();
        let to_found = "found";

        let is_found = search_in_line(&line, to_found);

        assert_eq!(is_found, true);
    }

    #[test]
    fn test_search_in_line_not_found() {
        let line = "test found data".to_string();
        let to_found = "not";

        let is_found = search_in_line(&line, to_found);

        assert_eq!(is_found, false);
    }
}