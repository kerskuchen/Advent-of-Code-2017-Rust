use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Point {
    x: i32, // moving NE is positive and moving SW is negative
    y: i32, // moving SE is positive and moving NW is negative
            //
            //       \ (1,-1) /
            // (0,-1) +------+  (1,0)
            //       /        \
            //  ----+          +------
            //       \        /
            // (-1,0) +------+  (0,1)
            //       / (-1,1) \
}

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let input: Vec<_> = input.trim().split(',').collect();

    let mut max_num_steps_from_origin = 0;
    let mut pos = Point { x: 0, y: 0 };
    for direction in input {
        match direction {
            "n" => {
                pos = Point {
                    x: pos.x + 1,
                    y: pos.y - 1,
                }
            }
            "ne" => {
                pos = Point {
                    x: pos.x + 1,
                    y: pos.y,
                }
            }
            "se" => {
                pos = Point {
                    x: pos.x,
                    y: pos.y + 1,
                }
            }
            "s" => {
                pos = Point {
                    x: pos.x - 1,
                    y: pos.y + 1,
                }
            }
            "sw" => {
                pos = Point {
                    x: pos.x - 1,
                    y: pos.y,
                }
            }
            "nw" => {
                pos = Point {
                    x: pos.x,
                    y: pos.y - 1,
                }
            }
            _ => panic!("invalid direction"),
        }

        max_num_steps_from_origin =
            i32::max(max_num_steps_from_origin, calc_numsteps_from_origin(&pos));
    }

    println!(
        "Furthest number of steps away from origin while walking: {}",
        max_num_steps_from_origin
    );

    let final_num_steps_from_origin = calc_numsteps_from_origin(&pos);
    println!(
        "Number of steps away from origin after finished walking: {}",
        final_num_steps_from_origin
    );
}

fn calc_numsteps_from_origin(pos: &Point) -> i32 {
    if pos.x.signum() != pos.y.signum() {
        // NOTE: Moving simultaneously in opposite x and y directions is equivalent to moving
        //       north/south and counts as one step (i.e. (1,-1) is north and (-1,1) is south).
        //       So in this case we can go north/south first till either x or y is zero and then
        //       add the remaining steps from the non-zero coordinate. This exactly adds up to
        //       the following:
        std::cmp::max(pos.x.abs(), pos.y.abs())
    } else {
        pos.x.abs() + pos.y.abs()
    }
}
