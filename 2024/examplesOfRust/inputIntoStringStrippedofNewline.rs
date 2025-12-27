fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Not a string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    println!("Got: {}", s);
}
