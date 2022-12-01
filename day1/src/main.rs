fn main() {
    let mut sum_calories: i32 = 0;
    let mut calories_vector: Vec<i32> = vec![];
    if let Ok(input) = std::fs::read_to_string("./input.txt") {
        let lines = input.split("\n");
        for line in lines {
            if line != "" {
                let newcalories: i32 = line.parse().unwrap();
                sum_calories += newcalories;
            } else {
                calories_vector.insert(calories_vector.len(), sum_calories);
                //println!("{}", sum_calories);
                sum_calories = 0;
            }
        }
    }

        calories_vector.sort();
        println!("Highest calories: {}", calories_vector[calories_vector.len()-1]);
        println!("Top3: {} {} {}", calories_vector[calories_vector.len()-1], calories_vector[calories_vector.len()-2], calories_vector[calories_vector.len()-3]);
        let added_up = calories_vector[calories_vector.len()-1] + calories_vector[calories_vector.len()-2] + calories_vector[calories_vector.len()-3];
        println!("Top3 together: {}", added_up);
}