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
            .split(' ')
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|string| string.starts_with('x') || string.starts_with('y'))
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
    sensors
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
            'S'
        } else if self.beacon_x == x && self.beacon_y == y {
            'B'
        } else if calculate_distance(vec![x, y], vec![self.x, self.y]) <= self.range {
            '#'
        } else {
            '.'
        }
    }
    fn field_empty(self, x: i32, y: i32) -> bool {
        (self.x == x && self.y == y)
            || self.beacon_x == x && self.beacon_y == y
            || calculate_distance(vec![x, y], vec![self.x, self.y]) <= self.range
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
    ranged_sensors
}

fn get_max_x(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.iter().map(|s| s.x + s.range).max().unwrap()
}
fn get_min_x(sensors: &Vec<RangedSensor>) -> i32 {
    sensors.iter().map(|s| s.x - s.range).min().unwrap()
}

fn get_y_values(sensors: Vec<RangedSensor>, y: i32) -> Vec<char> {
    let x_max = get_max_x(&sensors);
    let x_min = get_min_x(&sensors);
    let mut row: Vec<char> = Vec::new();
    for x in x_min..=x_max {
        let mut row_char: char = '.';
        for sensor in &sensors {
            row_char = sensor.get_map_char(x, y);
            match row_char {
                'S' | '#' | 'B' => {
                    break;
                }
                _ => continue,
            }
        }
        row.push(row_char);
    }
    row
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
    result
}

fn calculate_boundaries(sensors: Vec<RangedSensor>) -> Vec<(i32, i32)> {
    let mut sensor_boundaries: Vec<(i32, i32)> = Vec::new();
    for sensor in sensors {
        //sensor borders
        let left: (i32, i32) = (sensor.x - sensor.range, sensor.y);
        let up: (i32, i32) = (sensor.x, sensor.y - sensor.range);
        let right: (i32, i32) = (sensor.x + sensor.range, sensor.y);
        let down: (i32, i32) = (sensor.x, sensor.y + sensor.range);
        // walk other fields
        sensor_boundaries.push(left);
        let (mut x, mut y) = left;
        while (x, y) != up {
            x += 1;
            y -= 1;
            sensor_boundaries.push((x, y));
        }
        sensor_boundaries.push(up);
        let (mut x, mut y) = up;
        while (x, y) != right {
            x += 1;
            y += 1;
            sensor_boundaries.push((x, y));
        }
        sensor_boundaries.push(right);
        let (mut x, mut y) = right;
        while (x, y) != down {
            x -= 1;
            y += 1;
            sensor_boundaries.push((x, y));
        }
        sensor_boundaries.push(down);
        let (mut x, mut y) = down;
        while (x, y) != left {
            x -= 1;
            y -= 1;
            sensor_boundaries.push((x, y));
        }
    }
    sensor_boundaries
}

fn find_hole(
    sensor_boundaries: Vec<(i32, i32)>,
    search_radius: i32,
    sensors: Vec<RangedSensor>,
) -> (i32, i32) {
    for pos in &sensor_boundaries {
        let x = pos.0 + 1;
        let y = pos.1;
        if x > 0 && y > 0 && x < search_radius && y < search_radius {
            let mut empty = false;
            for sensor in &sensors {
                if sensor.field_empty(x, y) {
                    empty = true;
                    break;
                }
            }
            if !empty {
                return (x, y);
            }
        }
        let x = pos.0 - 1;
        let y = pos.1;
        if x > 0 && y > 0 && x < search_radius && y < search_radius {
            let mut empty = false;
            for sensor in &sensors {
                if sensor.field_empty(x, y) {
                    empty = true;
                    break;
                }
            }
            if !empty {
                return (x, y);
            }
        }
        let x = pos.0;
        let y = pos.1 - 1;
        if x > 0 && y > 0 && x < search_radius && y < search_radius {
            let mut empty = false;
            for sensor in &sensors {
                if sensor.field_empty(x, y) {
                    empty = true;
                    break;
                }
            }
            if !empty {
                return (x, y);
            }
        }
        let x = pos.0;
        let y = pos.1 + 1;
        if x > 0 && y > 0 && x < search_radius && y < search_radius {
            let mut empty = false;
            for sensor in &sensors {
                if sensor.field_empty(x, y) {
                    empty = true;
                    break;
                }
            }
            if !empty {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn part2(file_path: &str, frequency: i32, search_radius: i32) -> u128 {
    let mut result: u128 = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let sensors = parse_sensors(input);
        let sensors = calculate_ranges(sensors);
        let sensor_boundaries = calculate_boundaries(sensors.clone());
        let (x, y) = find_hole(sensor_boundaries, search_radius, sensors);
        result = x as u128 * frequency as u128 + y as u128;
    }
    println!("Part2: {}", result);
    result
}

fn main() {
    part1(INPUT_FILE, *ROW_NUM_PART1);
    part2(INPUT_FILE, *FREQUENCY_PART2, *SEARCH_RADIUS_PART2);
}

const INPUT_FILE: &str = "./input";
const ROW_NUM_PART1: &i32 = &2000000;
const FREQUENCY_PART2: &i32 = &4000000;
const SEARCH_RADIUS_PART2: &i32 = &4000000;
#[cfg(test)]
mod tests {
    const INPUT_TEST_FILE: &str = "./input_sample";
    use crate::{part1, part2};

    #[test]
    fn part_1_sample_input() {
        assert_eq!(part1(INPUT_TEST_FILE, 10), 26);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE, 4000000, 20), 56000011);
    }
}
