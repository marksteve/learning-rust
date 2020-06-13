use std::io::{self, Read};

fn void_absorption(h: i64) -> i64 {
    h / 2 + 10
}

fn lightning_strike(h: i64) -> i64 {
    h - 10
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    for (i, l) in input.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let words: Vec<&str> = l.split(" ").collect();
        let (mut x, mut n, mut m): (i64, i64, i64) = (
            words[0].parse().unwrap(),
            words[1].parse().unwrap(),
            words[2].parse().unwrap(),
        );
        loop {
            if x <= 0 {
                println!("YES");
                break;
            }
            if n > 0 && x > void_absorption(x) {
                x = void_absorption(x);
                n = n - 1;
            } else if m > 0 {
                x = lightning_strike(x);
                m = m - 1;
            } else {
                println!("NO");
                break;
            }
        }
    }
}
