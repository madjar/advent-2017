pub fn doit(input: String) {
    let mut instructions: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut offset: i64 = 0;
    let mut step = 0;

    while 0 <= offset && offset < instructions.len() as i64 {
        let jump = instructions[offset as usize];
        instructions[offset as usize] += if jump >= 3 { -1 } else { 1 };
        offset += jump;
        step += 1;
    }
    println!("Day 05, part 1: {}", step);
}
