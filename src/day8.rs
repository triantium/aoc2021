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
        let four = values.get(&4).unwrap();
        let nine = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 6) //5,6,9
            .filter(|s| has_chars(s.to_string(), four.to_string()))
            .collect::<Vec<String>>();
        assert_eq!(nine.len(), 1);
        let nine = nine.first().unwrap();
        values.insert(9, nine.to_string());
        map.insert(nine.to_string(), 9);
        // 3
        let seven = values.get(&7).unwrap();
        let three = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 5) //5,6,9
            .filter(|s| has_chars(s.to_string(), seven.to_string()))
            .collect::<Vec<String>>();
        assert_eq!(three.len(), 1);
        let three = three.first().unwrap();
        values.insert(3, three.to_string());
        map.insert(three.to_string(), 3);

        let mut display: [char; 7] = ['0'; 7];
        // map To Single Lines
        let line0 = difference(
            values.get(&1).unwrap().to_string()
            , values.get(&7).unwrap().to_string());
        assert_eq!(line0.len(), 1);
        display[0] = line0.first().map(|k| *k).unwrap();
        let line4 = difference(
            values.get(&9).unwrap().to_string()
            , values.get(&3).unwrap().to_string());
        assert_eq!(line4.len(), 1);
        display[4] = line4.first().map(|k| *k).unwrap();

        //2
        let two = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 5) //5,6,9
            .filter(|key| key != three)
            .filter(|key| has_chars(key.to_string(), display[4].to_string()))
            .collect::<Vec<String>>();
        assert_eq!(two.len(), 1);

        let two = two.first().unwrap();
        values.insert(2, two.to_string());
        map.insert(two.to_string(), 2);
        //5
        let five = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 5) //5,6,9
            .filter(|key| key != three)
            .filter(|key| key != two)
            .collect::<Vec<String>>();
        assert_eq!(five.len(), 1);
        let five = five.first().unwrap();
        values.insert(5, five.to_string());
        map.insert(five.to_string(), 5);

        let tmp = difference(
            values.get(&5).unwrap().to_string()
            , values.get(&8).unwrap().to_string());
        assert_eq!(tmp.len(), 2);
        let line2 = difference(
            display[4].to_string()
            , tmp.iter().collect());
        assert_eq!(line2.len(), 1);
        display[2] = line2.first().map(|k| *k).unwrap();

        //line3

        //6 5 differenz zur 1, 0 hat 6 differenz
        let one = values.get(&1).unwrap();
        let zero = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 6)//0,6,9
            .filter(|key| key != nine)
            .filter(|key| difference(key.to_string(), one.to_string()).len() == 6)
            .collect::<Vec<String>>();
        assert_eq!(zero.len(), 1);
        let zero = zero.first().unwrap();
        values.insert(0, zero.to_string());
        map.insert(zero.to_string(), 0);

        //6
        let six = self.coding.keys()
            .map(|s| s.to_string())
            .filter(|key| key.len() == 6)//0,6,9
            .filter(|key| key != nine)
            .filter(|key| key != zero)
            .collect::<Vec<String>>();
        assert_eq!(six.len(), 1);
        let six = six.first().unwrap();
        values.insert(0, six.to_string());
        map.insert(six.to_string(), 6);


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

fn has_chars(a: String, b: String) -> bool {
    let chars: Vec<char> = b.chars().collect();
    for c in chars {
        if !a.contains(c) {
            return false;
        }
    }
    return true;
}

fn difference(a: String, b: String) -> Vec<char> {
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
    use crate::day8::Wiring;

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
        assert!(result > 954737);
        assert_eq!(result, 1);
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
        .filter(|n| *n == 1 || *n == 4 || *n == 8 || *n == 7)
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


