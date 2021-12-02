use crate::{Command,Submarine, utils};

pub fn get_result() -> i32 {
    let mut submarine= Submarine{horizontal:0,depth:0,aim:0};
    let instructions=utils::read_file("inputs/2.txt");
    for instruction in instructions.iter() {
        let split = instruction.split_whitespace().collect::<Vec<&str>>();
        let action = split.get(0).unwrap().to_string();
        let units = utils::to_int(split.get(1).unwrap().to_string());
        let command = Command{action:action,units:units};
        match command.action.as_ref() {
            "forward" => {
                submarine.horizontal = submarine.horizontal + command.units;
                submarine.depth = submarine.depth + (submarine.aim * command.units);
            },
            "up" => {
                submarine.aim = submarine.aim - command.units
            },
            "down" => submarine.aim = submarine.aim + command.units,
            _ => println!("missing action {}", command.action),
        }

    }
    println!("Final Location {}:{}",submarine.horizontal,submarine.depth);
    println!("Result {}",submarine.depth*submarine.horizontal);
    return submarine.depth*submarine.horizontal;
}