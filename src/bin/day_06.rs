fn main() {
    let input = std::fs::read_to_string("input/day_06/input.txt").unwrap();
    let input = input.chars().collect::<Vec<char>>();

    // Part 1
    let windows = input.windows(4);
    let mut marker = std::usize::MAX;
    for (i, window) in windows.enumerate() {
        if !contains_same_char(window) {
            marker = i + 4;
            break;
        }
    }
    if marker == std::usize::MAX {
        println!("#1 No marker found");
    } else {
        println!("#1 Marker: {}", marker);
    }

    // Part 2
    let windows = input.windows(14);
    let mut marker = std::usize::MAX;
    for (i, window) in windows.enumerate() {
        if !contains_same_char(window) {
            marker = i + 14;
            break;
        }
    }
    if marker == std::usize::MAX {
        println!("#2 No marker found");
    } else {
        println!("#2 Marker: {}", marker);
    }
}

fn contains_same_char(array: &[char]) -> bool {
    for i in 0..array.len() - 1 {
        for j in i + 1..array.len() {
            if array[i] == array[j] {
                return true;
            }
        }
    }
    return false;
}
