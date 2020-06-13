use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    for (i, l) in input.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let line = String::from(l);
        let len = line.len();
        if len > 10 {
            println!("{}{}{}", &line[0..1], len - 2, &line[len - 1..len]);
        } else {
            println!("{}", l);
        }
    }
}
