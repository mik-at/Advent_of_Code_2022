use std::collections::HashMap;

fn part1() {
    #[derive(Debug, Default, Clone)]
    struct Monkey {
        name: u32,
        items: Vec<i32>,
        operation: String,
        operation_do: i32,
        test: i32,
        true_throw: u32,
        false_throw: u32,
        inspects: i32,
    }
    let mut result: i32 = 0;
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    let rounds = 20;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for monkey_config in input.split("\n\n") {
            // create monkeys hashmap
            let mut monkey = Monkey::default();
            for monkey_data in monkey_config.lines() {
                let monkey_data = monkey_data.trim();
                if monkey_data.starts_with("Monkey") {
                    monkey.name = monkey_data["Monkey ".len()..]
                        .replace(":", "")
                        .parse()
                        .unwrap();
                    monkey.name = monkey_data.chars().nth(7).unwrap().to_digit(10).unwrap()
                } else if monkey_data.starts_with("Starting items:") {
                    monkey.items = monkey_data["Starting items: ".len()..]
                        .split(", ")
                        .map(|item| item.parse().unwrap())
                        .collect::<Vec<i32>>()
                } else if monkey_data.starts_with("Operation:") {
                    if monkey_data.contains("+") {
                        monkey.operation = "Add".to_owned();
                        monkey.operation_do = monkey_data["Operation: new = old X ".len()..]
                            .parse()
                            .unwrap();
                    } else if monkey_data.contains("old * old") {
                        monkey.operation = "Special".to_owned();
                    } else {
                        monkey.operation = "Multiplicate".to_owned();
                        monkey.operation_do = monkey_data["Operation: new = old X ".len()..]
                            .parse()
                            .unwrap();
                    }
                } else if monkey_data.starts_with("Test:") {
                    monkey.test = monkey_data["Test: divisible by ".len()..].parse().unwrap();
                } else if monkey_data.starts_with("If true") {
                    monkey.true_throw = monkey_data["If true: throw to monkey ".len()..]
                        .parse()
                        .unwrap();
                } else if monkey_data.starts_with("If false") {
                    monkey.false_throw = monkey_data["If false: throw to monkey ".len()..]
                        .parse()
                        .unwrap();
                }
            }
            monkeys.insert(monkey.name, monkey);
        }
        for _i in 0..rounds {
            for num in 0..monkeys.len() {
                let num: u32 = num.try_into().unwrap();
                let mut monkey = monkeys.get(&num).unwrap().clone();
                for item in monkey.items {
                    monkey.inspects += 1;
                    let mut item = item;
                    if monkey.operation == "Add" {
                        item = item + monkey.operation_do;
                    } else if monkey.operation == "Multiplicate" {
                        item = item * monkey.operation_do;
                    } else {
                        item = item * item;
                    }
                    item = (item - item % 3) / 3;
                    let mut to_monkey = Monkey::default();
                    if item % monkey.test == 0 {
                        to_monkey = monkeys.get(&monkey.true_throw).unwrap().clone();
                    } else {
                        to_monkey = monkeys.get(&monkey.false_throw).unwrap().clone();
                    }
                    to_monkey.items.push(item);
                    monkeys.insert(to_monkey.name, to_monkey);
                }
                monkey.items = Vec::new();
                monkeys.insert(monkey.name, monkey);
            }
        }
        let mut inspects: Vec<i32> = Vec::new();
        for num in 0..monkeys.len() {
            inspects.push(monkeys.get(&num.try_into().unwrap()).unwrap().inspects);
        }
        inspects.sort();
        result = inspects[inspects.len() - 1] * inspects[inspects.len() - 2];
    }
    println!("Part1: {}", result);
}
fn part2() {
    #[derive(Debug, Default, Clone)]
    struct Monkey {
        name: u32,
        items: Vec<u64>,
        operation: String,
        operation_do: u64,
        test: u64,
        true_throw: u32,
        false_throw: u32,
        inspects: i32,
    }

    let mut result: u64 = 0;
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    let mut worry_manager: u64 = 1;
    let rounds = 10000;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for monkey_config in input.split("\n\n") {
            // create monkeys hashmap
            let mut monkey = Monkey::default();
            for monkey_data in monkey_config.lines() {
                let monkey_data = monkey_data.trim();
                if monkey_data.starts_with("Monkey") {
                    monkey.name = monkey_data["Monkey ".len()..]
                        .replace(":", "")
                        .parse()
                        .unwrap();
                    monkey.name = monkey_data.chars().nth(7).unwrap().to_digit(10).unwrap()
                } else if monkey_data.starts_with("Starting items:") {
                    monkey.items = monkey_data["Starting items: ".len()..]
                        .split(", ")
                        .map(|item| item.parse().unwrap())
                        .collect::<Vec<u64>>()
                } else if monkey_data.starts_with("Operation:") {
                    if monkey_data.contains("+") {
                        monkey.operation = "Add".to_owned();
                        monkey.operation_do = monkey_data["Operation: new = old X ".len()..]
                            .parse()
                            .unwrap();
                    } else if monkey_data.contains("old * old") {
                        monkey.operation = "Special".to_owned();
                    } else {
                        monkey.operation = "Multiplicate".to_owned();
                        monkey.operation_do = monkey_data["Operation: new = old X ".len()..]
                            .parse()
                            .unwrap();
                    }
                } else if monkey_data.starts_with("Test:") {
                    monkey.test = monkey_data["Test: divisible by ".len()..].parse().unwrap();
                    worry_manager *= monkey.test;
                } else if monkey_data.starts_with("If true") {
                    monkey.true_throw = monkey_data["If true: throw to monkey ".len()..]
                        .parse()
                        .unwrap();
                } else if monkey_data.starts_with("If false") {
                    monkey.false_throw = monkey_data["If false: throw to monkey ".len()..]
                        .parse()
                        .unwrap();
                }
            }
            monkeys.insert(monkey.name, monkey);
        }
        for _i in 0..rounds {
            for num in 0..monkeys.len() {
                let num: u32 = num.try_into().unwrap();
                let mut monkey = monkeys.get(&num).unwrap().clone();
                for item in monkey.items {
                    monkey.inspects += 1;
                    let mut item = item;
                    if monkey.operation == "Add" {
                        item = item + monkey.operation_do;
                    } else if monkey.operation == "Multiplicate" {
                        item = item * monkey.operation_do;
                    } else {
                        item = item * item;
                    }
                    item = item % worry_manager;
                    let mut to_monkey = Monkey::default();
                    if item % monkey.test == 0 {
                        to_monkey = monkeys.get(&monkey.true_throw).unwrap().clone();
                    } else {
                        to_monkey = monkeys.get(&monkey.false_throw).unwrap().clone();
                    }
                    to_monkey.items.push(item);
                    monkeys.insert(to_monkey.name, to_monkey);
                }
                monkey.items = Vec::new();
                monkeys.insert(monkey.name, monkey);
            }
        }
        let mut inspects: Vec<u64> = Vec::new();
        for num in 0..monkeys.len() {
            inspects.push(
                monkeys
                    .get(&num.try_into().unwrap())
                    .unwrap()
                    .inspects
                    .try_into()
                    .unwrap(),
            );
        }
        inspects.sort();
        result = inspects[inspects.len() - 1] * inspects[inspects.len() - 2]
    }
    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}
