use std::borrow::Borrow;
use crate::{utils};

#[cfg(test)]
mod tests {
    use crate::{day3, utils};

    #[test]
    fn part1() {
        println!("---DAY 3-1----");
        let report=utils::read_file("inputs/3.txt");
        let gamma=day3::get_gamma(report.clone());
        let epsilon=day3::get_epsilon(report.clone());
        let power_consumption= gamma*epsilon;
        println!("Power_Consumption: {}",power_consumption);
        assert_eq!(power_consumption,3901196);
    }

    #[test]
    fn part2() {
        println!("---DAY 3-1----");
        let report=utils::read_file("inputs/3.txt");
        let o2=day3::get_oxygen_rating(report.clone());
        let co2 = day3::get_co2_rating(report.clone());
        let life_support= (o2)*(co2);
        println!("Lifesupport: {}",life_support);
        assert_eq!(life_support,4412188);
    }
}


pub fn get_result() {
    let report=utils::read_file("inputs/3.txt");

    let gamma=get_gamma(report.clone());
    let epsilon=get_epsilon(report.clone());
    let o2=get_oxygen_rating(report.clone());
    let co2 = get_co2_rating(report.clone());

    let power_consumption= gamma*epsilon;

    let life_support= (o2)*(co2);

    println!("Power_Consumption: {}",power_consumption);
    println!("Lifesupport: {}",life_support);
    return;
}

pub fn get_oxygen_rating (lines: Vec<String> ) -> isize {
    //let mut results: Vec<String>= Vec::new();
    let results = lines.clone();
    let last_one = filter_oxygen_rating(results,0);
    println!("Oxygen Rating: 0b{}",last_one);
    let rating = isize::from_str_radix(last_one.borrow(), 2).unwrap();
    println!("Oxygen Rating: {}",rating);
    return rating;
}

fn filter_oxygen_rating (lines: Vec<String>, index:usize ) -> String {
    let mut results = lines.clone();
    let results2 = lines.clone();
    if lines.len() == 1 {
        return results.pop().unwrap();
    }
    let bit = most_common_bit(results,index);
    let meh = results2.iter()
        .filter(|line|{
        let mut chars=line.chars();
        let chara = chars.nth(index).unwrap();
        return bit==chara;
    }).map(|f|f.clone()).collect();

    return filter_oxygen_rating(meh,index+1);

}

pub fn get_gamma(lines: Vec<String> ) -> isize{
    let results = lines.clone();
    let len = lines.get(0).unwrap().len();
    let mut gamma_rate = String::new();
    for i in 0..len {
        let mut vec = Vec::new();
        for tmp in results.iter(){
            vec.push(tmp.clone());
        }
        gamma_rate.push(most_common_bit(vec,i));
    }
    println!("Gamma: Ob{}",gamma_rate);
    let gamma = isize::from_str_radix(gamma_rate.borrow(), 2).unwrap();
    return gamma;
}

pub fn get_epsilon(lines: Vec<String> ) -> isize{
    let results = lines.clone();
    let len = lines.get(0).unwrap().len();
    let mut epsilon_rate = String::new();
    for i in 0..len {
        let mut vec = Vec::new();
        for tmp in results.iter(){
            vec.push(tmp.clone());
        }
        epsilon_rate.push(least_common_bit(vec,i));
    }
    println!("Epsilon: Ob{}",epsilon_rate);
    let epsilon = isize::from_str_radix(epsilon_rate.borrow(), 2).unwrap();
    return epsilon;
}

pub fn get_co2_rating (lines: Vec<String> ) -> isize {
    let results = lines.clone();
    let last_one = filter_co2_rating(results,0);
    println!("Co Rating: 0b{}",last_one);
    let rating = isize::from_str_radix(last_one.borrow(), 2).unwrap();
    println!("Co2 Rating: {}",rating);
    return rating;
}

fn filter_co2_rating (lines: Vec<String>, index:usize ) -> String {
    let mut results = lines.clone();
    let results2 = lines.clone();
    if lines.len() == 1 {
        return results.pop().unwrap();
    }
    let bit = least_common_bit(results,index);
    let meh = results2.iter()
        .filter(|line|{
            let mut chars=line.chars();
            let chara = chars.nth(index).unwrap();
            return bit==chara;
        }).map(|f|f.clone()).collect();

    return filter_co2_rating(meh,index+1);

}

fn most_common_bit(lines: Vec<String> , index: usize) -> char {
    let mut ones=0;
    let mut zeros = 0;

    for line in lines.iter() {

        let mut chars=line.chars();
        let chara = chars.nth(index).unwrap();
        match chara {
            '0'  => {
                zeros+=1;
            },
            '1'  => {
                ones+=1;
            },
            _ => panic!("WTF")
        }

    }
    if zeros > ones {
        return '0';
    }else{
        // if equal
        return '1';
    }
}

fn least_common_bit(lines: Vec<String> , index: usize) -> char {
    let most = most_common_bit(lines,index);
    match most {
        '0' => return '1',
        '1' => return '0',
        _ => panic!("WTV")
    }
}

