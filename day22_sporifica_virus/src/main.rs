use std::collections::HashSet;
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
    let mut grid = HashSet::new();
    for y in 0..size {
        for x in 0..size {
            if input[y][x] == '#' {
                grid.insert(((x as i32) - origin, (y as i32) - origin));
            }
        }
    }

    let mut virus = Virus::new();
    for _ in 0..10_000 {
        virus.burst(&mut grid);
    }
    println!("{}", virus.num_infections);
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

    fn burst(&mut self, grid: &mut HashSet<(i32, i32)>) {
        let is_cur_cell_infected = grid.get(&self.pos).is_some();
        if is_cur_cell_infected {
            self.dir = self.dir.turn_right();
            grid.remove(&self.pos);
        } else {
            self.dir = self.dir.turn_left();
            grid.insert(self.pos);
            self.num_infections += 1;
        }

        match self.dir {
            Direction::Up => self.pos = (self.pos.0, self.pos.1 - 1),
            Direction::Down => self.pos = (self.pos.0, self.pos.1 + 1),
            Direction::Left => self.pos = (self.pos.0 - 1, self.pos.1),
            Direction::Right => self.pos = (self.pos.0 + 1, self.pos.1),
        };
    }
}
