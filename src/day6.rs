use crate::utils;

#[derive(Copy, Clone, Debug)]
struct Fish_School {
    timer: usize,
    count: u64,
}

#[derive(Copy, Clone, Debug)]
struct Fish {
    timer: usize,
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
        let result_test = day6::_get_result_2("inputs/6_test.txt", 80);
        assert_eq!(result_test, 5934);
        let result = day6::_get_result_2("inputs/6.txt", 80);
        println!("Hot Lanternfish : {} ", result);
        assert_eq!(result, 389726)
    }

    #[test]
    fn part2() {
        println!("--- DAY 6-2 ----");
        let result_test = day6::_get_result_2("inputs/6_test.txt", 256);
        assert_eq!(result_test, 26984457539);
        let result = day6::_get_result_2("inputs/6.txt", 256);
        println!("Hot Lanternfish : {} ", result);
        assert_eq!(result, 1743335992042)
    }
}

fn _get_result_1(file: &str, days: u16) -> usize {
    let input = utils::read_file(file);
    let mut fishes: Vec<Fish> = read_lines(input);

    for _day in 0..days {
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

fn _get_result_2(file: &str, days: u16) -> u64 {
    let input = utils::read_file(file);
    let fishes: Vec<Fish> = read_lines(input);
    let mut school: [Fish_School; 10] = [Fish_School {
        count: 0,
        timer: 10,
    }; 10];
    for i in 0..school.len() {
        school[i].timer = i;
    }
    for fish in fishes.iter() {
        school[fish.timer].count += 1;
    }
    for _day in 0..days {
        for timer in 0..school.len() {
            let count = school[timer].count;
            // Ringpuffer
            let index = (timer + school.len() - 1) % school.len();
            school[index].count = count;
        }
        // Breed and adjust
        school[8].count = school[9].count;
        school[6].count += school[9].count;
        school[9].count = 0;
    }

    let count: u64 = school.iter().map(|f| f.count).reduce(|a, b| a + b).unwrap();

    return count;
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
        .map(|s| s.parse::<usize>().unwrap())
        .map(|s| Fish { timer: s })
        .collect::<Vec<Fish>>();
    return fishes;
}
