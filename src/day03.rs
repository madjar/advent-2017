use std::collections::HashMap;

const INPUT: i64 = 368078;

pub fn doit() {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut size = -0;
    let mut vx = 0;
    let mut vy = 1;
    let mut map = HashMap::new();

    let record = |x: i64, y: i64, value: i64, map: &mut HashMap<(i64, i64), i64>| {
        println!("{} {} {}", x, y, value);
        map.insert((x, y), value);
    };

    record(0, 0, 1, &mut map);

    loop {
        x += 1;
        y -= 1;
        size += 2;
        for _ in 0..4 {
            for _ in 0..size {
                x += vx;
                y += vy;
                // Cell value computation
                let count = neighbours(x, y)
                    .iter()
                    .map(|nei| map.get(nei).map(|x| *x).unwrap_or(0))
                    .sum();

                record(x, y, count, &mut map);
                if count >= INPUT {
                    println!("Solution: distance={}, value={}", x.abs() + y.abs(), count);
                    return;
                }
            }
            // Rotate
            let tmp = -vy;
            vy = vx;
            vx = tmp;
        }
    }
}

fn neighbours(x: i64, y: i64) -> Vec<(i64, i64)> {
    vec![
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
    ]
}
