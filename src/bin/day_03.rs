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

        'cmp: for c1 in comp_1.chars() {
            for c2 in comp_2.chars() {
                if c1 == c2 {
                    sum += item_to_priority(c1);
                    break 'cmp;
                }
            }
        }
    }
    println!("#1: {}", sum);

    // Part 2
    sum = 0;
    for (g1, g2, g3) in input.lines().tuples() {
        'cmp: for c1 in g1.chars() {
            for c2 in g2.chars() {
                for c3 in g3.chars() {
                    if c1 == c2 && c1 == c3 {
                        sum += item_to_priority(c1);
                        break 'cmp;
                    }
                }
            }
        }
    }
    println!("#2: {}", sum);
}
