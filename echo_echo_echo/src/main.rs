use std::io::{BufRead, stdin};

fn main() {
    let mut buf = String::new();
    let _ = stdin().lock().read_line(&mut buf);
    let buf = buf.trim();
    let output = (0..3).map(|_| &buf[..]).collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}
