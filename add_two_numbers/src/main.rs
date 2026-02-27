// use std::io::{self, BufRead};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let _ = stdin.lock().read_line(&mut buf);
    let nums: Vec<i64> = buf
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    let a = nums[0];
    let b = nums[1];
    let c = a + b;
    println!("{c}");
}
