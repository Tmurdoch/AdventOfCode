use regex::Regex;
use std::fs;
use std::option;

fn parse_with_regex(line: &str) -> option::Option<Vec<&str>> {
    let pattern = r"mul\(\d+,\d+\)";
    let re = Regex::new(pattern).unwrap();
    let mut return_list: Vec<&str> = Vec::new();
    for matching_str in re.find_iter(line) {
        return_list.push(matching_str.as_str())
    }
    return Some(return_list);
}

//see https://doc.rust-lang.org/book/ch04-03-slices.html
fn extract_tuple_from_str(value: &str) -> Option<(i32, i32)> {
    let trimmed_value = value.trim();
    //let first_val: i32;
    //let second_val: i32;

    let parts: Vec<&str> = value.split(',').filter(|s| !s.trim().is_empty()).collect();
    if parts.len() == 2 {
        let first = parts[0].trim().parse::<i32>().ok();
        let second = parts[1].trim().parse::<i32>().ok();
        return Some((first, second));
    } else {
        return None;
    }
    //let bytes = trimmed_value.as_bytes();
    //for (_i, &item) in bytes.iter().enumerate() {
    //    if item.is_ascii_digit() {
    //        println!("found digit: {:?}", item);
    //    }
    //
    //    //get sliced with &value[..]
    //}

    //return (first_val, second_val);
}

fn test_extraction(value: &str, expected: (i32, i32)) {
    let val: (i32, i32) = extract_tuple_from_str(value);
    if val != expected {
        panic!("expected: {:?}, got {:?} ", expected, val);
    }
}

fn main() {
    let contents = fs::read_to_string("src/day3input.txt").expect("could not read file");

    test_extraction("0 0", (0, 0));

    let mut all_commands: Vec<&str> = Vec::new();
    for line in contents.lines() {
        let found_commands = parse_with_regex(line);

        for val in found_commands.expect("Found no values") {
            all_commands.push(&val);
        }
    }
}
