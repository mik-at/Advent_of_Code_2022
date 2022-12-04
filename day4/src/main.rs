fn extract_area(area: &str) -> (i32, i32) {
    let mut area = area.split("-");
    let (area_start, area_end) = (area.next().unwrap().parse::<i32>().unwrap(), area.next().unwrap().parse::<i32>().unwrap());
    return (area_start, area_end);
}

fn main() {
        let mut sum = 0;
        //if let Ok(input) = std::fs::read_to_string("./input_sample.txt") {
        if let Ok(input) = std::fs::read_to_string("./input.txt") {
            let pairs = input.split("\n");
            for pair in pairs {
                let mut pair = pair.split(",");
                let (first, last) = (pair.next().unwrap(), pair.next().unwrap());
                let (first_area_start, first_area_end) = extract_area(first);
                let (last_area_start, last_area_end) = extract_area(last);
                if (first_area_start <= last_area_start && first_area_end >= last_area_end) || (first_area_start >= last_area_start && first_area_end <= last_area_end) { //overlap found
                    sum += 1;
                }
            }
        }
        println!("Part1: {}", sum);
}