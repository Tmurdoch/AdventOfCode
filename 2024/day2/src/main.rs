use std::fs;

fn determine_safe(line: &str) -> i32 {
    let mut increasing = -1;
    let mut last_number = -1;
    let mut removed_one = false;
    //TODO: use peekable, extract iterator out of the for loop declaration then finish failing if
    //statement
    let mut iter = line.split_whitespace().peekable();

    while let Some(number) = &mut iter.next() {
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
                        let next = &iter.peek();
                        match next {
                            Some(next_number) => {
                                if c > 0 && p_number > next_number.parse::<i32>().unwrap() {
                                    increasing = 0;
                                }
                                else if c < -3 {
                                    continue; //don't update last number
                                }
                                else {
                                    return 0;   
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
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
                        let next = &iter.peek();
                        match next {
                            Some(next_number) => {
                                if c < 0 && last_number > next_number.parse::<i32>().unwrap() {
                                    increasing = 1;
                                }
                                else if c > 3 {
                                    continue; //don't update last number
                                }
                                else {
                                    return 0; //too many wrong
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
            }
            last_number = p_number;
        }
    }
    if removed_one {
        println!("line: {:?} safe: {:?}", line, 1);
    }
    return 1;
}

fn test_case(line: &str, expected: i32) {
    let ans = determine_safe(line);
    if ans != expected {
        panic!("expected {:?} to be {:?}, got: {:?}", line, expected, ans);
    }
}

fn main() {
    //use std::io::{stdin, stdout, Write};
    test_case("5 4 3 4 5", 0);
    test_case("41 39 36 38 40", 0);
    test_case("5 7 4 3 2 1", 1);
    test_case("79 82 78 76 74", 1);
    test_case("74 76 78 82 79", 1);
    test_case("9 7 6 2 1", 0);
    test_case("13 15 16 19 21 24 24", 1);
    test_case("7 6 4 2 1", 1);
    test_case("1 2 7 8 9", 0);
    test_case("2 1 3 4 5", 1); // need to check value before removed one and recalculate whether we
                               // are increasing or decreasing
    test_case("5 4 3 1 2", 1);
    test_case("1 3 2 4 5", 1);
    test_case("5 4 2 3 1", 1);
    test_case("5 6 4", 1);
    test_case("4 6 5", 1);
    test_case("8 6 4 4 1", 1);
    test_case("1 3 6 7 9", 1);
    test_case("55 55 55 53 51 48 46", 0);
    test_case("5 6 4 3 2 1", 1);
    test_case("1 2 3 4 6 5", 1);
    test_case("1 20 3 4 3 5", 0);
    test_case("5 3 4 3 20 1", 0);

    let contents = fs::read_to_string("input.txt").expect("could not read file");
    let mut output = 0;
    for line in contents.lines() {
        //println!("line: {:?}", line);
        let fn_ret = determine_safe(line);
        output = output + fn_ret;
    }
    println!("total {:?}", output);
}
