use std::fs;
use std::collections;
use collections::BTreeMap;

/// Count the number of occurrences of each value in an iterator
pub fn get_counts<K: Ord, I: Iterator<K>>(mut iter: I) -> BTreeMap<K, i32> {
    let mut counter: BTreeMap<K, i32> = BTreeMap::new();
    for key in iter {
        match counter.get(&key) {
            Some(value) => { *value += 1; continue; }
            None => {}
        }
        counter.insert(key, 1);
    }
    counter
}

fn main() {
    use std::io::{stdin, stdout, Write};
    let contents = fs::read_to_string("input1.txt").expect("could not read file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let mut count = 0;
    for s in contents.split_whitespace() {
        if count % 2 == 0 {
            left.push(s.trim().parse::<i32>().unwrap());
        } else {
            right.push(s.trim().parse::<i32>().unwrap());
        }
        count += 1;
    }
    let _ = stdout().flush();
    left.sort();
    right.sort();
    let mut output = 0;
    //for i in &left {
    //    for j in &right {
    //        let n = i - j;
    //        println!("diff: {:?}", n);
    //        output += n.abs();
    //    }
    //}

    for it in left.iter().zip(right.iter_mut()) {
        let (ai, bi) = it;
        println!("diff: {:?}", n);

        output += n.abs();
    }

    println!("Got sum: {:?}", output);
}
