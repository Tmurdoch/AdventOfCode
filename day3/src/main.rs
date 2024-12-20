use std::fs;
use regex::Regex;
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

fn extract_tuple_from_str(value: &str) -> (i32, i32) {

}

fn test_extraction(value: &str, expected: (i32, i32)) {
    let val: (i32, i32) = extract_tuple_from_str(value);
    if val != expected {
         panic!("expected: {:?}, got {:?} ", expected, val);
    }
}
 
fn main() {
    let contents = fs::read_to_string("src/day3input.txt").expect("could not read file");

    let mut all_commands: Vec<&str> = Vec::new(); 
    for line in contents.lines() {
        let found_commands = parse_with_regex(line);

        for val in found_commands.expect("Found no values") {
            all_commands.push(&val);
        }
    }

    for command in all_commands {
        let starting_num_index = command.find(|c: char| c >= '0' && c <= '9');
        
    }
}

