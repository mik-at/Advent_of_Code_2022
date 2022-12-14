use serde_json::{value, Result, Value};

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
fn extract_data(packet: &str) -> (Value, Value) {
    let (left, right) = packet.split_once("\n").unwrap();
    let left = r#"{ "data": "#.to_owned() + left + r"}";
    let right = r#"{ "data": "#.to_owned() + right + r"}";
    let left_data: Value = serde_json::from_str(left.as_str()).unwrap();
    let right_data: Value = serde_json::from_str(right.as_str()).unwrap();
    return (left_data, right_data);
}

fn part1() {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string("./input_sample") {
    //if let Ok(input) = std::fs::read_to_string("./input") {
        let mut packet_index = 1;
        for packet in input.split("\n\n") {
            let (left, right) = extract_data(packet);
            let mut count = 0;
            let mut order_ok: Option<bool> = None;
            loop {
                let left_value = &left["data"][count];
                let right_value = &right["data"][count];
                let left_vec: Vec<Value> = if left_value.is_array() {
                    left_value.as_array().unwrap().to_owned()
                } else if left_value.is_number() {
                    vec![left_value.to_owned()]
                } else {
                    Vec::new()
                };
                let right_vec: Vec<Value> = if right_value.is_array() {
                    right_value.as_array().unwrap().to_owned()
                } else if right_value.is_number() {
                    vec![right_value.to_owned()]
                } else {
                    Vec::new()
                };
                let mut vec_iter = 0;
                for _value in &left_vec {
                    if packet_index == 8 {
                        dbg!(left_vec.get(vec_iter), right_vec.get(vec_iter));
                    }
                    if right_vec.get(vec_iter).is_none() {
                        println!("right ran out first");
                        order_ok = Some(false);
                        break;
                    }

                    let mut left_check = left_vec.get(vec_iter).unwrap();
                    let mut right_check = right_vec.get(vec_iter).unwrap();

                    while !left_check.is_number() {
                        if left_check.is_array() {
                            left_check = left_check.as_array().unwrap().iter().next().unwrap();
                        }
                    }
                    while !right_check.is_number() {
                        if right_check.is_array() {
                            right_check = right_check.as_array().unwrap().iter().next().unwrap()
                        }
                    }
                    let left_int = left_check.as_i64().unwrap();
                    let right_int = right_check.as_i64().unwrap();
                    dbg!(left_int, right_int);
                    if left_int < right_int {
                        println!("left is smaller");
                        order_ok = Some(true);
                        break;
                    } else if left_int > right_int {
                        println!("right is smaller");
                        order_ok = Some(false);
                        break;
                    }
                    vec_iter += 1;
                }
                if order_ok == None {
                    let mut vec_iter = 0;
                    for _value in &right_vec {
                        if left_vec.get(vec_iter).is_none() {
                            println!("left ran out first");
                            order_ok = Some(true);
                            break;
                        }
                        vec_iter += 1;
                    }
                }

                if order_ok != None {
                    println!("order_ok is set");
                    break;
                }
                if right_value.is_null() && left_value.is_null() {
                    println!("everything is null");
                    break;
                }

                count += 1;
            }
            dbg!(packet_index, order_ok);
            if order_ok == Some(true) {
                result += packet_index;
            }
            packet_index += 1;
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
