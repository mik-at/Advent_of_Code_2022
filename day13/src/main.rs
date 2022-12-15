use serde_json::{json, Value};
use std::cmp::Ordering;

fn extract_data_in_pairs(input: &str) -> Vec<(Value, Value)> {
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

fn part1(file_path: &str) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let packets = extract_data_in_pairs(input.as_str());
        let mut pair_index = 1;
        for pair in packets {
            if compare_pairs(&pair.0, &pair.1) == Some(true) {
                result += pair_index;
            }
            pair_index += 1;
        }
    }
    println!("Part1: {}", result);
    result
}

fn extract_data(input: &str) -> Vec<Value> {
    let mut packets: Vec<Value> = Vec::new();
    for packet in input.split("\n") {
        if packet == "" {
            continue;
        }
        let data: Value = serde_json::from_str(packet).unwrap();
        packets.push(data);
    }
    return packets;
}

fn add_divider_packets(input: Vec<Value>) -> Vec<Value> {
    let mut data = input;
    data.push(json!([[2]]));
    data.push(json!([[6]]));
    return data;
}

fn part2(file_path: &str) -> i32 {
    let mut result = 1;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let mut packets = extract_data(input.as_str());
        packets = add_divider_packets(packets);

        packets.sort_by(|a, b| match compare_pairs(a, b) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            _ => Ordering::Equal,
        });
        let mut data_index = 1;
        for data in &packets {
            if data.get(0) != None
                && data.get(1) == None
                && data.get(0).unwrap().get(0) != None
                && data.get(0).unwrap().get(1) == None
                && data.get(0).unwrap().get(0).unwrap().is_number()
                && (data.get(0).unwrap().get(0).unwrap() == 2
                    || data.get(0).unwrap().get(0).unwrap() == 6)
            {
                result *= data_index;
            }

            data_index += 1;
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
        assert_eq!(part1(INPUT_TEST_FILE), 13);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 140);
    }
}
