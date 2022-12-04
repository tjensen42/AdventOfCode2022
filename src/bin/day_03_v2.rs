use std::fs;
use itertools::Itertools;

fn item_to_priority(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn main() {
    let input = fs::read_to_string("./input/day_03/input.txt").unwrap();

    // Part 1
    let mut sum: usize = 0;
    for line in input.lines() {
        let comp_1 = &line[..line.len() / 2];
        let comp_2 = &line[line.len() / 2..];
        let same_c = comp_1.chars().find(|c1| comp_2.chars().any(|c2| c1 == &c2));
        sum += item_to_priority(same_c.unwrap());
    }
    println!("#1: {}", sum);

    // Part 2
    sum = 0;
    for (g1, g2, g3) in input.lines().tuples() {
        let same_c = g1.chars().find(|c1| g2.chars().any(|c2| c1 == &c2) && g3.chars().any(|c3| c1 == &c3));
        sum += item_to_priority(same_c.unwrap());
    }
    println!("#2: {}", sum);
}
