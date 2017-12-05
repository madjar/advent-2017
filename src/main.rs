extern crate find_folder;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::fs::File;
use std::io::BufReader;
use find_folder::Search;

fn main() {
    let day = "05";
    match day {
        "01" => day01::doit(get_input("01")),
        "02" => day02::doit(get_input("02")),
        "03" => day03::doit(),
        "04" => day04::doit(get_input("04")),
        "05" => day05::doit(get_input("05")),
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
