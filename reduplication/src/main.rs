use std::io;

fn main() {
    let mut word = String::new();
    let mut number = String::new();

    let _ = io::stdin().read_line(&mut word);
    let word = word.trim();
    let _ = io::stdin().read_line(&mut number);
    let num = number.trim().parse::<i64>().unwrap();
    let res = (0..num).map(|_| word).collect::<Vec<&str>>().join("");
    println!("{}", res);
}
