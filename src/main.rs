
mod utils;
mod day2;

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
}


