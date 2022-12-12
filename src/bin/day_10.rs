fn main() {
    let input = std::fs::read_to_string("input/day_10/input.txt").unwrap();

    // Part 1
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;
    for input in input.trim().split_whitespace() {
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
        match input {
            "noop" => (),
            "addx" => (),
            _ => x += input.parse::<i32>().unwrap(),
        }
        cycle += 1;
    }
    println!("#1 Sum: {}", sum);

    // Part 2
    let mut x = 1;
    let mut cycle = 1;
    let mut crt = String::with_capacity(240 + 6);
    for input in input.trim().split_whitespace() {
        if (cycle - 1) % 40 >= x - 1 && (cycle - 1) % 40 <= x + 1 {
            crt.push('#');
        } else {
            crt.push('.');
        }
        if cycle % 40 == 0 {
            crt.push('\n');
        }
        match input {
            "noop" => (),
            "addx" => (),
            _ => x += input.parse::<i32>().unwrap(),
        }
        cycle += 1;
    }
    println!("#2 CRT:\n{}", crt.len());
}
