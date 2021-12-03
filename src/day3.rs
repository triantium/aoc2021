use std::borrow::Borrow;
use crate::{Command, Submarine, utils};

pub fn get_result() -> isize {
    let report=utils::read_file("inputs/3.txt");
    let len = report.get(0).unwrap().len();

    let gamma=get_gamma(report.clone());
    let epsilon=get_epsilon(report.clone());
    getPowerConsumption(report.clone());
    let o2=getOxygenRating(report.clone());
    let co2 = get_co2_rating(report.clone());

    let life_support= (o2*23)*(co2*10);

    println!("Lifesupport: {}",life_support);


    return life_support;
}

fn getOxygenRating (lines: Vec<String> ) -> isize {
    let len = lines.get(0).unwrap().len();
    let mut index = 0;
    //let mut results: Vec<String>= Vec::new();
    let mut results = lines.clone();
    let last_one = filter_oxygen_rating(results,0);
    println!("Oxygen Rating: 0b{}",last_one);
    let oxygenRating = isize::from_str_radix(last_one.borrow(), 2).unwrap();
    println!("Oxygen Rating: {}",oxygenRating);
    return oxygenRating;
}

fn filter_oxygen_rating (lines: Vec<String>, index:usize ) -> String {
    let mut results = lines.clone();
    let mut results2 = lines.clone();
    if lines.len() == 1 {
        return results.pop().unwrap();
    }
    let bit = mostCommonBit(results,index);
    let meh = results2.iter()
        .filter(|line|{
        let mut chars=line.chars();
        let chara = chars.nth(index).unwrap();
        return bit==chara;
    }).map(|f|f.clone()).collect();

    return filter_oxygen_rating(meh,index+1);

}

fn get_gamma(lines: Vec<String> ) -> isize{
    let mut results = lines.clone();
    let len = lines.get(0).unwrap().len();
    let mut gamma_rate = String::new();
    for i in 0..len {
        let mut vec = Vec::new();
        for tmp in results.iter(){
            vec.push(tmp.clone());
        }
        gamma_rate.push(mostCommonBit(vec,i));
    }
    println!("Gamma: Ob{}",gamma_rate);
    let gamma = isize::from_str_radix(gamma_rate.borrow(), 2).unwrap();
    return gamma;
}

fn get_epsilon(lines: Vec<String> ) -> isize{
    let mut results = lines.clone();
    let len = lines.get(0).unwrap().len();
    let mut epsilon_rate = String::new();
    for i in 0..len {
        let mut vec = Vec::new();
        for tmp in results.iter(){
            vec.push(tmp.clone());
        }
        epsilon_rate.push(leastCommon(vec,i));
    }
    println!("Epsilon: Ob{}",epsilon_rate);
    let epsilon = isize::from_str_radix(epsilon_rate.borrow(), 2).unwrap();
    return epsilon;
}

fn get_co2_rating (lines: Vec<String> ) -> isize {
    let len = lines.get(0).unwrap().len();
    let mut index = 0;
    //let mut results: Vec<String>= Vec::new();
    let mut results = lines.clone();
    let last_one = filter_co2_rating(results,0);
    println!("Co Rating: 0b{}",last_one);
    let rating = isize::from_str_radix(last_one.borrow(), 2).unwrap();
    println!("Co2 Rating: {}",rating);
    return rating;
}

fn filter_co2_rating (lines: Vec<String>, index:usize ) -> String {
    let mut results = lines.clone();
    let mut results2 = lines.clone();
    if lines.len() == 1 {
        return results.pop().unwrap();
    }
    let bit = leastCommon(results,index);
    let meh = results2.iter()
        .filter(|line|{
            let mut chars=line.chars();
            let chara = chars.nth(index).unwrap();
            return bit==chara;
        }).map(|f|f.clone()).collect();

    return filter_co2_rating(meh,index+1);

}

fn mostCommonBit(lines: Vec<String> , index: usize) -> char {
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

fn leastCommon(lines: Vec<String> , index: usize) -> char {
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
        return '1';
    }else{
        // if equal
        return '0';
    }
}

fn getPowerConsumption (lines: Vec<String>) -> isize{
    let len = lines.get(0).unwrap().len();
    let mut ones = vec![0;len];
    let mut zeros = vec![0;len];
    for line in lines.iter() {
        for i in 0..len {
            let mut chars=line.chars();
            let chara = chars.nth(i).unwrap();
            match chara {
                '0'  => {
                    let val= zeros.get(i).unwrap() + 1;
                    zeros[i]=val;
                },
                '1'  => {
                    let val= ones.get(i).unwrap() + 1;
                    ones[i]=val;
                },
                _ => panic!("WTF")
            }
        }
    }
    let mut gamma_rate = String::new();
    let  mut epsilon_rate = String::new();
    for i in 0..len {
        if zeros[i] > ones[i] {
            epsilon_rate.push('1');
            gamma_rate.push('0');
        } else if zeros[i] < ones [i]{
            epsilon_rate.push('0');
            gamma_rate.push('1');
        } else{
            panic!("Damn")
        }
    }
    println!("Epsilon: Ob{}",epsilon_rate);
    println!("Gamma: Ob{}",gamma_rate);
    let gamma = isize::from_str_radix(gamma_rate.borrow(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon_rate.borrow(), 2).unwrap();
    let power_consumption = gamma * epsilon;


    println!("Power consumption: {}",power_consumption);
    return power_consumption;
}