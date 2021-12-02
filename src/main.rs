
mod utils;

struct Submarine {
    horizontal: i32,
    depth: i32,
}

struct Command {
    action: String,
    duration: i32,
}


fn main() {
    let mut submarine= Submarine{horizontal:0,depth:0};
    let instructions=utils::read_file("inputs/2.txt");
    for instruction in instructions.iter() {
        let split = instruction.split_whitespace().collect::<Vec<&str>>();
        let action = split.get(0).unwrap().to_string();
        let duration = utils::to_int(split.get(1).unwrap().to_string());
        let command = Command{action:action,duration:duration};
        match command.action.as_ref() {
            "forward" => submarine.horizontal = submarine.horizontal + command.duration,
            "up" => submarine.depth = submarine.depth - command.duration,
            "down" => submarine.depth = submarine.depth + command.duration,
            _ => println!("missing action {}", command.action),
        }

    }
    println!("Final Location {}:{}",submarine.horizontal,submarine.depth);
    println!("Result {}",submarine.depth*submarine.horizontal);


}


