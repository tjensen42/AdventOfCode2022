fn main() {
    let input = std::fs::read_to_string("./input/day_08/input.txt").unwrap();
    let grid: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();

    // Part 1
    let mut sum = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if is_visible(&grid, row, column) {
                sum += 1;
            }
        }
    }
    println!("#1 Visible Trees: {}", sum);

    // Part 2
    let mut score = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            let tmp = scenic_score(&grid, row, column);
            if tmp > score {
                score = tmp;
            }
        }
    }
    println!("#2 Highest Scenic Score: {}", score);
}

//
// Part 1
//

fn is_visible(grid: &Vec<&[u8]>, row: usize, column: usize) -> bool {
    if row == 0 || row == grid.len() - 1 || column == 0 || column == grid.len() - 1 {
        return true;
    }
    if is_visible_top(grid, row, column)
        || is_visible_bottom(grid, row, column)
        || is_visible_left(grid, row, column)
        || is_visible_right(grid, row, column)
    {
        return true;
    }
    false
}

fn is_visible_top(grid: &Vec<&[u8]>, row: usize, column: usize) -> bool {
    for i in 0..row {
        if grid[i][column] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn is_visible_bottom(grid: &Vec<&[u8]>, row: usize, column: usize) -> bool {
    for i in row + 1..grid.len() {
        if grid[i][column] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn is_visible_left(grid: &Vec<&[u8]>, row: usize, column: usize) -> bool {
    for i in 0..column {
        if grid[row][i] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn is_visible_right(grid: &Vec<&[u8]>, row: usize, column: usize) -> bool {
    for i in column + 1..grid[row].len() {
        if grid[row][i] >= grid[row][column] {
            return false;
        }
    }
    true
}

//
// Part 2
//

fn scenic_score(grid: &Vec<&[u8]>, row: usize, column: usize) -> usize {
    scenic_score_up(grid, row, column)
        * scenic_score_down(grid, row, column)
        * scenic_score_left(grid, row, column)
        * scenic_score_right(grid, row, column)
}

fn scenic_score_up(grid: &Vec<&[u8]>, row: usize, column: usize) -> usize {
    let mut score = 0;
    for i in (0..row).rev() {
        score += 1;
        if grid[i][column] >= grid[row][column] {
            break;
        }
    }
    score
}

fn scenic_score_down(grid: &Vec<&[u8]>, row: usize, column: usize) -> usize {
    let mut score = 0;
    for i in row + 1..grid.len() {
        score += 1;
        if grid[i][column] >= grid[row][column] {
            break;
        }
    }
    score
}

fn scenic_score_left(grid: &Vec<&[u8]>, row: usize, column: usize) -> usize {
    let mut score = 0;
    for i in (0..column).rev() {
        score += 1;
        if grid[row][i] >= grid[row][column] {
            break;
        }
    }
    score
}

fn scenic_score_right(grid: &Vec<&[u8]>, row: usize, column: usize) -> usize {
    let mut score = 0;
    for i in column + 1..grid[row].len() {
        score += 1;
        if grid[row][i] >= grid[row][column] {
            break;
        }
    }
    score
}
