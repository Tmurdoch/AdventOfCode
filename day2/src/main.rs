use std::fs;

fn determine_safe(line: &str) -> i32 {
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

fn test_case(line: &str, expected: i32) {
    if determine_safe(line) != expected {
        panic!("expected {:?} to be {:?}", line, expected);
    }
}

fn main() {
    //use std::io::{stdin, stdout, Write};
    //TODO: pass some test cases into determine_safe so we can refactor and see where the problems
    //are

    test_case("7 6 4 2 1", 1);
    test_case("1 2 7 8 9", 0);  
    test_case("9 7 6 2 1", 0);
    test_case("1 3 2 4 5", 1);
    test_case("8 6 4 4 1", 1);
    test_case("1 3 6 7 9", 1);

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
