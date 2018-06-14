use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum MoveDir {
    Right { steps_remaining: i32 },
    Up { steps_remaining: i32 },
    Left { steps_remaining: i32 },
    Down { steps_remaining: i32 },
}

struct MemoryWalker {
    pos: Point,
    cur_direction: MoveDir,
    max_steps_right: i32,
    max_steps_up: i32,
    max_steps_left: i32,
    max_steps_down: i32,
}

const INPUT: i32 = 277678;

fn main() {
    part1();
    part2();
}

fn part1() {
    // NOTE: As the first square has a value of 1 instead of 0 we need to substract one
    let num_steps_left = INPUT - 1;
    let mut walker = MemoryWalker::new();
    for _ in 1..num_steps_left {
        walker.change_dir_if_necessary();
        walker.do_step();
    }
    println!(
        "When walking {} steps the manhattan distance from square one is: {} with terminal {:?}",
        INPUT,
        manhattan_distance_from_origin(&walker.pos),
        walker.pos
    );
}

fn part2() {
    let mut memory_grid = HashMap::new();
    memory_grid.insert((0, 0), 1);
    let mut walker = MemoryWalker::new();
    loop {
        walker.change_dir_if_necessary();
        walker.do_step();

        // Add up all adjacent values
        let (x, y) = (walker.pos.x, walker.pos.y);
        let mut value_to_write = 0;
        for &(dx, dy) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ].iter()
        {
            if let Some(val) = memory_grid.get(&(x + dx, y + dy)) {
                value_to_write += val;
            }
        }

        // Write out the value
        memory_grid.insert((x, y), value_to_write);
        if value_to_write > INPUT {
            println!(
                "The first value that is larger than our puzzle input {} is: {} at {:?}",
                INPUT, value_to_write, walker.pos
            );
            break;
        }
    }
}

fn manhattan_distance_from_origin(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

impl MemoryWalker {
    /// Creates a MemoryWalker at the origin facing to the right
    fn new() -> MemoryWalker {
        MemoryWalker {
            pos: Point { x: 0, y: 0 },
            max_steps_right: 1,
            max_steps_up: 1,
            max_steps_left: 2,
            max_steps_down: 2,
            cur_direction: MoveDir::Right { steps_remaining: 1 },
        }
    }

    /// If we do not have any steps left to go in on direction,
    /// we switch directions counter-clockwise (right -> up -> left -> down -> right).
    fn change_dir_if_necessary(&mut self) {
        match self.cur_direction {
            MoveDir::Right { steps_remaining: 0 } => {
                self.cur_direction = MoveDir::Up {
                    steps_remaining: self.max_steps_up,
                }
            }
            MoveDir::Up { steps_remaining: 0 } => {
                self.cur_direction = MoveDir::Left {
                    steps_remaining: self.max_steps_left,
                }
            }
            MoveDir::Left { steps_remaining: 0 } => {
                self.cur_direction = MoveDir::Down {
                    steps_remaining: self.max_steps_down,
                }
            }
            MoveDir::Down { steps_remaining: 0 } => {
                // After each spiral ring we need to move a little bit farther in each direction
                self.max_steps_right += 2;
                self.max_steps_up += 2;
                self.max_steps_left += 2;
                self.max_steps_down += 2;

                self.cur_direction = MoveDir::Right {
                    steps_remaining: self.max_steps_right,
                };
            }
            _ => {}
        };
    }

    fn do_step(&mut self) {
        match self.cur_direction {
            MoveDir::Right { steps_remaining: n } => {
                self.cur_direction = MoveDir::Right {
                    steps_remaining: n - 1,
                };
                self.pos = Point {
                    x: self.pos.x + 1,
                    y: self.pos.y,
                };
            }
            MoveDir::Up { steps_remaining: n } => {
                self.cur_direction = MoveDir::Up {
                    steps_remaining: n - 1,
                };
                self.pos = Point {
                    x: self.pos.x,
                    y: self.pos.y + 1,
                };
            }
            MoveDir::Left { steps_remaining: n } => {
                self.cur_direction = MoveDir::Left {
                    steps_remaining: n - 1,
                };
                self.pos = Point {
                    x: self.pos.x - 1,
                    y: self.pos.y,
                };
            }
            MoveDir::Down { steps_remaining: n } => {
                self.cur_direction = MoveDir::Down {
                    steps_remaining: n - 1,
                };
                self.pos = Point {
                    x: self.pos.x,
                    y: self.pos.y - 1,
                };
            }
        };
    }
}
