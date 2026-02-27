use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let res = buf.trim().parse::<f64>().unwrap() - 0.3;
    println!("{}", res);
}
