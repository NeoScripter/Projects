use std::collections::HashMap;

fn sliding_window(arr: &[&str]) -> Option<String> {
    let (s, t) = (arr[0], arr[1]);
    let mut need = HashMap::new();
    let mut window = HashMap::new();

    for c in t.chars() { *need.entry(c).or_insert(0) += 1 }

    let (mut left, mut right, mut have) = (0, 0, 0);
    let (mut min_len, mut st) = (usize::MAX, 0);

    while right < s.len() {
        let c = s.chars().nth(right)?;
        if let Some(&count) = need.get(&c) {
            *window.entry(c).or_insert(0) += 1;
            if window[&c] == count {
                have += 1;
            }
        }
        right += 1;

        while have == need.len() {
            if right - left < min_len {
                min_len = right - left;
                st = left;
            }

            let c = s.chars().nth(left)?;
            if let Some(&count) = need.get(&c) {
                *window.get_mut(&c)? -= 1;
                if window[&c] < count {
                    have -= 1;
                }
            }
            left += 1;
        }
    }
    if min_len != usize::MAX { Some(s[st..st + min_len].to_string()) } else { None }
}
fn main() {
    let arr = ["aaffhkksemckelloe", "fhea"];
    match sliding_window(&arr) {
        Some(s) => println!("{}", s),
        None => println!("The string is empty"),
    }
}