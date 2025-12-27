use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;



fn main() -> std::io::Result<()> {
    let file_path = Path::new("input.txt");
    let mut file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        println!("Line: {:?}", line);
    }
    Ok(())
}
