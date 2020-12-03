use super::{FnPuzzle, Result, Stage};

#[derive(Clone)]
pub enum Cell {
    Empty,
    Tree,
}

pub struct Grid {
    width: usize,
    height: usize,
    value: Vec<Vec<Cell>>,
}

impl Grid {
    fn get_cell(&self, mut x: usize, y: usize) -> Option<Cell> {
        x = x % self.width;
        self.value.get(y).and_then(|v| v.get(x)).map(Cell::clone)
    }
}

fn parse_inputs(inputs: &[&str]) -> Result<Grid> {
    let mut grid = Vec::new();
    let mut width = 0;

    for input in inputs {
        let mut line = Vec::new();
        for cell in input.chars() {
            if cell == '#' {
                line.push(Cell::Tree);
            } else if cell == '.' {
                line.push(Cell::Empty);
            }
        }
        grid.push(line);
        width = input.len();
    }

    Ok(Grid {
        width,
        height: inputs.len(),
        value: grid,
    })
}

fn slope(ox: usize, oy: usize, grid: &Grid) -> usize {
    let mut x = ox;
    let mut y = oy;
    let mut count = 0;

    while y < grid.height {
        if let Some(Cell::Tree) = grid.get_cell(x, y) {
            count += 1;
        }
        x += ox;
        y += oy;
    }

    count
}

#[puzzle(3, first, parse_inputs)]
pub fn first_stage(grid: Grid) -> Result<usize> {
    Ok(slope(3, 1, &grid))
}

#[puzzle(3, second, parse_inputs)]
pub fn second_stage(grid: Grid) -> Result<usize> {
    Ok(slope(1, 1, &grid)
        * slope(3, 1, &grid)
        * slope(5, 1, &grid)
        * slope(7, 1, &grid)
        * slope(1, 2, &grid))
}
