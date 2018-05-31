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
    num_steps_left: i32,
    cur_direction: MoveDir,
    max_steps_right: i32,
    max_steps_up: i32,
    max_steps_left: i32,
    max_steps_down: i32,
}

const INPUT: i32 = 277678;

fn main() {
    let mut walker = MemoryWalker {
        pos: Point { x: 0, y: 0 },
        num_steps_left: INPUT - 1, // Steps begin at 1
        max_steps_right: 1,
        max_steps_up: 1,
        max_steps_left: 2,
        max_steps_down: 2,
        cur_direction: MoveDir::Right { steps_remaining: 1 },
    };

    while walker.num_steps_left > 0 {
        walker.change_dir_if_necessary();
        walker.do_step();
    }

    println!("{:?}", walker.pos);
    println!("{}", manhattan_distance_from_origin(walker.pos));
}

fn manhattan_distance_from_origin(point: Point) -> i32 {
    point.x.abs() + point.y.abs()
}

impl MemoryWalker {
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
                    steps_remaining: self.max_steps_down,
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
        self.num_steps_left -= 1;
    }
}
