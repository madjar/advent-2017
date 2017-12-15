use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn doit(mut input: BufReader<File>) {
    let mut sum = 0;
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let list: Vec<u32> = s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = list.len();


    for i in 0..n {
        if list[i] == list[(i + 1) % n] {
            sum += list[i];
        }
    }
    println!("Day 1, part 2: {}", sum);
}
