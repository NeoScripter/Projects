use std::thread;
use std::sync::{Condvar, Arc, Mutex};

const BUFFER_SIZE: usize = 5;

fn producer(id: i32, shared_buffer: Arc<(Mutex<Vec<i32>>, Condvar, Condvar)>) {
    let (lock, cvar_not_full, cvar_not_empty) = &*shared_buffer;
    let mut buffer = lock.lock().unwrap();
    for _ in 0..10 {
        while buffer.len() == BUFFER_SIZE {
            buffer = cvar_not_full.wait(buffer).unwrap();
        }

        println!("Producer {} produced an item", id);
        buffer.push(id);

        cvar_not_empty.notify_one();
    }
}

fn consumer(id: i32, shared_buffer: Arc<(Mutex<Vec<i32>>, Condvar, Condvar)>) {
    let (lock, cvar_not_full, cvar_not_empty) = &*shared_buffer;
    let mut buffer = lock.lock().unwrap();
    for _ in 0..10 {
        while buffer.is_empty() {
            buffer = cvar_not_empty.wait(buffer).unwrap();
        }

        let item = buffer.remove(0);
        println!("Consumer {} consumed item {}.", id, item);
        
        cvar_not_full.notify_one();
    }
}

fn main() {
    let shared_buffer = Arc::new((
        Mutex::new(Vec::new()),
        Condvar::new(),
        Condvar::new(),
    ));

    let mut handles = Vec::new();
    for i in 0..3 {
        let sb_clone = Arc::clone(&shared_buffer);
        handles.push(thread::spawn(move || producer(i, sb_clone)))
    }

    for i in 0..3 {
        let sb_clone = Arc::clone(&shared_buffer);
        handles.push(thread::spawn(move || consumer(i, sb_clone)))
    }

    for handle in handles {
        handle.join().unwrap();
    }
}