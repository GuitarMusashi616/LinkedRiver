use crate::*;

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
}

