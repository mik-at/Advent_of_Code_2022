use std::{collections::HashMap, char};
use regex::Regex;

fn part1() {
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for line in input.lines() { // create the stacks HashMap
            if line.starts_with(" 1") {
                break;
            }
            let re = Regex::new(".(.)..?").unwrap();
            let items: Vec<&str> = re.find_iter(line).map(|mat| mat.as_str().trim()).collect();
            let mut i = 1;
            for item in items {
                if item == "" {
                    i += 1
                } else {
                    let item = item.chars().nth(1).unwrap();
                    if stacks.contains_key(&i) {
                        let mut stack = stacks[&i].to_owned();
                        stack.insert(0,item);
                        stacks.insert(i, stack);
                    } else {
                        let mut stack: Vec<char> = Vec::new();
                        stack.insert(0,item);
                        stacks.insert(i, stack);
                    }
                    i += 1;
                }
            }
        }
        for line in input.lines() {
            if line.starts_with("move") {
                let mut command = line.split(" ");
                let (quantity, from, to) = (command.nth(1).unwrap().parse::<i32>().unwrap(), command.nth(1).unwrap().parse::<i32>().unwrap(), command.nth(1).unwrap().parse::<i32>().unwrap());
                /* Note that all preceding elements, as well as the returned element, will be consumed from the iterator.
                That means that the preceding elements will be discarded, and also that calling nth(0) multiple times on the same iterator will return different elements. */
                for _i in 0..quantity {
                    let mut from_stack = stacks[&from].to_owned();
                    let mut to_stack = stacks[&to].to_owned();
                    to_stack.push(*from_stack.last().unwrap());
                    from_stack.remove(from_stack.len()-1);
                    stacks.insert(from, from_stack);
                    stacks.insert(to, to_stack);
                }

            }
        }
        let mut result: Vec<char> = Vec::new();
        let mut i = 1;
        for _stack in &stacks {
            result.push(*stacks[&i].last().unwrap());
            i += 1;
        }
        let result: String = result.into_iter().collect();
        println!("Part1: {}", result);
    }
}

fn part2() {
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for line in input.lines() { // create the stacks HashMap
            if line.starts_with(" 1") {
                break;
            }
            let re = Regex::new(".(.)..?").unwrap();
            let items: Vec<&str> = re.find_iter(line).map(|mat| mat.as_str().trim()).collect();
            let mut i = 1;
            for item in items {
                if item == "" {
                    i += 1
                } else {
                    let item = item.chars().nth(1).unwrap();
                    if stacks.contains_key(&i) {
                        let mut stack = stacks[&i].to_owned();
                        stack.insert(0,item);
                        stacks.insert(i, stack);
                    } else {
                        let mut stack: Vec<char> = Vec::new();
                        stack.insert(0,item);
                        stacks.insert(i, stack);
                    }
                    i += 1;
                }
            }
        }
        for line in input.lines() {
            if line.starts_with("move") {
                let mut command = line.split(" ");
                let (quantity, from, to) = (command.nth(1).unwrap().parse::<i32>().unwrap(), command.nth(1).unwrap().parse::<i32>().unwrap(), command.nth(1).unwrap().parse::<i32>().unwrap());
                /* Note that all preceding elements, as well as the returned element, will be consumed from the iterator.
                That means that the preceding elements will be discarded, and also that calling nth(0) multiple times on the same iterator will return different elements. */
                for i in 0..quantity {
                    let mut from_stack = stacks[&from].to_owned();
                    let mut to_stack = stacks[&to].to_owned();
                    let diff = i as usize;
                    to_stack.insert(to_stack.len() - diff, *from_stack.last().unwrap());
                    from_stack.remove(from_stack.len()-1);
                    stacks.insert(from, from_stack);
                    stacks.insert(to, to_stack);
                }

            }
        }
        let mut result: Vec<char> = Vec::new();
        let mut i = 1;
        for _stack in &stacks {
            result.push(*stacks[&i].last().unwrap());
            i += 1;
        }
        let result: String = result.into_iter().collect();
        println!("Part2: {}", result);
    
    }
}

fn main() {
    part1();
    part2();
    }