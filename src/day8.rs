use crate::utils;


#[cfg(test)]
mod tests {
    //use crate::day7;

    #[test]
    fn part1() {
        println!("--- DAY 7-1 ----");
        // let result_test = day7::_get_result_1("inputs/7_test.txt");
        // assert_eq!(result_test, 5934);
        // assert_eq!(result_test, result_test_1 as u64);
        // let result = day8::_get_result_1("inputs/8.txt");
        // assert_eq!(result, 389826);
        // assert_eq!(result,result_1 as u64);
    }

    #[test]
    fn part2() {
        println!("--- DAY 7-2 ----");
        // let result_test = day6::_get_result_2("inputs/6_test.txt", 256);
        // assert_eq!(result_test, 26984457539);
        // let result = day6::_get_result_2("inputs/6.txt", 256);
        // println!("Hot Lanternfish : {} ", result);
        // assert_eq!(result, 1743335992042);
    }
}

fn _get_result_1(file: &str) -> usize {
    let input = utils::read_file(file);
    let  _numbers: Vec<i32> = _read_lines(input);

    return 0;
}

fn _get_result_2(file: &str) -> u64 {
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

fn _read_lines(input: Vec<String>) -> Vec<i32> {
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
