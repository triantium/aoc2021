use crate::utils;

#[derive(Copy, Clone, Debug)]
struct Fish {
    timer: u8,
}

impl Fish {
    fn breed(&mut self) -> Option<Fish> {
        if self.timer > 0 {
            self.timer -= 1;
            return Option::None;
        }
        self.timer = 6;
        let fish = Fish { timer: 8 };
        return Option::Some(fish);
    }
}

#[cfg(test)]
mod tests {
    use crate::day6;

    #[test]
    fn part1() {
        println!("--- DAY 6-1 ----");
        let result_test = day6::get_result_1("inputs/6_test.txt");
        assert_eq!(result_test, 5934);
        let result = day6::get_result_1("inputs/6.txt");
        println!("Hot Lanternfish :{} ", result);
        assert_eq!(result, 389726)
    }

    #[test]
    fn part2() {
        println!("--- DAY 6-2 ----");
        let _result_test = day6::get_result_2("inputs/6_test.txt");
        // assert_eq!(result_test, 12);
        // let result = get_result_2("inputs/6.txt");
        // println!("Hot Spots :{} ", result);
        // assert_eq!(result, 20666)
    }
}

fn get_result_1(file: &str) -> usize {
    let input = utils::read_file(file);
    let mut fishes: Vec<Fish> = read_lines(input);

    for _day in 0..80 {
        let mut new_fishes = Vec::new();
        for fish in fishes.iter_mut() {
            let new_fish = fish.breed();
            match new_fish {
                Some(f) => new_fishes.push(f),
                None => {}
            }
        }
        for new_fish in new_fishes.iter() {
            fishes.push(*new_fish);
        }
    }
    return fishes.len();
}

fn get_result_2(file: &str) -> usize {
    let input = utils::read_file(file);
    let mut fishes: Vec<Fish> = read_lines(input);

    return fishes.len();
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

fn read_lines(input: Vec<String>) -> Vec<Fish> {
    let fishes = input
        .get(0)
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .map(|s| s.parse::<u8>().unwrap())
        .map(|s| Fish { timer: s })
        .collect::<Vec<Fish>>();
    return fishes;
}
