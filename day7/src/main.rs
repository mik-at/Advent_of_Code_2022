use regex::Regex;
use std::collections::HashMap;

fn part1(file_path: &str) -> i32 {
    let mut result = 0;
    let mut path: String = "".to_owned();
    let mut filesystem: HashMap<String, i32> = HashMap::new();
    if let Ok(input) = std::fs::read_to_string(file_path) {
        for command in input.lines() {
            // make filesystem
            if command.starts_with("$ cd") {
                let command_sections = command.split(' ');
                let directory = command_sections.into_iter().nth(2).unwrap();
                if directory != "/" {
                    path = path + directory + "/";
                } else {
                    path = directory.to_owned();
                }
                while path.contains("..") {
                    let re = Regex::new(r"[a-zA-Z]*/\.\./").unwrap();
                    path = re.replace(&path, "").to_string();
                }
                if !filesystem.contains_key(&path) {
                    filesystem.insert(path.clone(), 0);
                }
            } else if !command.starts_with('$') && !command.starts_with("dir") {
                let command_parts = command.split(' ');
                let size = command_parts.into_iter().next().unwrap();
                let mut size: i32 = size.parse().unwrap();
                size += filesystem[&path];
                filesystem.insert(path.clone(), size);
            }
        }
        let mut parent_directory_sizes: HashMap<&str, i32> = HashMap::new();
        for current_directory in &filesystem {
            let dir = current_directory.0;
            let mut size = current_directory.1.to_owned();
            for dirs in &filesystem {
                if dirs.0.starts_with(dir) && dirs.0 != dir {
                    size += dirs.1
                }
                parent_directory_sizes.insert(dir, size);
            }
        }
        for dirs in parent_directory_sizes {
            if dirs.1 < 100000 {
                result += dirs.1;
            }
        }
    }
    println!("Part1: {}", result);
    result
}
fn part2(file_path: &str) -> i32 {
    let mut result = 0;
    let mut path: String = "".to_owned();
    let mut filesystem: HashMap<String, i32> = HashMap::new();
    if let Ok(input) = std::fs::read_to_string(file_path) {
        for command in input.lines() {
            // make filesystem
            if command.starts_with("$ cd") {
                let command_sections = command.split(' ');
                let directory = command_sections.into_iter().nth(2).unwrap();
                if directory != "/" {
                    path = path + directory + "/";
                } else {
                    path = directory.to_owned();
                }
                while path.contains("..") {
                    let re = Regex::new(r"[a-zA-Z]*/\.\./").unwrap();
                    path = re.replace(&path, "").to_string();
                }
                if !filesystem.contains_key(&path) {
                    filesystem.insert(path.clone(), 0);
                }
            } else if !command.starts_with('$') && !command.starts_with("dir") {
                let command_parts = command.split(' ');
                let size = command_parts.into_iter().next().unwrap();
                let mut size: i32 = size.parse().unwrap();
                size += filesystem[&path];
                filesystem.insert(path.clone(), size);
            }
        }
        let mut parent_directory_sizes: HashMap<&str, i32> = HashMap::new();
        for current_directory in &filesystem {
            let dir = current_directory.0;
            let mut size = current_directory.1.to_owned();
            for dirs in &filesystem {
                if dirs.0.starts_with(dir) && dirs.0 != dir {
                    size += dirs.1
                }
                parent_directory_sizes.insert(dir, size);
            }
        }
        let space_occupied = parent_directory_sizes["/"];
        let space_needed = 30000000;
        let total_space = 70000000;
        let space_free = total_space - space_occupied;
        for dirs in parent_directory_sizes {
            if space_free + dirs.1 > space_needed
                && dirs.0 != "/"
                && (result == 0 || dirs.1 < result)
            {
                result = dirs.1;
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
        assert_eq!(part1(INPUT_TEST_FILE), 95437);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 24933642);
    }
}
