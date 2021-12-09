use crate::utils;

//heigtmap: Vec<Vec<(u8,bool)>>

pub fn get_result_1(file: &str) -> u64 {
    let heigtmap = read_file(file);
    let mut low_points = Vec::new();
    for x in 0..heigtmap.len() {
        let row = heigtmap.get(x).unwrap();
        for y in 0..row.len() {
            match is_low_point(&heigtmap, x, y) {
                Some(v) => low_points.push(v),
                _ => {}
            };
        }
    }
    let threath_level = low_points.iter()
        .map(|x| x + 1)
        .map(|x| x as u64)
        .reduce(|a, b| a + b)
        .unwrap();
    return threath_level as u64;
}

pub fn get_result_2(file: &str) -> u64 {
    let mut heigtmap = read_file(file);
    let mut low_points = Vec::new();
    for x in 0..heigtmap.len() {
        let row = heigtmap.get(x).unwrap();
        for y in 0..row.len() {
            match is_low_point(&heigtmap, x, y) {
                Some(_) => low_points.push((x, y)),
                _ => {}
            };
        }
    }
    let heigtmap = heigtmap.as_mut();
    //let _basins =
    search_basins(heigtmap, low_points);

    return 0 as u64;
}

fn is_low_point(heightmap: &Vec<Vec<(u8, bool)>>, x: usize, y: usize) -> Option<u8> {
    let row = heightmap.get(x).unwrap();
    let point_val = *row.get(y).unwrap();
    let point_val = point_val.0;
    if x > 0 {
        let up = (x - 1, y);
        let up_val = *heightmap.get(up.0).unwrap().get(up.1).unwrap();
        if up_val.0 <= point_val {
            return Option::None;
        }
    }
    if x < heightmap.len() - 1 {
        let down = (x + 1, y);
        let down_val = *heightmap.get(down.0).unwrap().get(down.1).unwrap();
        if down_val.0 <= point_val {
            return Option::None;
        }
    }
    if y > 0 {
        let left = (x, y - 1);
        let left_val = *heightmap.get(left.0).unwrap().get(left.1).unwrap();
        if left_val.0 <= point_val {
            return Option::None;
        }
    }
    if y < row.len() - 1 {
        let right = (x, y + 1);
        let right_val = *heightmap.get(right.0).unwrap().get(right.1).unwrap();
        if right_val.0 <= point_val {
            return Option::None;
        }
    }
    return Option::Some(point_val);
}

fn search_basins(heightmap: &mut Vec<Vec<(u8, bool)>>, starting_points: Vec<(usize, usize)>) {
    for point in starting_points.iter() {
        let point = *point;
        let point = (point.0 as i64, point.1 as i64);
        track_point(point, heightmap);
    }
}

fn track_point(point: (i64, i64), heightmap: &mut Vec<Vec<(u8, bool)>>) -> Vec<u8> {
    if point.0 < 0 || point.1 < 0 {
        let vec: Vec<u8> = Vec::new();
        return vec;
    }
    if point.0 >= (heightmap.len() as i64) {
        let vec: Vec<u8> = Vec::new();
        return vec;
    }
    let row = heightmap.get(point.0 as usize).unwrap();
    if point.1 >= (row.len() as i64) {
        let vec: Vec<u8> = Vec::new();
        return vec;
    }
    let actual_point = *row.get(point.1 as usize).unwrap();
    if actual_point.1 || actual_point.0 == 9 {
        let vec: Vec<u8> = Vec::new();
        return vec;
    }
    //all else need to be marked and returned after checking the surroundings

    heightmap[point.0 as usize][point.1 as usize].1=true;


    let vec: Vec<u8> = Vec::new();
    return vec;
}


fn read_file(file: &str) -> Vec<Vec<(u8, bool)>> {
    let input = utils::read_file(file);
    let mut results = Vec::with_capacity(input.len());
    for line in input.iter() {
        let mut mappings = Vec::with_capacity(line.len());
        for char in line.chars() {
            let value = char.to_string().parse::<u8>().unwrap();
            mappings.push((value, false));
        }
        results.push(mappings);
    }
    return results;
}


#[cfg(test)]
mod tests {
    use crate::day9;


    #[test]
    fn part1() {
        println!("--- DAY 9-1 ----");
        let result_test = day9::get_result_1("inputs/9_test.txt");
        assert_eq!(result_test, 15);
        // assert_eq!(result_test, result_test_1 as u64);
        let result = day9::get_result_1("inputs/9.txt");

        println!("Risklevel: {}", result);
        assert_eq!(result, 516);
    }

    #[test]
    fn part2() {
        println!("--- DAY 9-2 ----");
        let result_test = day9::get_result_2("inputs/9_test.txt");
        assert_eq!(result_test, 1134);
        // assert_eq!(result_test, result_test_1 as u64);
        let result = day9::get_result_2("inputs/9.txt");

        println!("Basinsize: {}", result);
        assert_eq!(result, 516);
    }
}

