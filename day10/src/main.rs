use grid::*;

fn add_result_part1(cycle: i32, x: i32) -> i32 {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        return cycle * x;
    } else {
        return 0;
    }
}
fn part1() {
    let mut result = 0;
    let mut x = 1;
    let mut cycle = 1;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for command in input.lines() {
            if command == "noop" {
                cycle += 1; // do nothing
            } else if command.starts_with("addx") {
                cycle += 1;
                result += add_result_part1(cycle, x);
                let add_x: i32 = command
                    .split(" ")
                    .into_iter()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                x += add_x;
                cycle += 1;
            }
            result += add_result_part1(cycle, x);
        }
    }
    println!("Part1: {}", result);
}

fn pixel_math(cycle: i32, x: i32) -> (bool, usize, usize) {
    let pixel_pos = cycle - 1;
    let col = pixel_pos % 40;
    let row = ((pixel_pos - col) / 40) % 6;
    let mut draw_pixel = false;
    let x = x % 40;
    if col == x - 1 || col == x || col == x + 1 {
        draw_pixel = true;
    }
    return (draw_pixel, row.try_into().unwrap(), col.try_into().unwrap());
}

fn part2() {
    let mut x = 1;
    let mut cycle = 1;
    let mut screen: Grid<&str> = Grid::new(6, 40);
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        for command in input.lines() {
            let (draw_pixel, row, col) = pixel_math(cycle, x);
            if draw_pixel {
                screen[row][col] = "#";
            } else {
                screen[row][col] = " ";
            }
            if command == "noop" {
                cycle += 1; // do nothing
            } else if command.starts_with("addx") {
                cycle += 1;
                let (draw_pixel, row, col) = pixel_math(cycle, x);
                if draw_pixel {
                    screen[row][col] = "#";
                } else {
                    screen[row][col] = " ";
                }
                let add_x: i32 = command
                    .split(" ")
                    .into_iter()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                x += add_x;
                cycle += 1;
            }
        }
    }
    println!("Part2:");
    for row in 0..screen.rows() {
        for col in 0..screen.cols() {
            print!("{}", screen[row][col]);
        }
        println!("");
    }
}

fn main() {
    part1();
    part2();
}
