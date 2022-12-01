use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut sum = 0;
    let mut elves = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    elves.sort();
    println!("#1: {}", elves.last().unwrap());
    println!("#2: {}", &elves[elves.len() - 3..].iter().sum::<usize>());
}
