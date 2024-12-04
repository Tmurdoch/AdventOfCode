use std::fs;

fn determine_safe(line: &str) -> i32 {
    //TODO: write this, control flow makes no sense like it is below
    //but copy some of it into here
    let mut increasing = -1;
    let mut last_number = -1;
    for number in line.split_whitespace() {
        let p_number = number.parse::<i32>().unwrap();
        if last_number == -1 {
            last_number = p_number;
            continue;
        } else {
            if increasing == -1 {
                //first comparison
                let c = last_number - p_number;
                if c == 0 {
                    return 0;
                }
                if c >= 0 && c <= 3 {
                    increasing = 0; //decreasing
                } else if c <= 0 && c >= -3 {
                    increasing = 1;
                } else {
                    return 0;
                }
            } else if increasing == 1 {
                //increasing
                let c = last_number - p_number;
                if c >= 0 || c < -3 {
                    return 0;
                }
            }
            else if increasing == 0 {
                //decreasing
                let c = last_number - p_number;
                if c <= 0 || c > 3 {
                    return 0;
                }
            }
            last_number = p_number;
        }
    }
    return 1;
}

fn main() {
    //use std::io::{stdin, stdout, Write};
    let contents = fs::read_to_string("input.txt").expect("could not read file");
    let mut output = 0;
    for line in contents.lines() {
        println!("line: {:?}", line);
        let fn_ret = determine_safe(line);
        if fn_ret == 0 {
        println!("safe: {:?}", fn_ret);
        }
        output = output + fn_ret;
    }
    println!("total {:?}", output);
}
