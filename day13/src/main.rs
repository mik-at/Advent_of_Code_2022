fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}
fn extract_data(packet: &str) -> Vec<Vec<i32>> {
    //let packet = rem_first(packet);
    let mut data: Vec<Vec<i32>> = Vec::new();
    let mut value = "".to_owned();
    let mut lists: Vec<String> = Vec::new();
    for char in packet.chars() {
        match char as u8 {
            b'[' => {
                value.push(char);
            }
            b'0'..=b'9' => {
                value.push(char);
            }
            b']' => {
                //value.push(char);
                lists.push(value);
                value = "".to_owned();
            }
            b',' => {
                value.push(']');
                value.push(char);
                value.push('[');
            }
            _ => (),
        }
    }
    dbg!(&lists);
    for list in lists {
        let list = list.trim_matches(|c| c == '[' || c == ']' || c == ',');
        let mut list_vec: Vec<i32> = Vec::new();
        for value in list.split(",") {
            //dbg!(&value);
            let value = value.trim_matches(|c| c == '[' || c == ']');
            if value != "" {
                list_vec.push(value.parse().unwrap());
            }
        }
        data.push(list_vec);
    }
    return data;
}

fn part1() {
    let mut result = "";
    if let Ok(input) = std::fs::read_to_string("./input_sample") {
        //if let Ok(input) = std::fs::read_to_string("./input") {
        for substring in input.split("\n\n") {
            let (left, right) = substring.split_once("\n").unwrap();
            let left_data = extract_data(left);
            let right_data = extract_data(right);
            //dbg!(left_data, right_data);
        }
    }
    println!("Part1: {}", result);
}
fn part2() {
    let mut result = "";
    if let Ok(input) = std::fs::read_to_string("./input_sample") {
        //if let Ok(input) = std::fs::read_to_string("./input") {
        // Code here
    }
    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}
