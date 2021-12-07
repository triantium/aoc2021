use std::cmp::{max, min};
use std::convert::TryInto;
use crate::utils;


#[cfg(test)]
mod tests {
    use crate::day7;

    #[test]
    fn part1() {
        println!("--- DAY 7-1 ----");
        let result_test = day7::_get_result_1("inputs/7_test.txt");
        assert_eq!(result_test, 37);
        let result = day7::_get_result_1("inputs/7.txt");
        println!("Minimum Fuel: {}", result);
        assert_eq!(result, 356922);
    }

    #[test]
    fn part2() {
        println!("--- DAY 7-2 ----");
        let result_test = day7::_get_result_2("inputs/7_test.txt");
        assert_eq!(result_test, 168);
        let result = day7::_get_result_2("inputs/7.txt");
        println!("Minimum Fuel: {}", result);
        assert_eq!(result, 100347031);
    }
}

fn _get_result_1(file: &str) -> i32 {
    let input = utils::read_file(file);
    let crabfishes: Vec<i32> = read_lines(input);
    let max = crabfishes.iter()
        .reduce(|a, b| max(a, b)).unwrap();
    let mut fuel_consumptions = Vec::new();
    for _i in 0..*max {
        fuel_consumptions.push(0);
    }
    for i in 0..fuel_consumptions.len() {
        for crabfish in crabfishes.iter() {
            let fuel = i32::abs((i as i32) - crabfish);
            fuel_consumptions[i] += fuel;
        }
    }
    let min_fuel = fuel_consumptions.iter()
        .reduce(|a, b| min(a, b)).unwrap();

    return *min_fuel;
}

fn _get_result_2(file: &str) -> u64 {
    let input = utils::read_file(file);
    let crabfishes: Vec<i32> = read_lines(input);
    let max = crabfishes.iter()
        .reduce(|a, b| max(a, b)).unwrap();
    let mut fuel_consumptions = Vec::new();
    for _i in 0..*max {
        fuel_consumptions.push(0);
    }
    for i in 0..fuel_consumptions.len() {
        for crabfish in crabfishes.iter() {
            let fuel = i32::abs((i as i32) - crabfish);
            // actually use the

            fuel_consumptions[i] += gauß(fuel.try_into().unwrap());
        }
    }
    let min_fuel = fuel_consumptions.iter()
        .reduce(|a, b| min(a, b)).unwrap();

    return *min_fuel;
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

fn gauß (n: u64) -> u64 {
    (n * (n + 1))/2
}

fn read_lines(input: Vec<String>) -> Vec<i32> {
    let fishes = input
        .get(0)
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .map(|s| s.parse::<i32>().unwrap())
        //.map(|s| Fish { timer: s })
        .collect::<Vec<i32>>();
    return fishes;
}
