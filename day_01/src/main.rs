use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut elves = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            sum += line.parse::<u64>().unwrap();
        } else {
            elves.push(sum);
            sum = 0;
        }
    }
    elves.sort();
    println!("#1: {}", elves.last().unwrap());
    println!("#2: {}", &elves[elves.len() - 3..].iter().sum::<u64>());
}
