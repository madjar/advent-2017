const INPUT : i64 = 368078;

pub fn doit() {
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    let mut count = 1;
    let mut size = -0;
    let mut vx = 0;
    let mut vy = 1;

    loop {
        x += 1;
        y -= 1;
        size +=2;
        for _ in 0..4 {
            for _ in 0..size {
                x += vx;
                y += vy;
                count +=1;
                println!("{} {} {}", x, y, count);
                if count == INPUT {
                    println!("Solution: {}", x.abs() + y.abs());
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
