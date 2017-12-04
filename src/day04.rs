use std::collections::HashSet;

pub fn doit(input: String) {
    let res = input.lines().filter(valid_passphrase).count();
    println!("Day 4, part 2: {}", res)
}

fn valid_passphrase(l: &&str) -> bool {
    let mut visited = HashSet::new();
    for w in l.split_whitespace() {
        let mut sorted = w.as_bytes().to_owned();
        sorted.sort();
        if visited.contains(&sorted) {
            return false;
        } else {
            // Part 1:
            // visited.insert(w);
            // Part 2:
            visited.insert(sorted);
        }
    }
    return true;
}
