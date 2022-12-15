use grid::*;

fn make_cave(input: String) -> (Grid<char>, usize, usize) {
    let mut grid: Grid<char> = Grid::from_vec(vec!['.'], 1);
    let mut y_modifier: usize = usize::MAX;
    for walls in input.lines() {
        for wall_coordinates in walls.split(" -> ") {
            let (y, _) = wall_coordinates.split_once(",").unwrap();
            let y: usize = y.parse().unwrap();
            if y_modifier > y {
                y_modifier = y;
            }
        }
    }
    for walls in input.lines() {
        for wall_coordinates in walls.split(" -> ") {
            let (y, x) = wall_coordinates.split_once(",").unwrap();
            let x: usize = x.parse().unwrap();
            let mut y: usize = y.parse().unwrap();
            y -= y_modifier;
            while x + 1 > grid.rows() {
                grid.push_row(vec!['.'; grid.cols()]);
            }
            while y + 1 > grid.cols() {
                grid.push_col(vec!['.'; grid.rows()]);
            }
        }
    }
    grid.push_row(vec!['.'; grid.cols()]);
    grid.push_col(vec!['.'; grid.rows()]);
    for walls in input.lines() {
        let mut last = false;
        let mut last_x: usize = 0;
        let mut last_y: usize = 0;
        for wall_coordinates in walls.split(" -> ") {
            let (y, x) = wall_coordinates.split_once(",").unwrap();
            let x: usize = x.parse().unwrap();
            let mut y: usize = y.parse().unwrap();
            y -= y_modifier;
            grid[x][y] = '#';
            if last == false {
                last = true;
                last_x = x;
                last_y = y;
            } else {
                if x != last_x {
                    if x > last_x {
                        for u in last_x..x {
                            grid[u][y] = '#';
                        }
                    } else {
                        for u in x..last_x {
                            grid[u][y] = '#';
                        }
                    }
                } else {
                    if y > last_y {
                        for u in last_y..y {
                            grid[x][u] = '#';
                        }
                    } else {
                        for u in y..last_y {
                            grid[x][u] = '#';
                        }
                    }
                }
                last_x = x;
                last_y = y;
            }
        }
    }
    grid.insert_col(0, vec!['.'; grid.rows()]);
    let abyss = grid.rows() - 1;
    let start_col = y_modifier - 1;
    return (grid, abyss, start_col);
}

fn simulate_sand_part1(
    cave: Grid<char>,
    abyss: usize,
    start_col: usize,
    start: usize,
) -> Grid<char> {
    let mut grid = cave.clone();
    let start = start - start_col;
    let mut x: usize = 0;
    let mut y = start;
    loop {
        if x >= abyss {
            break;
        }
        if grid.get(x + 1, y).unwrap().to_owned() == '#'
            || grid.get(x + 1, y).unwrap().to_owned() == 'o'
        {
            if grid.get(x + 1, y - 1).unwrap().to_owned() == '#'
                || grid.get(x + 1, y - 1).unwrap().to_owned() == 'o'
            {
                if grid.get(x + 1, y + 1).unwrap().to_owned() == '#'
                    || grid.get(x + 1, y + 1).unwrap().to_owned() == 'o'
                {
                    if x != 0 {
                        grid[x][y] = 'o';
                        x = 0;
                        y = start;
                    } else {
                        break;
                    }
                } else {
                    y += 1;
                }
            } else {
                y -= 1;
            }
        } else {
            x += 1;
        }
    }
    return grid;
}

fn make_cave_full(input: String) -> (Grid<char>, usize) {
    let mut grid: Grid<char> = Grid::from_vec(vec!['.'], 1);
    for walls in input.lines() {
        for wall_coordinates in walls.split(" -> ") {
            let (y, x) = wall_coordinates.split_once(",").unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            while x + 1 > grid.rows() {
                grid.push_row(vec!['.'; grid.cols()]);
            }
            while y + 1 > grid.cols() {
                grid.push_col(vec!['.'; grid.rows()]);
            }
        }
    }
    grid.push_row(vec!['.'; grid.cols()]);
    grid.push_col(vec!['.'; grid.rows()]);
    for walls in input.lines() {
        let mut last = false;
        let mut last_x: usize = 0;
        let mut last_y: usize = 0;
        for wall_coordinates in walls.split(" -> ") {
            let (y, x) = wall_coordinates.split_once(",").unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            grid[x][y] = '#';
            if last == false {
                last = true;
                last_x = x;
                last_y = y;
            } else {
                if x != last_x {
                    if x > last_x {
                        for u in last_x..x {
                            grid[u][y] = '#';
                        }
                    } else {
                        for u in x..last_x {
                            grid[u][y] = '#';
                        }
                    }
                } else {
                    if y > last_y {
                        for u in last_y..y {
                            grid[x][u] = '#';
                        }
                    } else {
                        for u in y..last_y {
                            grid[x][u] = '#';
                        }
                    }
                }
                last_x = x;
                last_y = y;
            }
        }
    }
    grid.insert_col(0, vec!['.'; grid.rows()]);
    grid.push_row(vec!['#'; grid.cols()]);
    let floor = grid.rows();
    return (grid, floor);
}

fn simulate_sand_part2(cave: Grid<char>, floor: usize, start: usize) -> Grid<char> {
    let mut grid = cave.clone();
    let mut x: usize = 0;
    let mut y = start;
    loop {
        while y + 1 >= grid.cols() {
            grid.push_col(vec!['.'; grid.rows()]);
            let cols = grid.cols() - 1;
            grid[floor - 1][cols] = '#';
        }
        if x > floor {
            break;
        }
        if grid.get(0, y).unwrap().to_owned() == 'o'
            && grid.get(0, y + 1).unwrap().to_owned() == 'o'
            && grid.get(0, y - 1).unwrap().to_owned() == 'o'
        {
            break;
        }
        if grid.get(x + 1, y).unwrap().to_owned() == '#'
            || grid.get(x + 1, y).unwrap().to_owned() == 'o'
        {
            if grid.get(x + 1, y - 1).unwrap().to_owned() == '#'
                || grid.get(x + 1, y - 1).unwrap().to_owned() == 'o'
            {
                if grid.get(x + 1, y + 1).unwrap().to_owned() == '#'
                    || grid.get(x + 1, y + 1).unwrap().to_owned() == 'o'
                {
                    if x != 0 {
                        grid[x][y] = 'o';
                        x = 0;
                        y = start;
                    } else {
                        break;
                    }
                } else {
                    y += 1;
                }
            } else {
                y -= 1;
            }
        } else {
            x += 1;
        }
    }
    grid[0][start] = 'o';
    return grid;
}

fn part1(file_path: &str) -> i32 {
    let mut result = 0;
    if let Ok(input) = std::fs::read_to_string(file_path) {
        let (cave, abyss, start_col) = make_cave(input);
        let sand_cave = simulate_sand_part1(cave, abyss, start_col, 500);
        for row in 0..sand_cave.rows() {
            for col in 0..sand_cave.cols() {
                if sand_cave.get(row, col).unwrap().to_owned() == 'o' {
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
        let (cave, floor) = make_cave_full(input);
        let sand_cave = simulate_sand_part2(cave, floor, 501);
        for row in 0..sand_cave.rows() {
            for col in 0..sand_cave.cols() {
                if sand_cave.get(row, col).unwrap().to_owned() == 'o' {
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
        assert_eq!(part1(INPUT_TEST_FILE), 24);
    }

    #[test]
    fn part_2_sample_input() {
        assert_eq!(part2(INPUT_TEST_FILE), 93);
    }
}
