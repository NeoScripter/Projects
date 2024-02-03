use std::thread;

fn parallel_map(v: &mut Vec<u32>, f: fn(&mut u32)) {
    let len = v.len();
    let nthreads = std::cmp::max(len / 3, 1);
    let chunk_size = len / nthreads + (len % nthreads != 0) as usize;

    thread::scope(|s| {
        let v_slice = v.as_mut_slice();
        
        for chunk in v_slice.chunks_mut(chunk_size) {
            s.spawn(move || {
                chunk.iter_mut().for_each(f);
            });
        }
    });
}

fn main() {
    let mut vec = vec![2, 5, 6, 3, 2, 1, 8, 9, 40, 22, 30, 24, 19, 23, 16];
    let add_two = |n: &mut u32| {*n += 2};
    parallel_map(&mut vec, add_two);
    println!("{:?}", vec);
}