use std::fs;

fn main() {
    use std::io::{stdin, stdout, Write};
    let contents = fs::read_to_string("input1.txt").expect("could not read file");
    let mut input: Vec<i32> = Vec::new();
    for s in contents.split_whitespace() {
        input.push(s.trim().parse::<i32>().unwrap());
    }
    let _=stdout().flush();
    //if let Some('\n')=s.chars().next_back() {
    //    s.pop();
    //}
    println!("Got: {:?}", input);
}
