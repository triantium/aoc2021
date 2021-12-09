
use crate::utils;



pub fn get_result_1(file: &str) -> u64 {
    let mut _heigtmap = read_file(file);

    return 0;
}



fn read_file(file: &str) -> Vec<Vec<u8>> {
    let input = utils::read_file(file);
    let mut results = Vec::with_capacity(input.len());
    for line in input.iter() {
        let mut mappings = Vec::with_capacity(line.len());
        for char in line.chars() {
            let value=char.to_string().parse::<u8>().unwrap();
            mappings.push(value);
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

        println!("Risklevel: {}",result);
        assert_eq!(result, 355);
    }



}

