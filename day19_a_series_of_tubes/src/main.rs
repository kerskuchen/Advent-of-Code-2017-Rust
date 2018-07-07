use std::fs::File;
use std::io::*;

fn main() {
    let grid: Vec<Vec<char>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let start_x = grid[0].iter().position(|&c| c == '|').unwrap();
    let mut packet = Packet::new(start_x as i32);
    while packet.movement != Movement::Stopped {
        packet.step(&grid);
    }
    println!("Letters visited: {}", packet.letters_visited);
    println!("Number of steps taken: {}", packet.num_steps_taken);
}

#[derive(PartialEq)]
enum Movement {
    Stopped,
    Up,
    Down,
    Left,
    Right,
}

struct Packet {
    x: i32,
    y: i32,
    movement: Movement,
    letters_visited: String,
    num_steps_taken: usize,
}

impl Packet {
    fn new(start_x: i32) -> Packet {
        Packet {
            x: start_x,
            y: 0,
            movement: Movement::Down,
            letters_visited: String::new(),
            num_steps_taken: 0,
        }
    }

    fn step(&mut self, grid: &[Vec<char>]) {
        let x = self.x;
        let y = self.y;
        let cur_cell = cell_type(grid, x, y);

        // Determine new move direction based on current cell and move direction
        match cur_cell {
            CellType::Empty => self.movement = Movement::Stopped,
            CellType::Letter(c) => self.letters_visited.push(c),
            CellType::Cross => {
                // If we were moving horizontally we search for the next non-empty cell
                // vertically (and vice-versa)
                match self.movement {
                    Movement::Up | Movement::Down => {
                        if cell_type(grid, x + 1, y) != CellType::Empty {
                            self.movement = Movement::Right;
                        } else if cell_type(grid, x - 1, y) != CellType::Empty {
                            self.movement = Movement::Left;
                        } else {
                            self.movement = Movement::Stopped;
                        }
                    }
                    Movement::Left | Movement::Right => {
                        if cell_type(grid, x, y + 1) != CellType::Empty {
                            self.movement = Movement::Down;
                        } else if cell_type(grid, x, y - 1) != CellType::Empty {
                            self.movement = Movement::Up;
                        } else {
                            self.movement = Movement::Stopped;
                        }
                    }
                    Movement::Stopped => {}
                };
            }
            CellType::Line => {}
        }

        // Move to next cell
        match self.movement {
            Movement::Up => self.y -= 1,
            Movement::Down => self.y += 1,
            Movement::Left => self.x -= 1,
            Movement::Right => self.x += 1,
            Movement::Stopped => {}
        }

        if self.movement != Movement::Stopped {
            self.num_steps_taken += 1;
        }
    }
}

#[derive(PartialEq)]
enum CellType {
    Empty,
    Line,
    Cross,
    Letter(char),
}

fn cell_type(grid: &[Vec<char>], x: i32, y: i32) -> CellType {
    let cell_value = if x >= 0 && x < (grid[0].len() as i32) && y >= 0 && y < (grid.len() as i32) {
        grid[y as usize][x as usize]
    } else {
        ' '
    };

    match cell_value {
        ' ' => CellType::Empty,
        '|' | '-' => CellType::Line,
        '+' => CellType::Cross,
        c => CellType::Letter(c),
    }
}
