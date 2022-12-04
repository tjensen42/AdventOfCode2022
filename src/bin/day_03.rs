use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input/day_03/input.txt").unwrap();

    // Part 1
    let mut sum: usize = 0;
    for line in input.lines() {
        let comp_1 = &line[..line.len() / 2];
        let comp_2 = &line[line.len() / 2..];

        'outer: for c1 in comp_1.as_bytes() {
            for c2 in comp_2.as_bytes() {
                if c1 == c2 {
                    // println!("c1: {}", *c1 as char);
                    if c1.is_ascii_uppercase() {
                        sum += *c1 as usize - 'A' as usize + 27;
                    } else {
                        sum += *c1 as usize - 'a' as usize + 1;
                    }
                    break 'outer;
                }
            }
        }
    }
    println!("#1: {}", sum);

    // Part 2
    sum = 0;
    for (g1, g2, g3) in input.split("\n").into_iter().tuples() {
        'outer: for c1 in g1.as_bytes() {
            for c2 in g2.as_bytes() {
                for c3 in g3.as_bytes() {
                    if c1 == c2 && c1 == c3 {
                        // println!("c1: {}", *c1 as char);
                        if c1.is_ascii_uppercase() {
                            sum += *c1 as usize - 'A' as usize + 27;
                        } else {
                            sum += *c1 as usize - 'a' as usize + 1;
                        }
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("#2: {}", sum);
}
