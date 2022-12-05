fn main() {
    let input = std::fs::read_to_string("input/day_05/input.txt").unwrap();

    let mut stacks: [String; 9] = Default::default();
    stacks[0] = "SZPDLBFC".to_string();
    stacks[1] = "NVGPHWB".to_string();
    stacks[2] = "FWBJG".to_string();
    stacks[3] = "GJNFLWCS".to_string();
    stacks[4] = "WJLTPMSH".to_string();
    stacks[5] = "BCWGFS".to_string();
    stacks[6] = "HTPMQBW".to_string();
    stacks[7] = "FSWT".to_string();
    stacks[8] = "NCR".to_string();

    println!("#1: {}", part_1(input.clone(), stacks.clone()));
    println!("#2: {}", part_2(input.clone(), stacks.clone()));
}

fn part_1(input: String, mut stacks: [String; 9]) -> String {
    for line in input.lines().skip(10) {
        let moves: Vec<_> = line.split(" ").collect();
        let items = moves[1].parse::<usize>().unwrap();
        let source = moves[3].parse::<usize>().unwrap() - 1;
        let target = moves[5].parse::<usize>().unwrap() - 1;
        for _ in 0..items {
            stacks[target].push(stacks[source].chars().last().unwrap());
            stacks[source].pop().unwrap();
        }
    }
    let mut solution = String::new();
    for stack in stacks.iter() {
        solution.push(stack.chars().last().unwrap());
    }
    solution
}

fn part_2(input: String, mut stacks: [String; 9]) -> String {
    for line in input.lines().skip(10) {
        let moves: Vec<_> = line.split(" ").collect();
        let items = moves[1].parse::<usize>().unwrap();
        let source = moves[3].parse::<usize>().unwrap() - 1;
        let target = moves[5].parse::<usize>().unwrap() - 1;
        let item_str = &stacks[source][stacks[source].len() - items..].to_string();
        stacks[target].push_str(item_str.as_str());
        for _ in 0..items {
            stacks[source].pop();
        }
    }
    let mut solution = String::new();
    for stack in stacks.iter() {
        solution.push(stack.chars().last().unwrap());
    }
    solution
}
