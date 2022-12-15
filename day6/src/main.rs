use itertools::Itertools;

fn part1(file_path: &str) -> usize {
    let mut buffer: Vec<char> = Vec::new();
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        for char in input.chars() {
            if buffer.len() >= 4 {
                let marker: &[char] = &buffer[buffer.len() - 4..=buffer.len() - 1];
                if marker.iter().unique().count() == 4 {
                    result = buffer.len();
                    break;
                }
            }
            buffer.push(char);
        }
    }
    println!("Part1: {}", result);
    result
}
fn part2(file_path: &str) -> usize {
    let mut buffer: Vec<char> = Vec::new();
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        for char in input.chars() {
            if buffer.len() >= 14 {
                let marker: &[char] = &buffer[buffer.len() - 14..=buffer.len() - 1];
                if marker.iter().unique().count() == 14 {
                    result = buffer.len();
                    break;
                }
            }
            buffer.push(char);
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
        assert_eq!(part1(INPUT_TEST_FILE), 7);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 19);
    }
}
