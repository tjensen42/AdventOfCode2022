use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day_01/input.txt").unwrap();

    let mut elves: Vec<_> = input.split("\n\n").map(|elv| {
        elv.lines().map(|line| line.parse::<usize>().unwrap()).sum()
    }).collect();

    elves.sort();
    println!("#1: {}", elves.last().unwrap());
    println!("#2: {}", elves.iter().rev().take(3).sum::<usize>());
}
