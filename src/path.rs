use std::{sync::Arc, collections::{HashSet, HashMap}};
use crate::pathiter::PathIter;

const ADJ: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug)]
pub struct Path {
    coord: (u8, u8),
    prev: Option<Arc<Self>>,
}

impl Path {
    pub fn new(coord: (u8, u8), prev: Option<Arc<Self>>) -> Self {
        Self {
            coord,
            prev,
        }
    }

    pub fn get_coord(&self) -> (u8, u8) {
        self.coord
    }

    pub fn get_prev(&self) -> &Option<Arc<Self>> {
        // get the arc out of the option, clone it, then wrap it back up in the option
        // if their isn't anything there then just make it a None
        &self.prev
    }


    pub fn find_neighbors(&self, size: (u8, u8)) -> HashSet<(u8, u8)> {
        let mut output = HashSet::new();
        for (dx, dy) in ADJ {
            let (x, y) = (self.coord.0 as i8 + dx, self.coord.1 as i8 + dy);
            if x < 0 || y < 0 || x >= size.0 as i8 || y >= size.1 as i8 {
                continue;
            }
            output.insert((x as u8, y as u8));
        }
        output
    }

    pub fn get_new_score(&self, prev_score: i32, size: (u8, u8)) -> (i32, HashSet<(u8, u8)>) {
        // calculates the new score based on the prev score before path0 was added on
        let neighbors = self.find_neighbors(size);
        let mut forest_tiles = neighbors.clone();

        let mut river_count = 0;
        
        for coord in self {
            if neighbors.contains(&coord) {
                river_count += 1;
                forest_tiles.remove(&coord);
            }
        }

        let num_adj = neighbors.len() as i32;
        let tree_count = num_adj - river_count;
        let score = prev_score + (-4*river_count) + (4*tree_count);

        (score, forest_tiles)
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = (u8, u8);
    type IntoIter = PathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathIter::new(self.get_coord(), self.get_prev())
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

    #[test]
    fn test_scoring() {
        let path0 = Arc::new(Path::new((0,0), None));
        let path1 = Arc::new(Path::new((0,1), Some(Arc::clone(&path0))));
        let path2 = Arc::new(Path::new((1,1), Some(Arc::clone(&path1))));
        let path3 = Arc::new(Path::new((2,1), Some(Arc::clone(&path2))));
        // this path0 should return a score of +8 = +8 total

        let (score, set) = path0.get_new_score(0, (5, 5));
        assert_eq!(score, 8);

        let (score, set) = path1.get_new_score(8, (5, 5));
        assert_eq!(score, 12);

        let (score, set) = path2.get_new_score(12, (5, 5));
        assert_eq!(score, 20);

        let (score, set) = path3.get_new_score(20, (5, 5));
        assert_eq!(score, 28);
    }
}
