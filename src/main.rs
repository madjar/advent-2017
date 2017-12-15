#![feature(slice_patterns)]
extern crate find_folder;
#[macro_use]
extern crate nom;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::fs::File;
use std::io::BufReader;
use find_folder::Search;

fn main() {
    let day = "08";
    match day {
        "01" => day01::doit(get_input("01")),
        "02" => day02::doit(get_input("02")),
        "03" => day03::doit(),
        "04" => day04::doit(get_input("04")),
        "05" => day05::doit(get_input("05")),
        "06" => day06::doit(get_input("06")),
        "07" => day07::doit(get_input("07")),
        "08" => day08::doit(get_input("08")),
        _ => (),
    }
}


fn get_input(file: &str) -> BufReader<File> {
    let mut path = Search::Parents(1)
        .for_folder("inputs")
        .expect("Could not find folder 'inputs'");
    path.push(file);
    let f = File::open(path).expect("file not found");
    return BufReader::new(f);
    // TODO bench mmap?
}
