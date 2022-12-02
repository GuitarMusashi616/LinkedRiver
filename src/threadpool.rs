use std::thread::JoinHandle;
use std::{thread, sync::mpsc, sync::mpsc::Sender};
use crate::path::Path;
use std::sync::Arc;

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



#[cfg(test)]
mod tests {
    use std::{collections::HashSet, thread::sleep, time::Duration, sync::mpsc::Receiver};

    use super::*;
    
    #[test]
    fn test_new_threadpool() {
        // create 7 worker threads that receive arc<path> and send 3 arc paths (or nothing) and the
        // score

        // create send/ receive thread that sends path, receives new paths, and distributes those
        // paths

        let mut tmd = ThreadMessageDistributor::new();
        let mut handles = Vec::new();

        for i in 0..7 {
            let (txthread, rxthread): (Sender<Arc<Path>>, Receiver<Arc<Path>>) = mpsc::channel();
            tmd.add(txthread);

            // worker thread
            let handle = thread::spawn(move|| {
                while let Ok(path) = rxthread.recv() {
                    println!("thread {}: {:?}", i, path.get_coord());
                }
            });
            handles.push(handle);
        }

        let (tx, rx) = mpsc::channel();

        // distrutor thread
        let handle = thread::spawn(move|| {
            while let Ok(msg) = rx.recv() {
                tmd.send(msg);
            }
        });
        handles.push(handle);

        let path0 = Arc::new(Path::new((0, 0), None));
        tx.send(path0).expect("distributor thread already 'hung up'");
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
