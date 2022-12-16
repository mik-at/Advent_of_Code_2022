use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    lead_to: Vec<String>,
}

fn parse_valves(input: String) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in input.lines() {
        let (str1, str2) = line.split_once("; ").unwrap();
        let mut str1 = str1.split_whitespace();
        let valve_name = str1.nth(1).unwrap().to_owned();
        let flow_rate: usize = str1.nth(2).unwrap()["rate=".len()..].parse().unwrap();
        let lead_to: Vec<String> = str2["tunnel leads to valve ".len()..]
            .split(", ")
            .map(|x| x.trim())
            .map(str::to_string)
            .collect();
        valves.insert(
            valve_name.clone(),
            Valve {
                name: valve_name,
                flow_rate,
                lead_to,
            },
        );
    }
    valves
}

fn simulate_trip(
    mut valves: HashMap<String, Valve>,
    start_with: &str,
    max_minutes: usize,
) -> usize {
    let mut current_position = start_with;
    let mut minutes_elapsed = 0;
    let mut final_flow_rate = 1;
    let mut valves_opened: Vec<String> = Vec::new();
    while minutes_elapsed < max_minutes {
        let valve = &valves[current_position];
        if !valves_opened.contains(&valve.name) {
            valves_opened.push(valve.name.clone());
            minutes_elapsed += 1;
            if valve.flow_rate != 0 {
                final_flow_rate *= valve.flow_rate;
            }
        }
        let mut go_next = "";
        let mut next_flow_rate: usize = 0;
        for check_flow_rates in &valve.lead_to {
            let next_valve = &valves[check_flow_rates];
            if next_valve.flow_rate > next_flow_rate && !valves_opened.contains(&next_valve.name) {
                go_next = next_valve.name.as_str();
                next_flow_rate = next_valve.flow_rate;
            }
        }
        current_position = go_next;
        minutes_elapsed += 1;
        dbg!(&minutes_elapsed, &valve, &go_next);
    }
    final_flow_rate
}

fn part1(file_path: &str, start_with: &str, minutes: usize) -> usize {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let valves = parse_valves(input);
        result = simulate_trip(valves, start_with, minutes);
    }
    println!("Part1: {}", result);
    result
}
fn part2(file_path: &str) -> &str {
    let mut result = "";
    if let Ok(input) = std::fs::read_to_string(file_path) {
        // Code here
    }
    println!("Part2: {}", result);
    result
}

fn main() {
    part1(INPUT_FILE, START_WITH, MINUTES);
    part2(INPUT_FILE);
}
const INPUT_FILE: &str = "./input";
const START_WITH: &str = "AA";
const MINUTES: usize = 30;
#[cfg(test)]
mod tests {
    const INPUT_TEST_FILE: &str = "./input_sample";
    use crate::{part1, part2};

    #[test]
    fn part_1_sample_input() {
        assert_eq!(part1(INPUT_TEST_FILE, "AA", 30), 1651);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), "");
    }
}
