use serde_json::{json, Value};

fn extract_data(input: &str) -> Vec<(Value, Value)> {
    let mut packet_pairs: Vec<(Value, Value)> = Vec::new();
    for packet in input.split("\n\n") {
        let (left, right) = packet.split_once("\n").unwrap();
        let left_data: Value = serde_json::from_str(left).unwrap();
        let right_data: Value = serde_json::from_str(right).unwrap();
        packet_pairs.push((left_data, right_data));
    }
    return packet_pairs;
}

fn compare_pairs(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            if left.as_u64() == right.as_u64() {
                None
            } else if left.as_u64() < right.as_u64() {
                Some(true)
            } else {
                Some(false)
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            if left.is_empty() || right.is_empty() {
                if left.len() == right.len() {
                    None
                } else if left.len() < right.len() {
                    Some(true)
                } else {
                    Some(false)
                }
            } else if let Some(value) = compare_pairs(&left[0], &right[0]) {
                Some(value)
            } else {
                compare_pairs(&json!(left[1..]), &json!(right[1..]))
            }
        }
        (Value::Number(left), Value::Array(right)) => {
            compare_pairs(&json!(vec![left]), &json!(right))
        }
        (Value::Array(left), Value::Number(right)) => {
            compare_pairs(&json!(left), &json!(vec![right]))
        }
        _ => Some(true),
    }
}

fn part1() {
    let mut result = 0;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        let mut packets = extract_data(input.as_str());
        let mut pair_index = 1;
        for pair in packets {
            if compare_pairs(&pair.0, &pair.1) == Some(true) {
                result += pair_index;
            }
            pair_index += 1;
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
