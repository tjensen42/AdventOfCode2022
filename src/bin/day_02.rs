use core::panic;
use std::fs;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSOR: u8 = 3;

fn main() {
    let input = fs::read_to_string("./input/day_02/input.txt").unwrap();

    // Part 1
    let mut score: usize = 0;
    for line in input.lines() {
        let elf = line.as_bytes()[0] - b'A' + 1;
        let me = line.as_bytes()[2] - b'X' + 1;
        match (elf, me) {
            (SCISSOR, ROCK) | (PAPER, SCISSOR) | (ROCK, PAPER) => score += 6,
            (elf, me) => if elf == me { score += 3 },
        }
        score += me as usize;
    }
    println!("#1: {}", score);

    // Part 2
    score = 0;
    for line in input.lines() {
        let elf = line.as_bytes()[0] - b'A' + 1;
        match line.as_bytes()[2] {
            b'X' => score += match elf {
                ROCK => SCISSOR as usize,
                PAPER => ROCK as usize,
                _ => PAPER as usize,
            },
            b'Y' => score += 3 + elf as usize,
            b'Z' => score += 6 + match elf {
                ROCK => PAPER as usize,
                PAPER => SCISSOR as usize,
                _ => ROCK as usize,
            },
            _ => panic!()
        }
    }
    println!("#2: {}", score);
}
