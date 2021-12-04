
mod utils;
mod day2;
mod day3;
mod day4;

struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

struct Command {
    action: String,
    units: i32,
}


fn main() {
    day2::get_result();
    day3::get_result();
}


