use std::sync::Arc;
use crate::path::Path;

pub struct PathIter<'a> {
    first: Option<(u8, u8)>,
    tail: &'a Option<Arc<Path>>,
}

impl<'a> PathIter<'a> {
    pub fn new(coord: (u8, u8), arcpath: &'a Option<Arc<Path>>) -> Self {
        Self {
            first: Some(coord),
            tail: arcpath,
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        // save tail
        // point tail to prev
        // return saved tail

        // iterator will need to reference the next location, having its own arc would allow it to
        // reference that memory from heap whenever it wanted
        if self.first.is_some() {
            return self.first.take();
        }
        
        self.tail.as_ref().map(|path| {
            self.tail = path.get_prev();
            path.get_coord()
        })
    }
}

// impl IntoIterator for Path {
//     type Item = (u8, u8);
//     type IntoIter = PathIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         PathIter::new(&Some(Arc::new(self)))
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_path() {
        let path0 = Arc::new(Path::new((0,0), None));
        let path1 = Arc::new(Path::new((0,1), Some(Arc::clone(&path0))));
        let path2 = Arc::new(Path::new((1,1), Some(Arc::clone(&path1))));
        let path3 = Arc::new(Path::new((2,1), Some(Arc::clone(&path2))));
        // let path3 = Path::new((2,1), Some(Arc::clone(&path2)));

        for (x, y) in path3.as_ref() {
            println!("path: {:?}", (x, y));
        }

        dbg!(path3);
    }
}

