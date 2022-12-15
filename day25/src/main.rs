fn part1(file_path: &str) -> &str {
    let mut result = "";
    if let Ok(input) = std::fs::read_to_string(file_path) {
        // Code here
    }
    println!("Part1: {}", result);
    result
}
fn part2(file_path: &str) -> &str {
    let mut result = "";
    if let Ok(input) = std::fs::read_to_string(file_path) {
        // Code here
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
        assert_eq!(part1(INPUT_TEST_FILE), "");
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), "");
    }
}
