use grid::*;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(&self, map: Grid<usize>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut height = 99;
        if x < map.rows() && y < map.cols() {
            height = *map.get(x, y).unwrap();
        }
        let mut neighbours: Vec<(Pos, usize)> = Vec::new(); // create neighbours vector

        // look up
        if x > 0 {
            let x_up = x - 1;
            if x_up < map.rows() && y < map.cols() {
                let up_height = *map.get(x_up, y).unwrap();
                if up_height <= height + 1 {
                    neighbours.push((Pos(x_up, y), 1));
                }
            }
        }
        // look down
        let x_down = x + 1;
        if x_down < map.rows() && y < map.cols() {
            let down_height = *map.get(x_down, y).unwrap();
            if down_height <= height + 1 {
                neighbours.push((Pos(x_down, y), 1));
            }
        }

        //look right
        let y_right = y + 1;
        if x < map.rows() && y_right < map.cols() {
            let right_height = *map.get(x, y_right).unwrap();
            if right_height <= height + 1 {
                neighbours.push((Pos(x, y_right), 1));
            }
        }

        //look left
        if y > 0 {
            let y_left = y - 1;
            if x < map.rows() && y_left < map.cols() {
                let left_height = *map.get(x, y_left).unwrap();
                if left_height <= height + 1 {
                    neighbours.push((Pos(x, y_left), 1));
                }
            }
        }
        return neighbours;
    }
}

fn make_grid(input: String) -> (Grid<usize>, Pos, Pos) {
    let mut start = Pos(0, 0);
    let mut goal = Pos(0, 0);
    let mut grid_vec: Vec<usize> = Vec::new();
    let (mut row_length, mut row, mut col) = (usize::MAX, 0, 0);
    for char in input.chars().map(|c| c as u8) {
        match char {
            b'a'..=b'z' => grid_vec.push((char - b'a') as usize),
            b'S' => {
                grid_vec.push(0);
                start = Pos(row, col);
            }
            b'E' => {
                grid_vec.push((b'z' - b'a') as usize);
                goal = Pos(row, col);
            }
            _ => {
                if col > row_length || row_length == usize::MAX {
                    row_length = col
                }
            }
        };
        if row_length == col {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    let grid: Grid<usize> = Grid::from_vec(grid_vec, row_length);
    return (grid, start, goal);
}
fn part1() {
    let mut result = 0;
    //if let Ok(input) = std::fs::read_to_string("./input_sample") {
    if let Ok(input) = std::fs::read_to_string("./input") {
        let (map, start, goal) = make_grid(input);
        let path = dijkstra(&start, |p| p.neighbours(map.clone()), |p| *p == goal);
        let mut count = 0;
        let (steps, _) = path.unwrap();
        for _step in steps {
            count += 1;
        }
        result = count - 1;
    }
    println!("Part1: {}", result);
}
fn part2() {
    let mut result = "";
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
