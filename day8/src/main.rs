use grid::*;

fn part1(file_path: &str) -> usize {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let row_length = &input.lines().nth(0).unwrap().len();
        let mut values: Vec<u32> = Vec::new();
        for value in input.chars() {
            if value != '\n' {
                let value: u32 = value.to_digit(10).unwrap();
                values.push(value);
            }
        }
        let grid = Grid::from_vec(values, *row_length);
        let (row_length, column_length) = grid.size();
        for row in 1..row_length - 1 {
            for column in 1..column_length - 1 {
                let mut visible_up = true;
                let mut visible_down = true;
                let mut visible_left = true;
                let mut visible_right = true;
                for look_up in 1..=column {
                    if grid[row][column - look_up] >= grid[row][column] {
                        visible_up = false;
                        break;
                    }
                }
                for look_left in 1..=row {
                    if grid[row - look_left][column] >= grid[row][column] {
                        visible_left = false;
                        break;
                    }
                }
                for look_down in 1..=(column_length - column) {
                    if column + look_down == column_length {
                        break;
                    }
                    if grid[row][column + look_down] >= grid[row][column] {
                        visible_down = false;
                        break;
                    }
                }
                for look_right in 1..=(row_length - row) {
                    if row + look_right == row_length {
                        break;
                    }
                    if grid[row + look_right][column] >= grid[row][column] {
                        visible_right = false;
                        break;
                    }
                }
                if visible_up | visible_left | visible_down | visible_right {
                    result += 1;
                }
            }
        }
        result += row_length * 2 + (column_length - 2) * 2;
    }
    println!("Part1: {}", result);
    result
}
fn part2(file_path: &str) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let row_length = &input.lines().nth(0).unwrap().len();
        let mut values: Vec<u32> = Vec::new();
        for value in input.chars() {
            if value != '\n' {
                let value: u32 = value.to_digit(10).unwrap();
                values.push(value);
            }
        }
        let grid = Grid::from_vec(values, *row_length);
        let (row_length, column_length) = grid.size();
        for row in 1..row_length - 1 {
            for column in 1..column_length - 1 {
                let mut score_up = 0;
                let mut score_down = 0;
                let mut score_left = 0;
                let mut score_right = 0;

                for look_up in 1..=column {
                    score_up += 1;
                    if grid[row][column - look_up] >= grid[row][column] {
                        break;
                    }
                }
                for look_left in 1..=row {
                    score_left += 1;
                    if grid[row - look_left][column] >= grid[row][column] {
                        break;
                    }
                }
                for look_down in 1..=(column_length - column) {
                    if column + look_down == column_length {
                        break;
                    }
                    score_down += 1;
                    if grid[row][column + look_down] >= grid[row][column] {
                        break;
                    }
                }
                for look_right in 1..=(row_length - row) {
                    if row + look_right == row_length {
                        break;
                    }
                    score_right += 1;
                    if grid[row + look_right][column] >= grid[row][column] {
                        break;
                    }
                }
                let score = score_up * score_left * score_down * score_right;
                if score > result {
                    result = score;
                }
            }
        }
    }
    println!("Part2: {}", result);
    result
}

fn main() {
    part1(INPUT_FILE);
    part2(INPUT_FILE);
}
const INPUT_FILE: &str = "./input";
#[cfg(test)]
mod tests {
    const INPUT_TEST_FILE: &str = "./input_sample";
    use crate::{part1, part2};

    #[test]
    fn part_1_sample_input() {
        assert_eq!(part1(INPUT_TEST_FILE), 21);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 8);
    }
}
