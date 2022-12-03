use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    let mut priorities: HashMap<char, i32> = HashMap::new();
    let mut i = 1;
    let lowercase: Vec<char> = ('a'..='z').collect();
    for char in lowercase {
        priorities.insert(char, i);
        i += 1;
    }
    let uppercase: Vec<char> = ('A'..='Z').collect();
    for char in uppercase {
        priorities.insert(char, i);
        i += 1
    }
    //println!("{:?}", priorities );
    if let Ok(input) = std::fs::read_to_string("./input.txt") {
        let cargo = input.split("\n");
        for rucksack in cargo {
            let (first, last) = rucksack.split_at(rucksack.len()/2);
            let first = first.chars();
            for item in first {
                if last.contains(item) {
                    //println!("{item}");
                    sum += priorities[&item];
                    break;
                }

            }
        }
    }
    println!("Part1 result: {sum}")
}
