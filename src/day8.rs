
use std::collections::HashMap;
use crate::utils;

struct Wiring {
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
        // 9
        let four= values.get(&4).unwrap();
        let nine = self.coding.keys()
            .map(|s|s.to_string())
            .filter(|key|key.len()==6) //5,6,9
            .filter(|s|has_chars(s.to_string(),four.to_string()))
            .collect::<Vec<String>>();
        assert_eq!(nine.len(), 1);
        let nine = nine.first().unwrap();
        values.insert(9, nine.to_string());
        map.insert(nine.to_string(), 9);
        // 3
        let seven= values.get(&7).unwrap();
        let three = self.coding.keys()
            .map(|s|s.to_string())
            .filter(|key|key.len()==5) //5,6,9
            .filter(|s|has_chars(s.to_string(),seven.to_string()))
            .collect::<Vec<String>>();
        assert_eq!(three.len(), 1);
        let three = three.first().unwrap();
        values.insert(3, three.to_string());
        map.insert(three.to_string(), 3);



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
        for i in (0..self.decode.len()).rev() {
            let deka = (10 * (i+1)) as u64;
            sum += deka*(self.decode[i] as u64);
        }

        return sum;
    }


}
fn has_chars(a: String,b:String) -> bool{
    let chars: Vec<char>= b.chars().collect();
    for c in chars {
        if !a.contains(c){
            return false
        }
    }
    return true;
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
        assert_eq!(result, 355);
        // assert_eq!(result,result_1 as u64);
    }

    #[test]
    fn part2() {
        println!("--- DAY 8-2 ----");
        let result_test = day8::get_result_2("inputs/8_test.txt");
        assert_eq!(result_test, 61229);
        let result = day8::get_result_2("inputs/8.txt");
        assert_eq!(result, 1);
    }
}

fn read_file(file: &str) -> Vec<Wiring> {
    let input = utils::read_file(file);
    let mut results = Vec::with_capacity(input.len());
    for line in input.iter() {
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
        .filter(|n| *n == 1 || *n == 4 ||*n == 8 ||*n == 7)
        .collect::<Vec<u8>>();

    return decodings.len();
}

pub fn get_result_2(file: &str) -> u64 {
    let mut wirings = read_file(file);
    for wiring in wirings.iter_mut() {
        wiring.setup_map();
        wiring.decode();
    }

    let sum = wirings
        .into_iter()
        .map(|w| w.sum())
        .reduce(|a, b| (a + b))
        .unwrap();


    return sum;
}

fn sort_string(wordy: String) -> String {
    let s_slice: &str = &wordy[..];
    let mut chars: Vec<char> = s_slice.chars().collect();
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


