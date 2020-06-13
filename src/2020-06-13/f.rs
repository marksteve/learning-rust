use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let words: Vec<&str> = input.lines().next().unwrap().split(" ").collect();
    let (n, m, a): (f64, f64, f64) = (
        words[0].parse().unwrap(),
        words[1].parse().unwrap(),
        words[2].parse().unwrap(),
    );
    let output = (n / a).ceil() * (m / a).ceil();
    // n + a - 1 also works for i64
    println!("{}", output);
}
