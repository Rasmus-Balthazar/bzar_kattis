use std::{collections::HashMap, i32};

// use std::io::{self, BufRead};
struct SmartestTupleEver {
    occurrences: i32,
    index: i32,
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let _ = stdin.read_line(&mut buf);
    let words: Vec<&str> = buf.trim().split_whitespace().collect();

    let mut word_map: HashMap<&str, SmartestTupleEver> = HashMap::new();
    // words.map(|w| &word_map.)
    let mut first_found = 0;
    let mut highest_occurence = 1;
    for w in words {
        // match word_map.contains_key(w) {
        //     true => &word_map.insert(w, word_map.get(w).unwrap().clone()),
        //     false => &word_map.insert(w, 1),
        // };
        if (word_map.contains_key(w)) {
            let tupe = word_map.get_mut(w).unwrap();
            tupe.occurrences += 1;
            if (highest_occurence < tupe.occurrences) {
                highest_occurence = tupe.occurrences;
            }
        } else {
            let tupe = SmartestTupleEver {
                occurrences: 1,
                index: first_found,
            };
            word_map.insert(w, tupe);
            first_found += 1;
        }
    }
    // find highest occuring results
    let mut it = word_map
        .iter()
        .filter(|w| w.1.occurrences == highest_occurence);
    let head = it.next().unwrap();
    let res = it.fold(head, |acc, ste| {
        if (ste.1.index < acc.1.index) {
            ste
        } else {
            acc
        }
    });
    println!("{}", &res.0);
}
