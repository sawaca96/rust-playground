#![allow(dead_code)]
use clap::{Arg, Command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => continue,
        }
    }
}

pub fn run() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(Arg::new("input").help("File to search").required(false))
        .get_matches();

    // Get the pattern value from the command line arguments
    let pattern = args.get_one::<String>("pattern").unwrap();
    let default = String::from("1");
    let input = args.get_one::<String>("input").unwrap_or(&default);

    let re = Regex::new(&pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let file = File::open("./src/ch02/".to_owned() + input).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, re);
    }
}
