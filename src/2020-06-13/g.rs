use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut lines = input.lines();
    let l = lines.next().unwrap();
    let params: Vec<&str> = l.split(" ").collect();
    let (_, k): (i64, i64) = (params[0].parse().unwrap(), params[1].parse().unwrap());
    let l = lines.next().unwrap();
    let mut output = 0;
    let mut score = 0;
    for (i, x) in l.split(" ").map(|x| x.parse::<i64>().unwrap()).enumerate() {
        if x < 1 {
            continue;
        }
        if i + 1 <= k as usize {
            output = output + 1;
            if i + 1 == k as usize {
                score = x;
            }
        } else if x >= score {
            output = output + 1;
        }
    }
    println!("{}", output);
}