pub fn doit(input: String) {
    let mut sum = 0;
    let list: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let n = list.len();


    for i in 0..n {
        if list[i] == list[(i + 1) % n] {
            sum += list[i];
        }
    }
    println!("Day 1, part 2: {}", sum);
}
