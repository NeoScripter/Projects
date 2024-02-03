use std::thread;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

fn quicksort<T: Ord + Send + Clone + 'static>(arr: Arc<Mutex<Vec<T>>>) -> Arc<Mutex<Vec<T>>> {
    let arr_len = {
        let locked_arr = arr.lock().unwrap();
        locked_arr.len()
    };
    
    if arr_len <= 1 { return arr }

    let pivot_idx = arr_len / 2;
    let pivot = {
        let locked_arr = arr.lock().unwrap();
        locked_arr[pivot_idx].clone()
    };

    let (less, equal, greater) = {
        let mut locked_arr = arr.lock().unwrap();
        locked_arr.drain(..).fold((Vec::new(), Vec::new(), Vec::new()), |(mut less, mut equal, mut greater), item| {
            match item.cmp(&pivot) {
                Ordering::Less => less.push(item),
                Ordering::Equal => equal.push(item),
                Ordering::Greater => greater.push(item),
            }
            (less, equal, greater)
        })
    };

    let lower_thread = thread::spawn(move || quicksort(Arc::new(Mutex::new(less))));
    let upper_thread = thread::spawn(move || quicksort(Arc::new(Mutex::new(greater))));

    let mut result = Vec::new();
    result.append(&mut lower_thread.join().unwrap().lock().unwrap());
    result.extend(equal);
    result.append(&mut upper_thread.join().unwrap().lock().unwrap());
    Arc::new(Mutex::new(result))
}

fn main() {
    let numbers = Arc::new(Mutex::new(vec![10, 5, 2, 3, 12, 7, 4, 3, 10, 10]));
    let sorted_numbers = quicksort(numbers);
    println!("{:?}", sorted_numbers.lock().unwrap());
}
