use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum_calories: i32 = 0;
    let mut calories_vector: Vec<i32> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(scalories) = line {
                if scalories != "" {
                    let icalories: i32 = scalories.parse().unwrap();
                    sum_calories += icalories;
                } else {
                    calories_vector.insert(calories_vector.len(), sum_calories);
                    println!("{}", sum_calories);
                    sum_calories = 0;
                }
            }
        }
    }
        calories_vector.sort();
        println!("Highes Calories: {}", calories_vector[calories_vector.len()-1]);
        println!("Top3: {} {} {}", calories_vector[calories_vector.len()-1], calories_vector[calories_vector.len()-2], calories_vector[calories_vector.len()-3]);
        let added_up = calories_vector[calories_vector.len()-1] + calories_vector[calories_vector.len()-2] + calories_vector[calories_vector.len()-3];
        println!("Top3 together: {}", added_up)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}