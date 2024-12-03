use std::fs;

fn main() {
    use std::io::{stdin, stdout, Write};
    let contents = fs::read_to_string("input1.txt").expect("could not read file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let mut count = 0;
    for s in contents.split_whitespace() {
        if count % 2 == 0{ 
            left.push(s.trim().parse::<i32>().unwrap());
        }
        else {
            right.push(s.trim().parse::<i32>().unwrap());
        }
        count+=1;
    }
    let _=stdout().flush();
    left.sort();
    right.sort();
    let mut output = 0;
    for i in &left {
        for j in &right {
            let n = i - j;
            output += n.abs();
        }
    }

    println!("Got sum: {:?}", output);
}
