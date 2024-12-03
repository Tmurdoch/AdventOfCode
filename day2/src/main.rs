use std::fs;

fn determine_safe() {
    //TODO: write this, control flow makes no sense like it is below
    //but copy some of it into here
}


fn main() {
    //use std::io::{stdin, stdout, Write};
    let contents = fs::read_to_string("input.txt").expect("could not read file");
    
    for line in contents.lines() {
        println!("line: {:?}", line);
        
        //flag for increasing/decreasing
        let mut increasing = -1;
        let mut last_number = -1;
        for number in line.split_whitespace() {
            println!("parsing: {:?}", number);
            let p_number = number.parse::<i32>().unwrap();
            if last_number == -1 {
                last_number = 
                continue;
            }
            else {
                if increasing == -1 {
                    //first comparison
                    let c = last_number - p_number;
                    if c > 0 && c <= 2 {
                        increasing = 0; //decreasing
                    }
                    else if c < 0 && c >= -2 {
                        increasing = 1;
                    }
                    else {
                        continue;
                    }
                }
            }
        }
    }
    
}
