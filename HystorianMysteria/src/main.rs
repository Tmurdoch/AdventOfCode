use std::fs;

/*
 * counts how many times num appears in nums
 * multiplied count by number and returns
 */
fn counts(mut nums: Vec<i32>, num: i32) -> i32 {
    let mut count = 0;
    for i in nums.iter_mut() {
        if *i == num {
            count+=1;
        }
    }
    return count * num;
}

fn main() {
    use std::io::{stdout, Write};
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
    //for it in left.iter().zip(right.iter_mut()) {
    //    let (ai, bi) = it;
    //    let n = *ai - *bi;
    //    println!("diff: {:?}", n);
    //
    //    output += n.abs();
    //}
    for i in left.iter() {
        output += counts(right.clone(), *i);
    }

    println!("Got sum: {:?}", output);
}
