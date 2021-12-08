use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::iter::Map;
use crate::utils;

struct Wiring {
    coding: HashMap<String,u8>,
    encode: Vec<String>,
    decode: Vec<u8>
}

impl Wiring {

    fn setup_map(&mut self) {
        let mut map = HashMap::with_capacity(10);

        //simple cases
        for code in self.coding.keys().borrow_mut() {
            match code.len() {
                3 => map.insert(code.to_string(),7),
                2 => map.insert(code.to_string(),1),
                4 => map.insert(code.to_string(),4),
                7 => map.insert(code.to_string(),8),
                _ => map.insert(code.to_string(),42),
            };
        }
        self.coding=map;

    }

    fn decode(&mut self) {
        let mut decode=Vec::with_capacity(self.encode.len());
        for chifrat in self.encode.iter(){
            let value = self.coding.get(&chifrat.to_string());
            match value {
                None => {},
                Some(a) => {decode.push(*a)},
            }

        }
        self.decode=decode;
    }

}


#[cfg(test)]
mod tests {
    use crate::day8;

    #[test]
    fn part1() {
        println!("--- DAY 8-1 ----");
        let result_test = day8::get_result_1("inputs/8_test.txt");
        assert_eq!(result_test, 26);
        // assert_eq!(result_test, result_test_1 as u64);
        let result = day8::get_result_1("inputs/8.txt");
        // assert_eq!(result, 389826);
        // assert_eq!(result,result_1 as u64);
    }

    #[test]
    fn part2() {
        println!("--- DAY 8-2 ----");
        // let result_test = day6::_get_result_2("inputs/6_test.txt", 256);
        // assert_eq!(result_test, 26984457539);
        // let result = day6::_get_result_2("inputs/6.txt", 256);
        // println!("Hot Lanternfish : {} ", result);
        // assert_eq!(result, 1743335992042);
    }
}

fn read_file(file:&str) -> Vec<Wiring> {
    let input = utils::read_file(file);
    let mut results= Vec::with_capacity(input.len());
    for line in input.iter(){
        let split = line.split("|")
            .map(|s| s.to_string())
            //.map(|s| Fish { timer: s })
            .collect::<Vec<String>>();
        let wirings = split.get(0).unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let output = split.get(1).unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut wiring = Wiring{
            coding: HashMap::with_capacity(10),
            decode: Vec::with_capacity(4),
            encode: output,
        };
        for x in wirings {
            wiring.coding.insert(x.to_string(),42);
        }
        results.push(wiring);
    }
    return results;
}

pub fn get_result_1(file: &str) -> usize {
    let mut wirings = read_file(file);
    for wiring in wirings.iter_mut() {
        wiring.setup_map();
        wiring.decode();
    }

    let decodings = wirings
        .into_iter()
        .map(|w|w.decode)
        .flatten()
        .filter(|n|*n!=42)
        .collect::<Vec<u8>>();

    return decodings.len();
}

pub fn get_result_2(file: &str) -> u64 {
    let _input = utils::read_file(file);

    return 0;
}

fn _print_map(map: Vec<Vec<i32>>) {
    for row in map.iter() {
        println!();
        for cell in row.iter() {
            print!("{}", cell);
        }
    }
    println!();
}


