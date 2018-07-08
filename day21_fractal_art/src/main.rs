use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    // Parse rules
    let rules: HashMap<String, String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("=>", "");
            let mut parts_iter = line.split_whitespace().map(|s| s.to_string());
            let key = parts_iter.next().unwrap();
            let value = parts_iter.next().unwrap();
            (key, value)
        })
        .collect();

    // First 5 iterations
    let mut grid = Grid::new();
    println!("{}\n", grid.to_pretty_string());
    for _ in 0..5 {
        let subgrids: Vec<Grid> = grid.partition()
            .into_iter()
            .map(|subgrid| subgrid.apply_rules(&rules))
            .collect();
        grid = Grid::recombine(&subgrids);
        println!("{}\n", grid.to_pretty_string());
    }
    println!(
        "Pattern after 5 iterations contains {} enabled pixels",
        grid.cells.iter().filter(|&&c| c == '#').count()
    );
}

#[derive(Clone)]
struct Grid {
    size: usize,
    cells: Vec<char>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            size: 3,
            cells: vec!['.', '#', '.', '.', '.', '#', '#', '#', '#'],
        }
    }

    fn new_empty(size: usize) -> Grid {
        Grid {
            size,
            cells: vec![],
        }
    }

    fn from_string(string: &str) -> Grid {
        let size = string.split('/').count();
        let mut result = Grid::new_empty(size);
        result.cells = string.chars().filter(|&c| c != '/').collect();
        result
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for chunk in self.cells.chunks(self.size) {
            result += &chunk.iter().cloned().collect::<String>();
            result += "/";
        }
        result.pop(); // Remove last superflous '/'
        result
    }

    fn to_pretty_string(&self) -> String {
        let mut result = String::new();
        for chunk in self.cells.chunks(self.size) {
            result += &chunk.iter().cloned().collect::<String>();
            result += "\n";
        }
        result.pop(); // Remove last superflous '/'
        result
    }

    fn get_value(&self, x: usize, y: usize) -> char {
        self.cells[x + y * self.size]
    }

    fn set_value(&mut self, x: usize, y: usize, value: char) {
        self.cells[x + y * self.size] = value;
    }

    /// Converts a coordinate of a given subgrid to the coordinate space of its parent grid
    fn subgrid_coord_to_grid_coord(
        grid_size: usize,
        subgrid_size: usize,
        subgrid_id: usize,
        x: usize,
        y: usize,
    ) -> (usize, usize) {
        assert!(grid_size % subgrid_size == 0);
        let num_grids_per_side = grid_size / subgrid_size;
        let subgrid_x = subgrid_id % num_grids_per_side;
        let subgrid_y = subgrid_id / num_grids_per_side;
        let subgrid_origin_x = subgrid_x * subgrid_size;
        let subgrid_origin_y = subgrid_y * subgrid_size;
        (subgrid_origin_x + x, subgrid_origin_y + y)
    }

    /// Returns the subgrid with of given id and size.
    /// The subgrid ids are arranged by row starting from the left i.e.:
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    fn subgrid(&self, subgrid_id: usize, subgrid_size: usize) -> Grid {
        assert!(self.size % subgrid_size == 0);
        let mut subgrid = Grid::new_empty(subgrid_size);
        for subgrid_y in 0..subgrid_size {
            for subgrid_x in 0..subgrid_size {
                let (grid_x, grid_y) = Grid::subgrid_coord_to_grid_coord(
                    self.size,
                    subgrid_size,
                    subgrid_id,
                    subgrid_x,
                    subgrid_y,
                );
                subgrid.cells.push(self.get_value(grid_x, grid_y));
            }
        }
        subgrid
    }

    /// Partitions a grid into subgrids of size 2 or 3 depending on wether the grids size is
    /// divisible by 2 or 3.
    fn partition(self) -> Vec<Grid> {
        let subgrid_size = if self.size % 2 == 0 { 2 } else { 3 };
        let num_subgrids = (self.size / subgrid_size).pow(2);
        (0..num_subgrids)
            .map(|subgrid_id| self.subgrid(subgrid_id, subgrid_size))
            .collect()
    }

    /// Recombines a list of subgrids into one big grid
    fn recombine(subgrids: &[Grid]) -> Grid {
        let subgrid_size = subgrids[0].size;
        let num_cells_per_subgrid = subgrid_size * subgrid_size;
        let num_subgrids = subgrids.len();
        let num_cells = num_cells_per_subgrid * num_subgrids;

        let mut grid = Grid::new_empty((num_cells as f64).sqrt().round() as usize);
        grid.cells = vec!['.'; num_cells];

        // Copy the subgrid-cells to the grid
        for (subgrid_id, subgrid) in subgrids.iter().enumerate() {
            for subgrid_y in 0..subgrid_size {
                for subgrid_x in 0..subgrid_size {
                    let (grid_x, grid_y) = Grid::subgrid_coord_to_grid_coord(
                        grid.size,
                        subgrid_size,
                        subgrid_id,
                        subgrid_x,
                        subgrid_y,
                    );
                    let subgrid_value = subgrid.get_value(subgrid_x, subgrid_y);
                    grid.set_value(grid_x, grid_y, subgrid_value);
                }
            }
        }
        grid
    }

    fn flipped_horizontal(&self) -> Grid {
        let mut result = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                result.set_value(x, y, self.get_value((self.size - 1) - x, y));
            }
        }
        result
    }

    fn flipped_vertical(&self) -> Grid {
        let mut result = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                result.set_value(x, y, self.get_value(x, (self.size - 1) - y));
            }
        }
        result
    }

    fn rotated_clockwise(&self) -> Grid {
        let mut result = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                result.set_value((self.size - 1) - y, x, self.get_value(x, y));
            }
        }
        result
    }

    fn apply_rules(self, rules: &HashMap<String, String>) -> Grid {
        let mut clone = self.clone();
        for _ in 0..3 {
            // Regular
            if let Some(result) = rules.get(&clone.to_string()) {
                return Grid::from_string(result);
            }

            // Flipped once
            if let Some(result) = rules.get(&clone.flipped_horizontal().to_string()) {
                return Grid::from_string(result);
            }
            if let Some(result) = rules.get(&clone.flipped_vertical().to_string()) {
                return Grid::from_string(result);
            }

            // Flipped twice
            if let Some(result) =
                rules.get(&clone.flipped_vertical().flipped_horizontal().to_string())
            {
                return Grid::from_string(result);
            }
            if let Some(result) =
                rules.get(&clone.flipped_horizontal().flipped_vertical().to_string())
            {
                return Grid::from_string(result);
            }

            clone = clone.rotated_clockwise();
        }
        panic!("No pattern found for:\n{}\n", self.to_pretty_string());
    }
}
