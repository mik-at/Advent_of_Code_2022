use std::collections::HashMap;

fn main() {
    let mut sump1 = 0;
    let mut sump2 = 0;
    let mut priorities: HashMap<&char, i32> = HashMap::new();
    let mut i = 1;
    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = ('A'..='Z').collect();
    let mut items: Vec<char> = Vec::new();
    items.extend(lowercase);
    items.extend(uppercase);
    for item in &items {
        priorities.insert(item, i);
        i += 1;
    }
    //println!("{:?}", priorities );
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        let cargo = input.lines();
        for rucksack in cargo {
            let (first, last) = rucksack.split_at(rucksack.len()/2);
            let first = first.chars();
            for item in first {
                if last.contains(item) {
                    //println!("{item}");
                    sump1 += priorities[&item];
                    break;
                }

            }
        }
        let groups: Vec<&str> = input.split("\n").collect();
        //println!("{:?}", groups);
        for j in 0..=groups.len() {
            if j != 0 && j % 3 == 0 {
                let group = [groups[j-3], groups[j-2], groups[j-1]];
                //println!("{:?}", group);
                for item in &items {
                    if group[0].contains(*item) && group[1].contains(*item) && group[2].contains(*item) {
                        sump2 += priorities[&item];
                        //println!("Group: {:?}, Item: {}", group, item);
                        break;
                    }
                }
            }
        }
    }   
    println!("Part1 result: {}",sump1);
    println!("Part1 result: {}",sump2);
}
