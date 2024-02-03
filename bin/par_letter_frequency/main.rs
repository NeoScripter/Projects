const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

use std::{
    collections::HashMap,
    thread,
};

fn frequency(input: &[&str], nthreads: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();
        for lines in input {
            for c in lines.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()) {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };

    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(nthreads);
            for lines in input.chunks(input.len() / nthreads + 1) {
                handles.push(s.spawn(|| counter(lines)))
            }
            let mut map = handles.pop().unwrap().join().unwrap();
            for handle in handles {
                handle.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }
            map
        })
    }
}

fn main() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency(&v[..], 3);
    println!("{:?}, {:?}", freqs.get(&'ü'), freqs.get(&'t'));
}
