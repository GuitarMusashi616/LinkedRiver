use std::thread::JoinHandle;
use std::{thread, sync::mpsc, sync::mpsc::Sender};
use crate::path::Path;
use crate::grid::Grid;
use std::sync::Arc;
use std::{collections::HashSet, thread::sleep, time::Duration, sync::mpsc::Receiver};


// pub struct ThreadPool {
//     handles: Vec<JoinHandle>,
// }

// impl ThreadPool {
//     pub fn new() -> Self {
//         Self {

//         }
//     }
// }

pub struct ThreadMessageDistributor<T> {
    txs: Vec<Sender<T>>,
    i: usize,
}

impl<T> ThreadMessageDistributor<T> {
    pub fn new() -> Self {
        Self {
            txs: Vec::new(),
            i: 0,
        }
    }

    pub fn add(&mut self, sender: Sender<T>) {
        self.txs.push(sender);
    }

    pub fn send(&mut self, item: T) {
        let tx = self.txs.get(self.i).expect("self.i does not exist in txs");
        tx.send(item).expect("item could not be sent by tx");
        
        if self.i < self.txs.len() {
            self.i = (self.i + 1) % self.txs.len();
        }
    }
}

pub fn threadpool_demo() {
    // create 7 worker threads that receive arc<path> and send 3 arc paths (or nothing) and the
    // score

    // create send/ receive thread that sends path, receives new paths, and distributes those
    // paths

    let mut tmd = ThreadMessageDistributor::new();
    let mut handles = Vec::new();
    let size = (10, 10);
    let (txtobase, rxbase) = mpsc::channel();
    let (txhighscore, rxhighscore) = mpsc::channel();

    for i in 0..6 {
        let (txthread, rxthread): (Sender<(Arc<Path>, i32)>, Receiver<(Arc<Path>, i32)>) = mpsc::channel();
        tmd.add(txthread);
        let txtobaseclone = txtobase.clone();
        let txhsclone = txhighscore.clone();

        // worker thread
        let handle = thread::spawn(move|| {
            while let Ok((path, score)) = rxthread.recv() {
                let (new_score, next_coords) = path.get_new_score(score, size);
                let coord_disp: Vec<(u8, u8)> = path.as_ref().into_iter().collect();
                // println!("thread {}: {:?} = {} => {:?}", i, coord_disp, new_score, next_coords);
                for coord in next_coords {
                    let new_path = Arc::new(Path::new(coord, Some(Arc::clone(&path))));
                    txtobaseclone.send((new_path, new_score)).expect("base already 'hung up'");
                }
                txhsclone.send((path, new_score)).expect("high score thread already 'hung up'");
            }
        });
        handles.push(handle);
    }

    // distributor thread
    let distributor = thread::spawn(move|| {
        while let Ok(msg) = rxbase.recv() {
            tmd.send(msg);
        }
    });
    handles.push(distributor);

    // high score thread
    let highscore = thread::spawn(move|| {
        let mut highest = 0;
        // let mut highest_path = None;
        while let Ok((path, score)) = rxhighscore.recv() {
            if score > highest {
                highest = score;
                // highest_path = Some(Arc::clone(&path));
                let coord_disp: Vec<(u8, u8)> = path.as_ref().into_iter().collect();
                println!("{} = {:?}", highest, coord_disp);
                let grid = Grid::new(size.0, size.1).set_coords(coord_disp);
                println!("score: {}\n{}", highest, grid);

            }
        }
    });
    handles.push(highscore);

    let path0 = Arc::new(Path::new((0, 0), None));
    txtobase.send((path0, 0)).expect("distributor thread already 'hung up'");

    for handle in handles {
        handle.join().expect("deadlock");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_threadpool() {
        threadpool_demo();
    }

    #[test]
    fn test_tmd() {
        let mut tmd = ThreadMessageDistributor::new();
        let (txtobase, rxbase) = mpsc::channel();

        for i in 0..4 {
            let (txthread, rxthread) = mpsc::channel();
            tmd.add(txthread);
            let txtobaseclone = txtobase.clone();

            thread::spawn(move|| {
                println!("thread {} spawned", i);
                loop {
                    let a = rxthread.recv();
                    match a {
                        Ok(msg) => {
                            println!("thread {}: {}", i, &msg);
                            txtobaseclone.send(a).unwrap();
                        },
                        _ => break,
                    };
                }
                println!("thread {} killed", i);
            });
        }

        let test_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let test_set: HashSet<i32> = test_vec.clone().into_iter().collect();

        for &num in &test_vec {
            tmd.send(num);
        }

        let mut output = HashSet::new();

        loop {
            let a = rxbase.recv_timeout(Duration::from_secs(1));
            match a {
                Ok(msg) => output.insert(msg.unwrap()),
                _ => break,
            };
        }
        println!("main thread killed");

        assert_eq!(output, test_set);
    }
}
