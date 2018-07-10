use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    // Collect cells
    let input: Vec<Vec<char>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // Create grid
    let size = input.len();
    let origin = (size / 2) as i32;
    let mut grid = HashMap::new();
    for y in 0..size {
        for x in 0..size {
            if input[y][x] == '#' {
                grid.insert(
                    ((x as i32) - origin, (y as i32) - origin),
                    CellType::Infected,
                );
            }
        }
    }
    let grid_original = grid;

    // Part one
    let mut grid = grid_original.clone();
    let mut virus = Virus::new();
    for _ in 0..10_000 {
        virus.burst(&mut grid);
    }
    println!(
        "Number of cells infected by the virus: {}",
        virus.num_infections
    );

    // Part one
    let mut grid = grid_original;
    let mut virus = Virus::new();
    for _ in 0..10_000_000 {
        virus.burst_evolved(&mut grid);
    }
    println!(
        "Number of cells infected by the evolved virus: {}",
        virus.num_infections
    );
}

#[derive(Copy, Clone)]
enum CellType {
    // NOTE: Clean is implicit (missing in grid)
    Weakened,
    Infected,
    Flagged,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Virus {
    pos: (i32, i32),
    dir: Direction,
    num_infections: usize,
}

impl Virus {
    fn new() -> Virus {
        Virus {
            pos: (0, 0),
            dir: Direction::Up,
            num_infections: 0,
        }
    }

    fn burst(&mut self, grid: &mut HashMap<(i32, i32), CellType>) {
        let is_cur_cell_infected = grid.get(&self.pos).is_some();
        if is_cur_cell_infected {
            grid.remove(&self.pos);
            self.dir = self.dir.turn_right();
        } else {
            grid.insert(self.pos, CellType::Infected);
            self.dir = self.dir.turn_left();
            self.num_infections += 1;
        }

        match self.dir {
            Direction::Up => self.pos = (self.pos.0, self.pos.1 - 1),
            Direction::Down => self.pos = (self.pos.0, self.pos.1 + 1),
            Direction::Left => self.pos = (self.pos.0 - 1, self.pos.1),
            Direction::Right => self.pos = (self.pos.0 + 1, self.pos.1),
        };
    }

    fn burst_evolved(&mut self, grid: &mut HashMap<(i32, i32), CellType>) {
        match grid.get(&self.pos) {
            None => {
                grid.insert(self.pos, CellType::Weakened);
                self.dir = self.dir.turn_left();
            }
            Some(CellType::Weakened) => {
                grid.insert(self.pos, CellType::Infected);
                self.num_infections += 1;
            }
            Some(CellType::Infected) => {
                grid.insert(self.pos, CellType::Flagged);
                self.dir = self.dir.turn_right();
            }
            Some(CellType::Flagged) => {
                grid.remove(&self.pos);
                self.dir = self.dir.reverse();
            }
        }

        match self.dir {
            Direction::Up => self.pos = (self.pos.0, self.pos.1 - 1),
            Direction::Down => self.pos = (self.pos.0, self.pos.1 + 1),
            Direction::Left => self.pos = (self.pos.0 - 1, self.pos.1),
            Direction::Right => self.pos = (self.pos.0 + 1, self.pos.1),
        };
    }
}
