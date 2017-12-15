use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Reverse;

pub fn doit<B>(mut input: B)
where
    B: BufRead,
{
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let mut configuration: Vec<usize> = s.split_whitespace().map(|l| l.parse().unwrap()).collect();
    let length = configuration.len();
    let mut seen: HashMap<Vec<usize>, i64> = HashMap::new();
    let mut step = 0;
    //println!("{:?}", configuration);

    while !seen.contains_key(&configuration) {
        seen.insert(configuration.clone(), step);

        let (index, &blocks) = configuration
            .iter()
            .enumerate()
            .max_by_key(|&(i, v)| (v, Reverse(i)))
            .unwrap();
        configuration[index] = 0;
        for i in 0..blocks {
            configuration[(index + i + 1) % length] += 1;
        }

        step += 1;
        //println!("{:?}", configuration);
    }

    println!("Day 6, part 1: {}", step);
    println!("Day 6, part 2: {}", step - seen[&configuration])
}
