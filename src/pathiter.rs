pub struct PathIter {

}

impl PathIter {
    pub fn new() -> Self {
        Self {

        }
    }
}

// impl Iterator for PathIter {
//     type Item = &'a(u8, u8);

//     fn next(&mut self) -> Option<'aSelf::Item> {

//     }
// }

#[derive(Debug)]
struct Collection {
    
}

impl Collection {
    fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for Collection {
    type Item = i8;
    type IntoIter = CountIter;

    fn into_iter(self) -> Self::IntoIter {
        CountIter::new()
    }
}

impl IntoIterator for &Collection {
    type Item = i8;
    type IntoIter = CountIter;

    fn into_iter(self) -> Self::IntoIter {
        CountIter::new()
    }
}

struct CountIter {
    count: i8,
}

impl CountIter {
    fn new() -> Self {
        Self {
            count: 0,
        }
    }
}

impl Iterator for CountIter {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.count;
        if next > 20 {
            return None;
        }
        self.count += 1;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_iter() {
        let coll = Collection::new();
        for i in &coll {
            println!("{}", i);
        }
        dbg!(coll);
    }
}
