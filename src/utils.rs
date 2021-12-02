
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(filename: &str) -> Vec<String> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }

    }
    return vec
}

pub fn to_int (input: String) -> i32{
    return input.to_string().parse::<i32>().unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}