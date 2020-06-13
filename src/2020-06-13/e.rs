use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    for (i, l) in input.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let words: Vec<&str> = l.split(" ").collect();
        let (mut a, b): (i64, i64) = (
            words[0].parse().unwrap(),
            words[1].parse().unwrap(),
        );
        let mut ops = 0;
        let mut prev_ops = ops;
        loop {
            if a == b {
                break;
            }
            if a > b {
                for s in &[8, 4, 2] {
                    if a / s >= b && a % s == 0 {
                        a = a / s;
                        ops = ops + 1;
                        break;
                    }
                }
            }
            if a < b {
                for s in &[8, 4, 2] {
                    if a * s <= b {
                        a = a * s;
                        ops = ops + 1;
                        break;
                    }
                }
            }
            if ops == prev_ops {
                ops = -1;
                break;
            }
            prev_ops = ops;
        }
        println!("{}", ops);
    }
}
