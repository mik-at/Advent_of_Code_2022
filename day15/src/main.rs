use smartcore::metrics::distance::manhattan::Manhattan;
use smartcore::metrics::distance::Distance;

#[derive(Debug, Default, Clone)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    fn get_location(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }
    fn get_beacon(&self) -> Vec<i32> {
        vec![self.beacon_x, self.beacon_y]
    }
}

fn parse_sensors(input: String) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.lines() {
        let mut values = line
            .split(" ")
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|string| string.starts_with("x") || string.starts_with("y"))
            .map(|value| {
                value.trim_matches(|c| c == 'x' || c == 'y' || c == '=' || c == ',' || c == ':')
            })
            .collect::<Vec<_>>();
        let sensor: Sensor = Sensor {
            beacon_y: values.pop().unwrap().parse().unwrap(),
            beacon_x: values.pop().unwrap().parse().unwrap(),
            y: values.pop().unwrap().parse().unwrap(),
            x: values.pop().unwrap().parse().unwrap(),
        };
        sensors.push(sensor);
    }
    return sensors;
}

#[derive(Debug, Default, Clone, Copy)]
struct RangedSensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
    range: i32,
}

impl RangedSensor {
    fn new(sensor: Sensor) -> RangedSensor {
        RangedSensor {
            x: sensor.x,
            y: sensor.y,
            beacon_x: sensor.beacon_x,
            beacon_y: sensor.beacon_y,
            range: calculate_distance(sensor.get_location(), sensor.get_beacon()),
        }
    }
    fn get_map_char(self, x: i32, y: i32) -> char {
        if self.x == x && self.y == y {
            return 'S';
        } else if self.beacon_x == x && self.beacon_y == y {
            return 'B';
        } else if calculate_distance(vec![x, y], vec![self.x, self.y]) <= self.range {
            return '#';
        } else {
            return '.';
        }
    }
    fn field_empty(self, x: i32, y: i32) -> bool {
        if (self.x == x && self.y == y)
            || self.beacon_x == x && self.beacon_y == y
            || calculate_distance(vec![x, y], vec![self.x, self.y]) <= self.range
        {
            return true;
        } else {
            return false;
        }
    }
}

fn calculate_distance(a: Vec<i32>, b: Vec<i32>) -> i32 {
    Manhattan::new().distance(&a, &b) as i32
}

fn calculate_ranges(sensors: Vec<Sensor>) -> Vec<RangedSensor> {
    let mut ranged_sensors: Vec<RangedSensor> = Vec::new();
    for sensor in sensors {
        ranged_sensors.push(RangedSensor::new(sensor));
    }
    return ranged_sensors;
}

fn get_max_x(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.into_iter().map(|s| s.x + s.range).max().unwrap()
}
fn get_min_x(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.into_iter().map(|s| s.x - s.range).min().unwrap()
}
fn get_max_y(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.into_iter().map(|s| s.y + s.range).max().unwrap()
}
fn get_min_y(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.into_iter().map(|s| s.y - s.range).min().unwrap()
}

fn get_y_values(sensors: Vec<RangedSensor>, y: i32) -> Vec<char> {
    let x_max = get_max_x(&sensors);
    let x_min = get_min_x(&sensors);
    let mut row: Vec<char> = Vec::new();
    for x in x_min..=x_max {
        let mut row_char: char = '.';
        for sensor in &sensors {
            match sensor.get_map_char(x, y) {
                'S' => {
                    row_char = 'S';
                    break;
                }
                '#' => {
                    row_char = '#';
                    break;
                }
                'B' => {
                    row_char = 'B';
                    break;
                }
                _ => continue,
            }
        }
        row.push(row_char);
    }
    return row;
}

fn part1(file_path: &str, row_num: i32) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let sensors = parse_sensors(input);
        let sensors = calculate_ranges(sensors);
        let row = get_y_values(sensors, row_num);
        for char in row {
            match char {
                '#' => result += 1,
                _ => continue,
            };
        }
    }
    println!("Part1: {}", result);
    return result;
}

fn find_beacon(sensors: Vec<RangedSensor>, search_radius: i32) -> (i32, i32) {
    let mut empty = false;
    let mut x_max = get_max_x(&sensors);
    let mut x_min = get_min_x(&sensors);
    let mut y_max = get_max_y(&sensors);
    let mut y = get_min_y(&sensors);
    if x_min < 0 {
        x_min = 0;
    }
    if x_max > search_radius {
        x_max = search_radius;
    }
    if y < 0 {
        y = 0;
    }
    if y_max > search_radius {
        y_max = search_radius;
    }
    for x in x_min..=x_max {
        while y <= y_max {
            for sensor in &sensors {
                if sensor.field_empty(x, y) {
                    empty = true;
                    if sensor.y > y {
                        y += (sensor.y - y) * 2;
                    }
                    break;
                }
            }
            if empty == false {
                return (x, y);
            }
            empty = false;
            y += 1;
        }
        y = 0;
        println!("{:.5}%", ((x * 100) / search_radius) as f64);
    }
    return (search_radius, search_radius);
}

fn part2(file_path: &str, frequency: i32, search_radius: i32) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let sensors = parse_sensors(input);
        let sensors = calculate_ranges(sensors);
        let (x, y) = find_beacon(sensors, search_radius);
        result = x * frequency + y;
    }
    println!("Part2: {}", result);
    return result;
}

fn main() {
    //part1(INPUT_FILE, *ROW_NUM_PART1);
    part2(INPUT_FILE, *FREQUENCY_PART2, *SEARCH_RADIUS_PART2);
}

static INPUT_FILE: &'static str = "./input";
static ROW_NUM_PART1: &'static i32 = &2000000;
static FREQUENCY_PART2: &'static i32 = &4000000;
static SEARCH_RADIUS_PART2: &'static i32 = &4000000;
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part_1_sample_input() {
        let test_input = "./input_sample";
        //part1(test_input, 10);
        assert_eq!(part1(test_input, 10), 26);
    }

    #[test]
    fn part_2_sample_input() {
        let test_input = "./input_sample";
        assert_eq!(part2(test_input, 4000000, 20), 56000011);
    }
}
