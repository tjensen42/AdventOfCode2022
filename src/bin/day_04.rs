use std::{fs, ops::RangeInclusive};

fn range_includes_range(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    if r1.end() - r1.start() > r2.end() - r2.start() {
        r2.start() >= r1.start() && r2.end() <= r1.end()
    } else {
        r1.start() >= r2.start() && r1.end() <= r2.end()
    }
}

fn ranges_overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.start() <= r2.end() && r1.end() >= r2.start()
}

fn main() {
    let input = fs::read_to_string("./input/day_04/input.txt").unwrap();

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for line in input.lines() {
        let assignments: Vec<_> = line.split(&['-', ','][..]).map(|x| x.parse::<usize>().unwrap()).collect();
        let range_1 = assignments[0]..=assignments[1];
        let range_2 = assignments[2]..=assignments[3];

        // Part 1
        if range_includes_range(&range_1, &range_2) {
            sum_1 += 1;
        }

        // Part 2
        if ranges_overlap(&range_1, &range_2) {
            sum_2 += 1;
        }
    }
    println!("#1: {}", sum_1);
    println!("#2: {}", sum_2);
}
