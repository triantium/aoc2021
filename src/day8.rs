use std::collections::HashMap;
use crate::utils;

pub struct Wiring {
    coding: HashMap<String, u8>,
    encode: Vec<String>,
    decode: Vec<u8>,
}

impl Wiring {
    fn setup_map(&mut self) {
        let mut map = HashMap::with_capacity(10);
        let mut values = HashMap::with_capacity(10);

        //simple cases
        for code in self.coding.keys() {
            match code.len() {
                3 => {
                    values.insert(7, code.to_string());
                    map.insert(code.to_string(), 7)
                }
                2 => {
                    values.insert(1, code.to_string());
                    map.insert(code.to_string(), 1)
                }
                4 => {
                    values.insert(4, code.to_string());
                    map.insert(code.to_string(), 4)
                }
                7 => {
                    values.insert(8, code.to_string());
                    map.insert(code.to_string(), 8)
                }
                _ => map.insert(code.to_string(), 42),
            };
        }
        // get Count
        /* Verteilungshäufigkeit
                8/0
           ───────────
         │            │
         │            │
         │ 6/1        │ 8/2
         │            │
         │            │
         │     7/3    │
           ──────────
         │            │
         │            │
         │ 4/4        │ 9/5
         │            │
         │     7/6    │
          ────────────
        */

        let mut distribution = HashMap::new();
        for word in self.coding.keys() {
            for c in word.chars() {
                *distribution.entry(c).or_insert(0) += 1;
            }
        }

        let display = decode_display(distribution,values);

        let zero = [display[0],display[1],display[2],display[4],display[5],display[6]];
        let mut zero_s = String::new();
        zero.iter().for_each(|c|zero_s.push(*c));
        map.insert(sort_string(zero_s),0);

        let two = [display[0],display[2],display[3],display[4],display[6]];
        let mut two_s = String::new();
        two.iter().for_each(|c|two_s.push(*c));
        map.insert(sort_string(two_s),2);

        let three = [display[0],display[2],display[3],display[5],display[6]];
        let mut three_s = String::new();
        three.iter().for_each(|c|three_s.push(*c));
        map.insert(sort_string(three_s),3);

        let five = [display[0],display[1],display[3],display[5],display[6]];
        let mut five_s = String::new();
        five.iter().for_each(|c|five_s.push(*c));
        map.insert(sort_string(five_s),5);

        let six = [display[0],display[1],display[3],display[4],display[5],display[6]];
        let mut six_s = String::new();
        six.iter().for_each(|c|six_s.push(*c));
        map.insert(sort_string(six_s),6);

        let nine = [display[0],display[1],display[2],display[3],display[5],display[6]];
        let mut nine_s = String::new();
        nine.iter().for_each(|c|nine_s.push(*c));
        map.insert(sort_string(nine_s),9);


        self.coding = map;
    }

    fn decode(&mut self) {
        let mut decode = Vec::with_capacity(self.encode.len());
        for i in 0..self.encode.len() {
            let chiffrat = self.encode.get(i).unwrap().clone();
            let value = self.coding.get(&chiffrat);
            match value {
                None => {}
                Some(a) => { decode.push(*a) }
            }
        }
        self.decode = decode;
    }

    fn sum(&self) -> u64 {
        let mut sum = 0;
        let mut exp = 0;
        for i in (0..self.decode.len()).rev() {
            let base: u64 = 10;
            let deka = base.pow(exp as u32);
            let multi = self.decode[i] as u64;
            sum += deka * multi;
            exp += 1;
        }

        return sum;
    }
}

fn decode_display(distribution:HashMap<char,i32>,values:HashMap<i32,String>)->[char;7]{
    let mut display = ['0'; 7];

    for entry in distribution.iter() {
        match entry.1 {
            4 => {
                display[4] = *entry.0;
            }
            6 => {
                display[1] = *entry.0;
            }
            9 => {
                display[5] = *entry.0;
            }
            // a bissl komplexa
            7 => {
                let four = values.get(&4).unwrap();
                if !has_chars(four.to_string(), (*entry.0).to_string()) {
                    display[6] = *entry.0;
                } else {
                    display[3] = *entry.0;
                }
            }
            8 => {
                let one = values.get(&1).unwrap().to_string();
                if !has_chars(one, (*entry.0).to_string()) {
                    display[0] = *entry.0;
                } else {
                    display[2] = *entry.0;
                }
            }
            _ => {}
        }
    }
    return display;
}

fn has_chars(a: String, b: String) -> bool {
    let chars: Vec<char> = b.chars().collect();
    for c in chars {
        if !a.contains(c) {
            return false;
        }
    }
    return true;
}

