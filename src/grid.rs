use std::{fmt, sync::Arc};
use rand::Rng;

use crate::path::Path;

#[derive(Debug)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(rows: u8, cols: u8) -> Self {
        Self {
            // data: vec![vec!['⬜'; cols as usize]; rows as usize],
            data: vec![vec!['⬜'; cols as usize]; rows as usize],
        }
    }

    pub fn set_coords(mut self, values: Vec<(u8, u8)>) -> Self {
        for (row, col) in values {
            self.data[row as usize][col as usize] = '🔵';
        }
        self
    }

    fn get_height_width(values: &[(u8, u8)]) -> [u8; 2] {
        values.iter().fold([0, 0], |mut accum, item| {
            if item.0 >= accum[0] {
                accum[0] = item.0 + 1;
            }
            if item.1 >= accum[1] {
                accum[1] = item.1 + 1;
            }
            accum
        })
    }
}

impl From<Vec<(u8, u8)>> for Grid {
    fn from(values: Vec<(u8, u8)>) -> Grid {
        let [height, width] = Grid::get_height_width(&values);
        Grid::new(height, width).set_coords(values)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row in &self.data {
            for col in row {
                output.push(*col);
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_grid() {
        let grid = Grid::new(5, 10);
        dbg!(grid);
    }
    
    #[test]
    fn test_get_height_width() {
        let coords = vec![(0, 0), (0, 1), (0, 2), (1, 2)];
        let [height, width] = Grid::get_height_width(&coords);
        dbg!(height);
        dbg!(width);
    }

    #[test]
    fn test_from_coords() {
        let mut coords = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            coords.push((rng.gen_range(0..5), rng.gen_range(0..10)))
        }
        let grid = Grid::from(coords);
        // dbg!(grid);
        println!("{}", grid);
    }
}
