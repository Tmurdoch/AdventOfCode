use std::fs;

fn determine_safe(line: &str) -> i32 {
    let mut increasing = -1;
    let mut last_number = -1;
    let mut removed_one = false;
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
                    if removed_one {
                        return 0;
                    }
                    removed_one = true;
                    last_number = p_number;
                    continue;
                }
                if c >= 0 && c <= 3 {
                    increasing = 0; //decreasing
                } else if c <= 0 && c >= -3 {
                    increasing = 1;
                } else {
                    println!("got zero in first comparison again?");
                    if removed_one {
                        return 0;
                    } else {
                        //last_number = p_number;
                        removed_one = true;
                        continue;
                    }
                }
            } else if increasing == 1 {
                //increasing
                let c = last_number - p_number;
                if c >= 0 || c < -3 {
                    if removed_one {
                        return 0;
                    } else {
                        //last_number = p_number;
                        removed_one = true;
                        continue;
                    }
                }
            } else if increasing == 0 {
                //decreasing
                let c = last_number - p_number;
                if c <= 0 || c > 3 {
                    if removed_one {
                        return 0;
                    } else {
                        //last_number = p_number;
                        removed_one = true;
                        continue;
                    }
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

    test_case("13 15 16 19 21 24 24", 1);
    test_case("7 6 4 2 1", 1);
    test_case("1 2 7 8 9", 0);
    test_case("2 1 3 4 5", 1); // need to check value before removed one and recalculate whether we
                               // are increasing or decreasing
    test_case("9 7 6 2 1", 0);
    test_case("1 3 2 4 5", 1);
    test_case("8 6 4 4 1", 1);
    test_case("1 3 6 7 9", 1);
    test_case("55 55 55 53 51 48 46", 0);

    let contents = fs::read_to_string("input.txt").expect("could not read file");
    let mut output = 0;
    for line in contents.lines() {
        //println!("line: {:?}", line);
        let fn_ret = determine_safe(line);
        println!("line: {:?} safe: {:?}", line, fn_ret);
        output = output + fn_ret;
    }
    println!("total {:?}", output);
}
