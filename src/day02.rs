pub fn doit(input: String) {
    println!(
        "Day 2, part 1: {}",
        input.lines().map(|l| width(parse_line(l))).sum::<i64>()
    );
    println!(
        "Day 2, part 2: {}",
        input
            .lines()
            .map(|l| divisible_rest(parse_line(l)))
            .sum::<i64>()
    );
}

fn parse_line(l: &str) -> Vec<i64> {
    return l.split_whitespace().map(|e| e.parse().unwrap()).collect();
}

fn width(values: Vec<i64>) -> i64 {
    return values.iter().max().unwrap() - values.iter().min().unwrap();
}

fn divisible_rest(values: Vec<i64>) -> i64 {
    for x in values.iter() {
        for y in values.iter() {
            if x != y && x % y == 0 {
                return x / y;
            }
        }
    }
    return 0;
}
