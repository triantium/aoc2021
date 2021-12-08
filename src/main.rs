mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;
mod day8;

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
    day8::get_result_1("inputs/8_test.txt");
    day8::get_result_2("inputs/8_test.txt");
}
