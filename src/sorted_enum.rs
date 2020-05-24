use std::collections::{BTreeMap, HashMap};
use std::ptr::hash;

// https://crates.io/crates/strum_macros
// You only need to give the first name a value, and thereafter the value goes up by one each time:
// To enable your values to be sorted, you must implement Ord
// Before you can implement Ord, you must first implement PartialOrd, Eq and PartialEq
#[derive(PartialEq, PartialOrd, Eq, Hash, Debug, Ord)]
enum Difficulty {
    Easy,
    Medium, // is 2
    Hard,   // is 3
}

pub fn print_sorted_enum() {
    println!(
        "hard is greater than easy: {}",
        Difficulty::Hard > Difficulty::Easy
    );
    println!(
        "easy is smaller than hard: {}",
        Difficulty::Easy < Difficulty::Hard
    );
    let mut hash_map = HashMap::new();
    hash_map.insert(Difficulty::Easy, 1);
    hash_map.insert(Difficulty::Hard, 1);
    println!("hash: {:?}", hash_map); // sometimes hard comes first

    let mut hash_map = BTreeMap::new();
    // hash_map.insert("", 1);
    hash_map.insert(Difficulty::Easy, 1);
    hash_map.insert(Difficulty::Hard, 1);
    println!("hash sorted: {:?}", hash_map); // always easy comes first
                                             // hard is greater than easy: true
                                             // easy is smaller than hard: true
                                             // hash: {Hard: 1, Easy: 1}
                                             // hash sorted: {Easy: 1, Hard: 1}
}
