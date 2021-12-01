use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("inputs/1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut prev = i32::MAX ;
        let mut count = 0 ;
        let mut vec = Vec::new();
        for line in lines {
            if let Ok(ip) = line {

                let z = (ip.to_string()).parse::<i32>().unwrap();
                vec.push(z);
                if z > prev {
                    count = count + 1;
                }
                prev  = z;
                //println!("{}", z);
            }
        }
        println!("count: {}",count);

        println!("count: {}",vec.len());
        let mut slidecount = 0 ;
        prev = i32::MAX ;
        let len=vec.len()-2;
        for i in 0..len {
            let measurement = vec.get(i).unwrap() + vec.get(i+1).unwrap() + vec.get(i+2).unwrap();
            if measurement > prev {
                slidecount = slidecount + 1;
            }
            prev=measurement;

        }
        println!("slidecount: {}",slidecount);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

