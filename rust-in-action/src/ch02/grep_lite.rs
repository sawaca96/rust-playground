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

    let pattern = args.value_source("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    match args.value_of("input") {
        Some(filename) => {
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);
            process_lines(reader, re);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }
    }
}
