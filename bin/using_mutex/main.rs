use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
fn main() {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let clone = Arc::clone(&map);
        let mut rng = rand::thread_rng();
        let iters = rng.gen_range(1..=5);
        let handle = thread::spawn(move || {
            let mut locked = clone.lock().unwrap();
            let id = thread::current().id();
            for _ in 0..iters {
                *locked.entry(id).or_insert(0) += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", map.lock().unwrap());
}