fn _difference(a: String, b: String) -> Vec<char> {
    let mut map = HashMap::new();
    let chars: Vec<char> = b.chars().collect();
    for c in chars {
        if !a.contains(c) {
            map.insert(c, 0);
        }
    }
    let chars: Vec<char> = a.chars().collect();
    for c in chars {
        if !b.contains(c) {
            map.insert(c, 0);
        }
    }
    return map.keys().map(|k| *k).collect();
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day8;
    use crate::day8::{read_line, Wiring};

    #[test]
    fn test_wiring_decode() {
        let mut wiring = read_line("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string());
        wiring.setup_map();


        assert_eq!(*wiring.coding.get(&day8::sort_string_str("cagedb")).unwrap(), 0);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("dab")).unwrap(), 7);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("eafb")).unwrap(), 4);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("ab")).unwrap(), 1);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("acedgfb")).unwrap(), 8);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("cdfbe")).unwrap(), 5);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("gcdfa")).unwrap(), 2);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("fbcad")).unwrap(), 3);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("cefabd")).unwrap(), 9);
        assert_eq!(*wiring.coding.get(&day8::sort_string_str("cdfgeb")).unwrap(), 6);

        wiring.decode();

        assert_eq!(wiring.sum(), 5353)
    }


    #[test]
    fn part1() {
        println!("--- DAY 8-1 ----");
        let result_test = day8::get_result_1("inputs/8_test.txt");
        assert_eq!(result_test, 26);
        // assert_eq!(result_test, result_test_1 as u64);
        let result = day8::get_result_1("inputs/8.txt");
        assert_eq!(result, 355);
        println!("Result: {}",result);
    }

    #[test]
    fn part2() {
        println!("--- DAY 8-2 ----");
        let (result_test,testwire) = day8::get_result_2("inputs/8_test.txt");

        assert_eq!(testwire.get(0).unwrap().sum(),8394);
        assert_eq!(testwire.get(1).unwrap().sum(),9781);
        assert_eq!(testwire.get(2).unwrap().sum(),1197);
        assert_eq!(testwire.get(3).unwrap().sum(),9361);
        assert_eq!(testwire.get(4).unwrap().sum(),4873);
        assert_eq!(testwire.get(5).unwrap().sum(),8418);
        assert_eq!(testwire.get(6).unwrap().sum(),4548);
        assert_eq!(testwire.get(7).unwrap().sum(),1625);
        assert_eq!(testwire.get(8).unwrap().sum(),8717);
        assert_eq!(testwire.get(9).unwrap().sum(),4315);

        assert_eq!(result_test, 61229);
        let (result,_) = day8::get_result_2("inputs/8.txt");
        assert!(result > 954737);
        assert_eq!(result, 983030);
        println!("Result: {}",result);
    }

    #[test]
    fn sum_up() {
        let mut wire = Wiring { encode: Vec::new(), coding: HashMap::new(), decode: Vec::new() };

        wire.decode.push(1);
        assert_eq!(wire.sum(), 1);
        wire.decode.push(2);
        assert_eq!(wire.sum(), 12);
        wire.decode.push(3);
        assert_eq!(wire.sum(), 123);
        wire.decode.push(4);
        assert_eq!(wire.sum(), 1234)
    }
}

fn read_line(line: String) -> Wiring {
    let split = line.split("|")
        .map(|s| s.to_string())
        //.map(|s| Fish { timer: s })
        .collect::<Vec<String>>();
    let wirings = split.get(0).unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .map(|s| sort_string(s))
        .collect::<Vec<String>>();
    let output = split.get(1).unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .map(|s| sort_string(s))
        .collect::<Vec<String>>();
    let mut wiring = Wiring {
        coding: HashMap::with_capacity(10),
        decode: Vec::with_capacity(4),
        encode: output,
    };
    for x in wirings {
        wiring.coding.insert(x, 42);
    }
    return wiring;
}

fn read_file(file: &str) -> Vec<Wiring> {
    let input = utils::read_file(file);
    let mut results = Vec::with_capacity(input.len());
    for line in input.iter() {
        let wiring = read_line(line.to_string());
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
        .map(|w| w.decode)
        .flatten()
        .filter(|n| *n == 1 || *n == 4 || *n == 8 || *n == 7)
        .collect::<Vec<u8>>();

    return decodings.len();
}

pub fn get_result_2(file: &str) -> (u64, Vec<Wiring>) {
    let mut wirings = read_file(file);
    for wiring in wirings.iter_mut() {
        wiring.setup_map();
        wiring.decode();
    }


    let sum = wirings
        .iter()
        .map(|w| w.sum())
        .reduce(|a, b| (a + b))
        .unwrap();


    return (sum, wirings);
}

fn sort_string(wordy: String) -> String {
    let s_slice: &str = &wordy[..];
    return sort_string_str(s_slice);
}

fn sort_string_str(wordy: &str) -> String {
    let mut chars: Vec<char> = wordy.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));

    //println!("test{:?}", chars);
    let s: String = chars.into_iter().collect();
    return s;
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


