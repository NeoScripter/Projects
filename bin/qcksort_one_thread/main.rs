use std::cmp::Ordering;

fn qcksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return }

    let pivot_idx = partition(arr);
    let (lower, rest) = arr.split_at_mut(pivot_idx);
    let (pivot, greater) = rest.split_at_mut(1);

    qcksort(lower);
    qcksort(greater);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_idx = arr.len() / 2;
    arr.swap(pivot_idx, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() - 1 {
        match arr[j].cmp(&arr[arr.len()-1]) {
            Ordering::Less => {
                arr.swap(i, j);
                i += 1;
            }
            _ => (),
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    let mut v = [4, 6, 8, 14, 1, 3, 9];
    qcksort(&mut v);
    println!("{:?}", v);
}
