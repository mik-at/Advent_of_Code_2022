use grid::*;

fn move_tail(pos_h: (usize, usize), pos_t: (usize, usize)) -> (usize, usize) {
    let mut pos_t = pos_t;
    if pos_t.0 > 0 && pos_h.0 < pos_t.0 - 1 {
        pos_t.0 -= 1;
        if pos_h.1 != pos_t.1 {
            if pos_h.1 > pos_t.1 {
                pos_t.1 += 1;
            } else {
                pos_t.1 -= 1;
            }
        }
    }
    if pos_h.0 > pos_t.0 + 1 {
        pos_t.0 += 1;
        if pos_h.1 != pos_t.1 {
            if pos_h.1 > pos_t.1 {
                pos_t.1 += 1;
            } else {
                pos_t.1 -= 1;
            }
        }
    }
    if pos_h.1 > pos_t.1 + 1 {
        pos_t.1 += 1;
        if pos_h.0 != pos_t.0 {
            if pos_h.0 > pos_t.0 {
                pos_t.0 += 1;
            } else {
                pos_t.0 -= 1;
            }
        }
    }
    if pos_t.1 > 0 && pos_h.1 < pos_t.1 - 1 {
        pos_t.1 -= 1;
        if pos_h.0 != pos_t.0 {
            if pos_h.0 > pos_t.0 {
                pos_t.0 += 1;
            } else {
                pos_t.0 -= 1;
            }
        }
    }

    pos_t
}

fn part1(file_path: &str) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let mut grid = grid!(["."]);
        let mut pos_h = (0, 0);
        let mut pos_t = (0, 0);
        for h_move in input.lines() {
            let (direction, count) = h_move.split_once(' ').unwrap();
            let count: i32 = count.parse().unwrap();
            for movements in 0..=count {
                if movements == count {
                    break;
                }
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
                pos_t = move_tail(pos_h, pos_t);
                grid[pos_t.0][pos_t.1] = "#";
            }
        }
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if grid[row][col] == "#" {
                    result += 1;
                }
            }
        }
    }
    println!("Part1: {}", result);
    result
}

fn part2(file_path: &str) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let mut grid = grid!(["."]);
        let mut pos_h = (0, 0);
        let mut pos_t1 = (0, 0);
        let mut pos_t2 = (0, 0);
        let mut pos_t3 = (0, 0);
        let mut pos_t4 = (0, 0);
        let mut pos_t5 = (0, 0);
        let mut pos_t6 = (0, 0);
        let mut pos_t7 = (0, 0);
        let mut pos_t8 = (0, 0);
        let mut pos_t9 = (0, 0);
        for h_move in input.lines() {
            let (direction, count) = h_move.split_once(' ').unwrap();
            let count: i32 = count.parse().unwrap();
            for movements in 0..=count {
                if movements == count {
                    break;
                }
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
                            pos_t1.1 += 1;
                            pos_t2.1 += 1;
                            pos_t3.1 += 1;
                            pos_t4.1 += 1;
                            pos_t5.1 += 1;
                            pos_t6.1 += 1;
                            pos_t7.1 += 1;
                            pos_t8.1 += 1;
                            pos_t9.1 += 1;
                        } else {
                            pos_h.1 -= 1;
                        }
                    }
                    "U" => {
                        if pos_h.0 == 0 {
                            grid.insert_row(0, vec!["."; grid.cols()]);
                            pos_t1.0 += 1;
                            pos_t2.0 += 1;
                            pos_t3.0 += 1;
                            pos_t4.0 += 1;
                            pos_t5.0 += 1;
                            pos_t6.0 += 1;
                            pos_t7.0 += 1;
                            pos_t8.0 += 1;
                            pos_t9.0 += 1;
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
                pos_t1 = move_tail(pos_h, pos_t1);
                pos_t2 = move_tail(pos_t1, pos_t2);
                pos_t3 = move_tail(pos_t2, pos_t3);
                pos_t4 = move_tail(pos_t3, pos_t4);
                pos_t5 = move_tail(pos_t4, pos_t5);
                pos_t6 = move_tail(pos_t5, pos_t6);
                pos_t7 = move_tail(pos_t6, pos_t7);
                pos_t8 = move_tail(pos_t7, pos_t8);
                pos_t9 = move_tail(pos_t8, pos_t9);
                grid[pos_t9.0][pos_t9.1] = "#";
            }
        }
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if grid[row][col] == "#" {
                    result += 1;
                }
            }
        }
    }
    println!("Part2: {}", result);
    result
}

fn main() {
    part1(INPUT_FILE);
    part2(INPUT_FILE);
}
const INPUT_FILE: &str = "./input";
#[cfg(test)]
mod tests {
    const INPUT_TEST_FILE: &str = "./input_sample";
    use crate::{part1, part2};

    #[test]
    fn part_1_sample_input() {
        assert_eq!(part1(INPUT_TEST_FILE), 12);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 36);
    }
}
