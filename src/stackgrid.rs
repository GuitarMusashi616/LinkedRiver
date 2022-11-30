#[derive(Debug)]
pub struct Grid<const N: usize, const M: usize> {
    data: [[u8; M]; N],
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Self {
        Self {
            data: [[0; M]; N],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_grid() {
        let grid = Grid::<5, 4>::new();
        dbg!(grid);
    }
}
