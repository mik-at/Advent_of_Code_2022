use itertools::Itertools;

fn part1() {
    let mut buffer: Vec<char> = Vec::new();
    let mut result = 0;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for char in input.chars() {
            if buffer.len() < 4 {
                buffer.push(char);
            } else {
                let marker: [char; 4] = [buffer[buffer.len()-4], buffer[buffer.len()-3], buffer[buffer.len()-2], buffer[buffer.len()-1]];
                if marker.iter().unique().count() == 4 {
                    result = buffer.len();
                    break;
                }
                buffer.push(char);
            }
        }
    }
    println!("Part1: {}", result);
}
fn part2() {
    let mut buffer: Vec<char> = Vec::new();
    let mut result = 0;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for char in input.chars() {
            if buffer.len() < 14 {
                buffer.push(char);
            } else {
                let marker: [char; 14] = [buffer[buffer.len()-14], buffer[buffer.len()-13], buffer[buffer.len()-12], buffer[buffer.len()-11], buffer[buffer.len()-10], buffer[buffer.len()-9], buffer[buffer.len()-8], buffer[buffer.len()-7], buffer[buffer.len()-6], buffer[buffer.len()-5], buffer[buffer.len()-4], buffer[buffer.len()-3], buffer[buffer.len()-2], buffer[buffer.len()-1]];
                if marker.iter().unique().count() == 14 {
                    result = buffer.len();
                    break;
                }
                buffer.push(char);
            }
        }
    }
    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}