use grid::*;

fn part1() {
    let mut result = 0;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        let mut grid = grid!(["."]);
        let mut pos_h = (0, 0);
        let mut pos_t = (0, 0);
        for h_move in input.lines() {
            let (direction, count) = h_move.split_once(" ").unwrap();
            let count: i32 = count.parse().unwrap();
            for movements in 0..=count {
                if movements == count {
                    break;
                }
                grid[pos_t.0][pos_t.1] = "#";
                match direction {
                    "R" => {
                        pos_h.1 += 1;
                        if pos_h.1 == grid.cols() {
                            grid.push_col(vec!["."; grid.rows()]);
                        }
                    }
                    "L" => {
                        if pos_h.1 == 0 {
                            grid.insert_col(0, vec!["."; grid.rows()]);
                            pos_t.1 += 1;
                        } else {
                            pos_h.1 -= 1;
                        }
                    }
                    "U" => {
                        if pos_h.0 == 0 {
                            grid.insert_row(0, vec!["."; grid.cols()]);
                            pos_t.0 += 1;
                        } else {
                            pos_h.0 -= 1;
                        }
                    }
                    "D" => {
                        pos_h.0 += 1;
                        if pos_h.0 == grid.rows() {
                            grid.push_row(vec!["."; grid.cols()]);
                        }
                    }
                    _ => (),
                };
                if pos_h.0 > pos_t.0 + 1 {
                    pos_t.0 += 1;
                    if pos_h.1 != pos_t.1 {
                        pos_t.1 = pos_h.1;
                    }
                }
                if pos_h.1 > pos_t.1 + 1 {
                    pos_t.1 += 1;
                    if pos_h.0 != pos_t.0 {
                        pos_t.0 = pos_h.0;
                    }
                }
                if pos_t.0 > 0 && pos_h.0 < pos_t.0 - 1 {
                    pos_t.0 -= 1;
                    if pos_h.1 != pos_t.1 {
                        pos_t.1 = pos_h.1;
                    }
                }
                if pos_t.1 > 0 && pos_h.1 < pos_t.1 - 1 {
                    pos_t.1 -= 1;
                    if pos_h.0 != pos_t.0 {
                        pos_t.0 = pos_h.0;
                    }
                }
            }
            grid[pos_t.0][pos_t.1] = "#";
        }
        /*dbg!(grid);
        println!("    [ \".\", \".\", \"#\", \"#\", \".\", \".\"]");
        println!("    [ \".\", \".\", \".\", \"#\", \"#\", \".\"]");
        println!("    [ \".\", \"#\", \"#\", \"#\", \"#\", \".\"]");
        println!("    [ \".\", \".\", \".\", \".\", \"#\", \".\"]");
        println!("    [ \"#\", \"#\", \"#\", \"#\", \".\", \".\"]");*/
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if grid[row][col] == "#" {
                    result += 1;
                }
            }
        }
    }
    println!("Part1: {}", result);
}
fn part2() {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string("./input_sample") {
        //if let Ok(input) = std::fs::read_to_string("./input") {
        // Code here
    }
    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}
