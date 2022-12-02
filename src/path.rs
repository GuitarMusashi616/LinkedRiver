use std::sync::Arc;
use crate::pathiter::PathIter;

const ADJ: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug)]
pub struct Path {
    coord: (u8, u8),
    prev: Option<Arc<Path>>,
}

impl Path {
    pub fn new(coord: (u8, u8), prev: Option<Arc<Path>>) -> Self {
        Self {
            coord,
            prev,
        }
    }

    pub fn get_coord(&self) -> (u8, u8) {
        self.coord
    }

    pub fn get_prev(&self) -> &Option<Arc<Path>> {
        // get the arc out of the option, clone it, then wrap it back up in the option
        // if their isn't anything there then just make it a None

        &self.prev
    }

    pub fn find_neighbors(&self, size: (u8, u8)) -> Vec<(u8, u8)> {
        let mut output = Vec::new();
        for (dx, dy) in ADJ {
            let (x, y) = (self.coord.0 as i8 + dx, self.coord.1 as i8 + dy);
            if x < 0 || y < 0 || x >= size.0 as i8 || y >= size.1 as i8 {
                continue;
            }
            output.push((x as u8, y as u8));
        }
        output
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = (u8, u8);
    type IntoIter = PathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathIter::new(self.get_prev())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_new_path() {
        let path0 = Arc::new(Path::new((0,0), None));
        let path1 = Arc::new(Path::new((0,1), Some(Arc::clone(&path0))));
        let path2 = Arc::new(Path::new((1,1), Some(Arc::clone(&path1))));
        let path3 = Arc::new(Path::new((2,1), Some(Arc::clone(&path2))));

        let mut currpath = Some(&path3);
        let mut output = Vec::new();
        while let Some(path) = currpath {
            output.push(path.coord);
            currpath = path.prev.as_ref();
        }

        let expected = vec![(2, 1), (1, 1), (0, 1), (0, 0)];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_neighbors() {
        let path0 = Arc::new(Path::new((0,0), None));
        let path1 = Arc::new(Path::new((0,1), Some(Arc::clone(&path0))));
        let path2 = Arc::new(Path::new((1,1), Some(Arc::clone(&path1))));
        let path3 = Arc::new(Path::new((2,1), Some(Arc::clone(&path2))));

        let res = path2.find_neighbors((5, 5));
        let exp = vec![(0, 1), (2, 1), (1, 2), (1, 0)];
        let (res, exp): (HashSet<_>, HashSet<_>) = (res.into_iter().collect(), exp.into_iter().collect());
        assert_eq!(res, exp);
    }
}
