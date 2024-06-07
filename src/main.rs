use std::env;
use std::fs;
use std::path::PathBuf;

use aoc;
use clap;

fn main() {
    let matches = clap::command!()
        .arg(
            clap::arg!(--day <day>)
                .short('d')
                .default_value("1")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            clap::arg!(--part[part])
                .short('p')
                .default_value("1")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            clap::arg!(<input>)
                .help("the input file for the day")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();

    let day = matches.get_one::<usize>("day").unwrap();
    let part = matches.get_one::<usize>("part").unwrap();
    let input_filename = matches.get_one::<PathBuf>("input").unwrap();
    let input = fs::read_to_string(input_filename).unwrap();

    let solutions = aoc::Solutions::new();
    solutions.solve(*day, *part, &input);
}
