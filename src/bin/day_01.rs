use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day_01/input.txt").unwrap();

    let mut elves = Vec::new();
    for elv in input.split("\n\n") {
        let mut sum = 0;
        for line in elv.lines() {
            sum += line.parse::<usize>().unwrap();
        }
        elves.push(sum);
    }

    elves.sort();
    println!("#1: {}", elves.last().unwrap());
    println!("#2: {}", elves.iter().rev().take(3).sum::<usize>());
}
