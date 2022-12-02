use std::sync::Arc;

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

    pub fn get_coord(&self) -> &(u8, u8) {
        &self.coord
    }

    // pub fn iter(&self) -> PathIter<&(u8, u8)> {

    // }

    pub fn neighbors(&self, size: &(u8, u8)) {
        
    }
}

#[cfg(test)]
mod tests {
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
}
