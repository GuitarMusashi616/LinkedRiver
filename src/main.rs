#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod path;
mod pathiter;
mod grid;
mod stackgrid;
mod threadpool;
mod itertest;
use path::Path;
use rand::Rng;
use grid::Grid;

use std::io::Read;
use std::sync::{Arc, mpsc};
use std::thread;

fn main() {
    let mut coords = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        coords.push((rng.gen_range(0..5), rng.gen_range(0..10)))
    }
    let grid = Grid::from(coords);
    // dbg!(grid);
    println!("{}", grid);
    let _ = std::io::stdin().read_line(&mut String::new());
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_recursive_paths() {
        let coord = (5, 4);
        let coord2 = (8, 8);

        let apath = Arc::new(Path::new(coord, None));
    }

    #[test]
    fn test_mult_threads() {
        let coord = (5, 4);
        let coord2 = (8, 8);

        let apath = Arc::new(Path::new(coord, None));
        let a2path = Arc::new(Path::new(coord2, Some(Arc::clone(&apath))));

        // let (tx, rx) = mpsc::channel();
        let tpathclone = Arc::clone(&apath);
        let tpathclone2 = Arc::clone(&a2path);

        let join = thread::spawn(move || {
            dbg!(tpathclone);
            thread::sleep(Duration::from_secs(1));
            thread::spawn(move || {
                dbg!(tpathclone2);
                thread::sleep(Duration::from_secs(1));
            });
        });

        let _ = join.join();
    }

    #[test]
    fn test_thread_pool() {
        let mut handles = Vec::new();
        for i in 0..8 {
            handles.push(thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                println!("hello from {}", i);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_grid_score() {
        let coords = vec![(0, 0), (0, 1), (0, 2), (1, 2)];
    }
}
