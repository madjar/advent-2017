extern crate find_folder;
mod day01;
mod day02;
mod day03;

use std::fs::File;
use std::io::prelude::*;
use find_folder::Search;

fn main() {
    let day = "03";
    match day {
        "01" => day01::doit(get_input("01")),
        "02" => day02::doit(get_input("02")),
        "03" => day03::doit(),
        _ => (),
    }
}


fn get_input(file: &str) -> String {
    let mut path = Search::Parents(1)
        .for_folder("inputs")
        .expect("Could not find folder 'inputs'");
    path.push(file);
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
    // TODO trim here
}
