fn bracket_combinations(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }

    let mut catalan = vec![0; (num + 1) as usize];
    catalan[0] = 1;
    catalan[1] = 1;

    for i in 2..=num as usize {
        for j in 0..i {
            catalan[i] += catalan[j] * catalan[i - j - 1];
        }
    }

    catalan[num as usize]
}

fn main() {
    let num = 8;
    println!("{}", bracket_combinations(num));
}